#!/usr/bin/env bash
set -euo pipefail

: "${MACOS_CERTIFICATE_P12_BASE64:?Missing signing certificate}"
: "${MACOS_CERTIFICATE_PASSWORD:?Missing certificate password}"
: "${APPLE_ID:?Missing Apple Account email}"
: "${APPLE_TEAM_ID:?Missing Apple Team ID}"
: "${APPLE_APP_SPECIFIC_PASSWORD:?Missing notarization password}"

signing_dir=$(mktemp -d "$RUNNER_TEMP/work-log-signing.XXXXXX")
keychain="$signing_dir/signing.keychain-db"
cleanup() {
    security delete-keychain "$keychain" >/dev/null 2>&1 || true
    rm -rf "$signing_dir"
}
trap cleanup EXIT

keychain_password=$(openssl rand -hex 32)
echo "::add-mask::$keychain_password"
printf '%s' "$MACOS_CERTIFICATE_P12_BASE64" | base64 --decode > "$signing_dir/certificate.p12"
security create-keychain -p "$keychain_password" "$keychain"
security set-keychain-settings -lut 3600 "$keychain"
security unlock-keychain -p "$keychain_password" "$keychain"
security import "$signing_dir/certificate.p12" -k "$keychain" \
    -P "$MACOS_CERTIFICATE_PASSWORD" -T /usr/bin/codesign
security set-key-partition-list -S apple-tool:,apple: -k "$keychain_password" "$keychain" >/dev/null
security list-keychains -d user -s "$keychain"

mkdir "$signing_dir/payload"
binary="$signing_dir/payload/work_log"
cp target/aarch64-apple-darwin/release/work_log "$binary"
codesign --force --sign "Developer ID Application:" --keychain "$keychain" \
    --identifier org.daniel-sc.work_log --options runtime --timestamp "$binary"
codesign --verify --strict --verbose=2 "$binary"
"$binary" --help

# A disk image can carry the notarization ticket; a standalone executable cannot.
disk_image=work_log-aarch64-apple-darwin.dmg
hdiutil create -volname work_log -srcfolder "$signing_dir/payload" -format UDZO "$disk_image"
codesign --sign "Developer ID Application:" --keychain "$keychain" --timestamp "$disk_image"

notary_credentials=(--apple-id "$APPLE_ID" --team-id "$APPLE_TEAM_ID" --password "$APPLE_APP_SPECIFIC_PASSWORD")
if ! xcrun notarytool submit "$disk_image" "${notary_credentials[@]}" \
    --wait --timeout 30m --output-format json > "$signing_dir/notarization.json"; then
    cat "$signing_dir/notarization.json"
    exit 1
fi
cat "$signing_dir/notarization.json"
submission_id=$(jq -r '.id' "$signing_dir/notarization.json")
xcrun notarytool log "$submission_id" "${notary_credentials[@]}"
jq -e '.status == "Accepted"' "$signing_dir/notarization.json" > /dev/null

xcrun stapler staple "$disk_image"
xcrun stapler validate "$disk_image"
spctl --assess --type open --context context:primary-signature --verbose=2 "$disk_image"
