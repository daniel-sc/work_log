# work_log

Log the active window (title + app name) to CSV on startup, then on window changes
or mouse movement since the previous sample. Cursor positions are not stored;
the two legacy coordinate columns stay empty. CSV quoting preserves commas,
quotes, and embedded newlines.

This can be used to analyze how you spend your time on your computer.

Supported platforms:
- Windows
- Linux (X11 and Wayland)
- MacOS

## Prerequisites

### Linux

For mouse movement tracking, the user running `work_log` must be a member of the `input` group:

```bash
sudo usermod -aG input $USER
```

Log out and back in for the group change to take effect.

> **Note:** On some distributions, the group may be called `plugdev` instead. If `input` doesn't work, try adding yourself to `plugdev` as well.

#### GNOME Wayland

Active window detection requires the **x-win GNOME Shell extension**. On startup,
`work_log` installs it if missing and refreshes outdated compatibility metadata or
incomplete installations in `~/.local/share/gnome-shell/extensions/x-win@miniben90.org`.
The bundled extension declares support for GNOME 42–50.

After installation or an update, log out and back in, then enable it:

```bash
gnome-extensions enable x-win@miniben90.org
```

Restart `work_log` afterwards. Updating the files does not reload the extension in
the running GNOME session. At subsequent logins, `work_log` waits up to 18 seconds
for the extension to become available.

### macOS

The application needs the following permissions in **System Settings > Privacy & Security**:

- **Accessibility** for mouse movement tracking.
- **Screen & System Audio Recording** (called **Screen Recording** on older macOS versions) for window titles. Without this permission, app names may be logged while window titles remain empty.

Enable these permissions for the terminal or application running `work_log`, or for `work_log` itself if running independently (for example, via autostart). Fully quit and restart the affected application or `work_log` after granting permissions.

On older macOS versions, these settings are under **System Preferences > Security & Privacy > Privacy**.

## Install

Download the latest release for your platform from the [releases page](https://github.com/daniel-sc/work_log/releases).


## Usage

```text
Usage: work_log [OPTION]...
Log the active window on window changes or mouse movement to a CSV file.

Options:
  --frequency=SECONDS  set the logging interval (positive; default: 2.0)
  --file=FILE          set the file to write to (default: ~/work_log.csv)
  --stdout             print to stdout instead of a file
  --autostart-install  install the program to run at startup.
    (options are preserved, do not move the executable afterwards)
  --autostart-remove   remove the program from startup
  --help               display this help and exit
```

To refresh an existing autostart entry after an upgrade, rerun `--autostart-install`
with your desired options.

## Example

The resulting csv file will look like this:

```csv
2025-02-10T14:11:27+01:00,Editor,my-project,,
2025-02-10T14:11:29+01:00,Editor,my-project,,
2025-02-10T14:11:31+01:00,Browser,"Report, Q4",,
```

## View
The resulting CSV can be analysed by [work_log_view](https://daniel-sc.github.io/work_log_view/) - just drop the CSV in your browser window and get your result:

![image](https://github.com/user-attachments/assets/b99ffd02-4189-4236-b89a-8497bf7e8959)
