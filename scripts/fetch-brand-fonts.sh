#!/usr/bin/env bash
# Fetch Cenote brand fonts (Cormorant Garamond + Sora) from the official
# Google Fonts repo. Both are licensed under the SIL Open Font License 1.1,
# so redistribution / upload is fine.
#
# Usage:  ./scripts/fetch-brand-fonts.sh
# Output: assets/fonts/CormorantGaramond/*.ttf
#         assets/fonts/Sora/*.ttf
#         assets/fonts/<family>/OFL.txt (license)

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/assets/fonts"
mkdir -p "$DEST/CormorantGaramond" "$DEST/Sora"

BASE="https://raw.githubusercontent.com/google/fonts/main/ofl"

# ---- Cormorant Garamond (static TTFs, 5 weights x roman + italic) ----
CG_FILES=(
  "CormorantGaramond-Light.ttf"
  "CormorantGaramond-LightItalic.ttf"
  "CormorantGaramond-Regular.ttf"
  "CormorantGaramond-Italic.ttf"
  "CormorantGaramond-Medium.ttf"
  "CormorantGaramond-MediumItalic.ttf"
  "CormorantGaramond-SemiBold.ttf"
  "CormorantGaramond-SemiBoldItalic.ttf"
  "CormorantGaramond-Bold.ttf"
  "CormorantGaramond-BoldItalic.ttf"
  "OFL.txt"
)
echo "→ Cormorant Garamond"
for f in "${CG_FILES[@]}"; do
  echo "   $f"
  curl -fsSL -o "$DEST/CormorantGaramond/$f" "$BASE/cormorantgaramond/$f"
done

# ---- Sora (variable font + static TTFs for Thin → ExtraBold) ----
# Sora ships as a variable font plus a /static/ folder of pre-rendered weights.
SORA_STATIC=(
  "Sora-Thin.ttf"
  "Sora-ExtraLight.ttf"
  "Sora-Light.ttf"
  "Sora-Regular.ttf"
  "Sora-Medium.ttf"
  "Sora-SemiBold.ttf"
  "Sora-Bold.ttf"
  "Sora-ExtraBold.ttf"
)
echo "→ Sora (variable)"
curl -fsSL -o "$DEST/Sora/Sora[wght].ttf"    "$BASE/sora/Sora%5Bwght%5D.ttf"
curl -fsSL -o "$DEST/Sora/OFL.txt"           "$BASE/sora/OFL.txt"

echo "→ Sora (static weights)"
for f in "${SORA_STATIC[@]}"; do
  echo "   $f"
  curl -fsSL -o "$DEST/Sora/$f" "$BASE/sora/static/$f"
done

echo
echo "Done. Files under: $DEST"
ls -la "$DEST/CormorantGaramond" "$DEST/Sora"
