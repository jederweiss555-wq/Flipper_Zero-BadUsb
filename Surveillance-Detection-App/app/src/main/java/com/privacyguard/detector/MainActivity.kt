package com.privacyguard.detector

import android.graphics.Color
import android.graphics.Typeface
import android.os.Bundle
import android.view.Gravity
import android.view.View
import android.widget.Button
import android.widget.LinearLayout
import android.widget.ProgressBar
import android.widget.ScrollView
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import com.google.android.material.card.MaterialCardView
import java.util.concurrent.Executors

/**
 * Single-screen UI: a "Scan starten" button runs every check on a background
 * thread and renders the grouped findings as colour-coded cards.
 *
 * Everything happens on-device. The app requests no permissions and makes no
 * network calls — it only *reads* public system state.
 */
class MainActivity : AppCompatActivity() {

    private val executor = Executors.newSingleThreadExecutor()

    private lateinit var root: LinearLayout
    private lateinit var resultsContainer: LinearLayout
    private lateinit var scanButton: Button
    private lateinit var progress: ProgressBar
    private lateinit var summary: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        buildUi()
    }

    private fun buildUi() {
        val pad = dp(16)

        root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(pad, pad, pad, pad)
        }

        val title = TextView(this).apply {
            text = getString(R.string.app_name)
            textSize = 22f
            setTypeface(typeface, Typeface.BOLD)
        }
        val subtitle = TextView(this).apply {
            text = getString(R.string.subtitle)
            textSize = 13f
            setPadding(0, dp(4), 0, dp(12))
            setTextColor(Color.GRAY)
        }

        scanButton = Button(this).apply {
            text = getString(R.string.start_scan)
            setOnClickListener { startScan() }
        }

        progress = ProgressBar(this).apply {
            visibility = View.GONE
        }

        summary = TextView(this).apply {
            textSize = 15f
            setTypeface(typeface, Typeface.BOLD)
            setPadding(0, dp(12), 0, dp(8))
            visibility = View.GONE
        }

        resultsContainer = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
        }

        val scroll = ScrollView(this).apply {
            addView(resultsContainer)
        }

        root.addView(title)
        root.addView(subtitle)
        root.addView(scanButton)
        root.addView(progress)
        root.addView(summary)
        root.addView(scroll, LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.MATCH_PARENT
        ))

        setContentView(root)
    }

    private fun startScan() {
        scanButton.isEnabled = false
        progress.visibility = View.VISIBLE
        summary.visibility = View.GONE
        resultsContainer.removeAllViews()

        executor.execute {
            val sections = SecurityScanner(applicationContext).runAll()
            runOnUiThread { renderResults(sections) }
        }
    }

    private fun renderResults(sections: List<CheckSection>) {
        progress.visibility = View.GONE
        scanButton.isEnabled = true

        var danger = 0; var warn = 0
        sections.forEach { s ->
            s.findings.forEach {
                when (it.severity) {
                    Severity.DANGER -> danger++
                    Severity.WARN -> warn++
                    else -> {}
                }
            }
        }

        summary.visibility = View.VISIBLE
        summary.text = when {
            danger > 0 -> getString(R.string.summary_danger, danger, warn)
            warn > 0 -> getString(R.string.summary_warn, warn)
            else -> getString(R.string.summary_ok)
        }
        summary.setTextColor(
            when {
                danger > 0 -> colorFor(Severity.DANGER)
                warn > 0 -> colorFor(Severity.WARN)
                else -> colorFor(Severity.OK)
            }
        )

        sections.forEach { section ->
            val header = TextView(this).apply {
                text = section.name
                textSize = 16f
                setTypeface(typeface, Typeface.BOLD)
                setPadding(0, dp(16), 0, dp(6))
            }
            resultsContainer.addView(header)
            section.findings.forEach { resultsContainer.addView(findingCard(it)) }
        }
    }

    private fun findingCard(f: Finding): View {
        val card = MaterialCardView(this).apply {
            radius = dp(10).toFloat()
            cardElevation = dp(2).toFloat()
            val lp = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            )
            lp.setMargins(0, dp(4), 0, dp(4))
            layoutParams = lp
        }
        val inner = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(dp(14), dp(12), dp(14), dp(12))
        }
        val badge = TextView(this).apply {
            text = severityLabel(f.severity)
            setTextColor(Color.WHITE)
            textSize = 11f
            setTypeface(typeface, Typeface.BOLD)
            setPadding(dp(8), dp(2), dp(8), dp(2))
            gravity = Gravity.CENTER
            setBackgroundColor(colorFor(f.severity))
        }
        val titleView = TextView(this).apply {
            text = f.title
            textSize = 15f
            setTypeface(typeface, Typeface.BOLD)
            setPadding(0, dp(6), 0, dp(2))
        }
        val detailView = TextView(this).apply {
            text = f.detail
            textSize = 13f
            setTextColor(Color.DKGRAY)
        }
        inner.addView(badge, LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.WRAP_CONTENT,
            LinearLayout.LayoutParams.WRAP_CONTENT
        ))
        inner.addView(titleView)
        inner.addView(detailView)
        card.addView(inner)
        return card
    }

    private fun severityLabel(s: Severity): String = when (s) {
        Severity.OK -> "OK"
        Severity.INFO -> "INFO"
        Severity.WARN -> "ACHTUNG"
        Severity.DANGER -> "GEFAHR"
    }

    private fun colorFor(s: Severity): Int = ContextCompat.getColor(
        this,
        when (s) {
            Severity.OK -> R.color.sev_ok
            Severity.INFO -> R.color.sev_info
            Severity.WARN -> R.color.sev_warn
            Severity.DANGER -> R.color.sev_danger
        }
    )

    private fun dp(v: Int): Int = (v * resources.displayMetrics.density).toInt()

    override fun onDestroy() {
        super.onDestroy()
        executor.shutdownNow()
    }
}
