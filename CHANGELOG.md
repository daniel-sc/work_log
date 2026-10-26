# Changelog

## 1.2.0-beta.2

- Sign and notarize the Apple Silicon macOS release with Apple Developer ID.
- The Mac download is now a `.dmg` with an attached notarization ticket. Open it,
  copy `work_log` to a permanent folder, then run it from Terminal. Accessibility
  and Screen Recording permissions are still required; see the README.
- Linux and Windows behavior is unchanged from beta.1.

## 1.2.0-beta.1

### Changes

- Log the initial window immediately, then record window changes or mouse movement.
  Mouse activity no longer depends on absolute cursor coordinates.
- Preserve commas, quotes, and embedded newlines with standard CSV quoting.
- Reject invalid logging intervals, preserve filenames containing `=`, and allow
  autostart installation before the output file exists. The default interval stays
  at two seconds.
- Quote Linux and Windows autostart paths and arguments correctly. On macOS,
  reject values that would produce a broken LaunchAgent file.
- Refresh outdated GNOME extension metadata and incomplete installations for
  GNOME 42–50, with recovery guidance for upgrades across GNOME 45.
- Update dependencies and check formatting, Clippy, and locked builds in CI.

### Upgrading

- CSV still has five columns, but the two legacy coordinate columns are now empty:
  `timestamp,app_name,window_title,,`. Consumers must support CSV quoting, including
  multiline fields. Mouse activity still produces timestamped records.
- Rerun `--autostart-install` with your desired options to refresh an existing
  startup entry. Keep the executable at that location afterwards.
- After a GNOME extension installation or update, log out/in and enable
  `x-win@miniben90.org`. See the README for permissions and recovery instructions.

### Beta platforms

Release binaries target Linux x86_64, Windows x86_64, and macOS Apple Silicon.
The macOS binary replaces the Intel build provided in 1.1.2; this beta does not
include an Intel macOS binary.
