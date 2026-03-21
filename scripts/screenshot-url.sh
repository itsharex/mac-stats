#!/bin/bash
URL="${1:-http://localhost:8787/alert-test.html}"
TS=$(date -u +%Y%m%d_%H%M%S)
DOMAIN=$(echo "$URL" | sed 's|https\?://||' | sed 's|/.*||' | tr '.:' '_')
OUT="$HOME/.mac-stats/screenshots/${TS}_${DOMAIN}.png"
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --headless=new --disable-gpu --no-sandbox \
  --screenshot="$OUT" --window-size=1800,1200 "$URL" 2>&1
echo "Screenshot saved: $OUT"
