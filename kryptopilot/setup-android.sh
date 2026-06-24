#!/usr/bin/env bash
set -e

echo "=== KryptoPilot Android Setup ==="
echo ""

# 1. Check Node/pnpm
if ! command -v pnpm &>/dev/null; then
  echo "ERROR: pnpm not found. Install: npm install -g pnpm"
  exit 1
fi

# 2. Install frontend deps
echo "[1/6] Installing frontend dependencies..."
pnpm install

# 3. Check ANDROID_HOME
if [ -z "$ANDROID_HOME" ]; then
  echo ""
  echo "ERROR: ANDROID_HOME is not set."
  echo "  Install Android Studio: https://developer.android.com/studio"
  echo "  Then add to ~/.bashrc:"
  echo "    export ANDROID_HOME=\$HOME/Android/Sdk"
  echo "    export PATH=\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/platform-tools"
  echo "    export NDK_HOME=\$ANDROID_HOME/ndk/\$(ls \$ANDROID_HOME/ndk | head -1)"
  exit 1
fi
echo "[2/6] ANDROID_HOME: $ANDROID_HOME ✓"

# 4. Check NDK_HOME
if [ -z "$NDK_HOME" ]; then
  NDK_VERSION=$(ls "$ANDROID_HOME/ndk" 2>/dev/null | head -1)
  if [ -n "$NDK_VERSION" ]; then
    export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
    echo "[3/6] NDK_HOME auto-detected: $NDK_HOME"
  else
    echo "ERROR: NDK not found. Install in Android Studio → SDK Manager → SDK Tools → NDK (Side by side)"
    exit 1
  fi
else
  echo "[3/6] NDK_HOME: $NDK_HOME ✓"
fi

# 5. Add Rust Android targets
echo "[4/6] Adding Rust Android targets..."
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android

# 6. tauri android init
echo "[5/6] Running tauri android init..."
pnpm tauri android init

# 7. Patch AndroidManifest.xml with required permissions
MANIFEST="src-tauri/gen/android/app/src/main/AndroidManifest.xml"
if [ -f "$MANIFEST" ]; then
  echo "[6/6] Patching AndroidManifest.xml with permissions..."

  # Add permissions after <manifest ...> line if not already present
  PERMS='    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
    <uses-permission android:name="android.permission.WAKE_LOCK" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE" />'

  if ! grep -q "POST_NOTIFICATIONS" "$MANIFEST"; then
    # Insert after first <manifest line
    sed -i "/<manifest /a\\$PERMS" "$MANIFEST"
    echo "  Permissions added ✓"
  else
    echo "  Permissions already present ✓"
  fi
else
  echo "[6/6] AndroidManifest.xml not found - run this script again after 'tauri android init'"
fi

echo ""
echo "=== Setup complete! ==="
echo ""
echo "Next steps:"
echo "  Dev (USB device or emulator):  pnpm tauri android dev"
echo "  Release build:                 pnpm tauri android build"
echo "  Check rust only:               cargo check --target aarch64-linux-android"
echo ""
