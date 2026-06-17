package com.privacyguard.detector

/**
 * Severity levels for a single check result.
 *
 * OK     – nothing suspicious found for this check.
 * INFO   – worth knowing, but not necessarily a problem.
 * WARN   – something that *can* be abused for surveillance; review it.
 * DANGER – a strong indicator of monitoring/compromise; act on it.
 */
enum class Severity { OK, INFO, WARN, DANGER }

/**
 * A single result line shown to the user.
 *
 * @param title   Short headline ("Aktive Bedienungshilfen").
 * @param detail  Human-readable explanation of what was found and why it matters.
 * @param severity How alarming the finding is.
 */
data class Finding(
    val title: String,
    val detail: String,
    val severity: Severity
)

/**
 * A group of findings produced by one checker, e.g. "Netzwerk".
 */
data class CheckSection(
    val name: String,
    val findings: List<Finding>
)
