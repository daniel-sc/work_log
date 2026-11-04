# Changelog

## 1.2.0

### macOS: signed and notarized Apple Silicon downloads

Mac releases now come as a **Developer ID signed and Apple-notarized `.dmg`**,
with the notarization ticket attached and a Gatekeeper check before upload.
This addresses the unverified-developer warning encountered with earlier downloads.
([Signing and notarization](https://github.com/daniel-sc/work_log/commit/975fef3d6d4d8ebcb747c1d7997fc41675bf2db9),
[release validation](https://github.com/daniel-sc/work_log/commit/a32e41d0891ea584f989c5742779e11ad3bc0add))

**Compatibility change:** the Mac download now targets **Apple Silicon (M-series)**.
It replaces the Intel Mac binary supplied in 1.1.2; this release does **not** include
an Intel Mac binary. Linux and Windows downloads remain x86_64.
([Architecture change](https://github.com/daniel-sc/work_log/commit/ef09714d7e716b4d331283ef5bc01a44763b0c06))

Open the `.dmg`, copy `work_log` to a permanent folder, then eject the mounted
`work_log` disk in Finder. Run the copied executable from Terminal. Accessibility
and Screen Recording permissions are still required; see the
[installation and permissions guide](https://github.com/daniel-sc/work_log/blob/v1.2.0/README.md).

### Logging and compatibility fixes

- Log the initial window and subsequent window changes or mouse movement. Mouse
  activity no longer depends on absolute cursor coordinates; the two legacy CSV
  coordinate fields stay empty. ([Change](https://github.com/daniel-sc/work_log/commit/3dd0ac07ccca86592806f0b7d5604d4af1d61113))
- Preserve commas, quotes, and embedded newlines with standard CSV quoting.
  The CSV remains headerless with five fields. Readers must support quoted and
  multiline fields; the companion viewer's parser has been fixed too.
  ([Logger fix](https://github.com/daniel-sc/work_log/commit/472acc9ba29465cfbb4030c962803842ec6d97ae),
  [viewer issue #6](https://github.com/daniel-sc/work_log_view/issues/6))
- Reject invalid logging intervals, preserve filenames containing `=`, and allow
  autostart setup before the output file exists. The default interval remains two
  seconds. ([Fix](https://github.com/daniel-sc/work_log/commit/2faede2dd924678e6102c6f25db4ce96ea3f3138))
- Quote Linux and Windows autostart paths and arguments correctly. On macOS,
  reject paths/options that would produce invalid LaunchAgent XML.
  ([Quoting fix](https://github.com/daniel-sc/work_log/commit/37466477dd6ad77a2baebcf120066c8ad2e81fb1),
  [macOS validation](https://github.com/daniel-sc/work_log/commit/7d2695d92440db589f5d8392811f63d9394850b6))
- Update dependencies and refresh outdated or incomplete GNOME extensions for
  GNOME 42–50, with recovery instructions for upgrades across GNOME 45.
  ([Extension update](https://github.com/daniel-sc/work_log/commit/9b4b89ff3d410b3d38bfe443b4b6b572b0e5abfc),
  [recovery guidance](https://github.com/daniel-sc/work_log/commit/776bd1f4d754bb3203e7fcb392caee3ba5f97cff))

### Upgrading

- Rerun `--autostart-install` from the permanent executable with your desired
  options to refresh an existing startup entry.
- After a GNOME extension update, log out/in and enable `x-win@miniben90.org`.
  See the README for recovery instructions.
- Windows autostart can still open a console window;
  [background startup #1](https://github.com/daniel-sc/work_log/issues/1) remains open.

[All changes since 1.1.2](https://github.com/daniel-sc/work_log/compare/v1.1.2...v1.2.0)

## 1.2.0-beta.3

- Ship the Developer ID signed and notarized Apple Silicon Mac download as a `.dmg`.
  Open it, copy `work_log` to a permanent folder, and run it from Terminal.
- Remove a duplicate ticket-validation request that failed on GitHub's Mac runner
  after successful notarization and stapling. Gatekeeper assessment still gates upload.
- Accessibility and Screen Recording permissions are still required; see the README.
  Linux and Windows behavior is unchanged from beta.1.

## 1.2.0-beta.2

The Mac upload failed during ticket validation; use beta.3 instead.

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
