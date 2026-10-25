use std::fs;
use std::io;
use std::path::Path;

const EXTENSION_UUID: &str = "x-win@miniben90.org";
// Keep in sync with x-win's bundled metadata when upgrading the dependency.
// Its version-name stayed at 1.8.0 when GNOME 47–50 support was added.
const SHELL_VERSIONS: &[&str] = &["42", "43", "44", "45", "46", "47", "48", "49", "50"];

/// Inspect files on disk: GNOME's D-Bus view can be stale until the next login.
fn needs_update(extension_dir: &Path) -> io::Result<bool> {
    let metadata = match fs::read(extension_dir.join("metadata.json")) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(true),
        Err(err) => return Err(err),
    };
    let Ok(metadata) = serde_json::from_slice::<serde_json::Value>(&metadata) else {
        return Ok(true);
    };
    if metadata["uuid"].as_str() != Some(EXTENSION_UUID) {
        return Ok(true);
    }
    let Some(versions) = metadata["shell-version"].as_array() else {
        return Ok(true);
    };
    if !SHELL_VERSIONS
        .iter()
        .all(|version| versions.iter().any(|value| value.as_str() == Some(version)))
    {
        return Ok(true);
    }
    match fs::metadata(extension_dir.join("extension.js")) {
        Ok(script) => Ok(!script.is_file() || script.len() == 0),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(true),
        Err(err) => Err(err),
    }
}

/// Install or refresh the bundled extension, then allow time for login startup.
#[cfg(target_os = "linux")]
pub fn ensure_gnome_wayland_extension() {
    use std::{env, path::PathBuf, thread, time::Duration};

    // Match x-win's Wayland detection, including sessions without XDG_SESSION_TYPE.
    let is_wayland = env::var_os("WAYLAND_DISPLAY").is_some_and(|s| !s.is_empty());
    let is_gnome = env::var("XDG_CURRENT_DESKTOP")
        .map(|s| {
            s.split(':')
                .any(|desktop| desktop.eq_ignore_ascii_case("gnome"))
        })
        .unwrap_or(false);
    if !is_wayland || !is_gnome {
        return;
    }

    let Some(home) = env::var_os("HOME").filter(|s| !s.is_empty()) else {
        eprintln!("Cannot set up the GNOME shell extension: HOME is not set.");
        return;
    };
    // x-win's installer uses HOME rather than XDG_DATA_HOME.
    let extension_dir = PathBuf::from(home)
        .join(".local/share/gnome-shell/extensions")
        .join(EXTENSION_UUID);
    match needs_update(&extension_dir) {
        Ok(true) => {
            eprintln!("GNOME Wayland: installing/updating the x-win shell extension...");
            // x-win 5.8 still panics on some installer I/O errors.
            let result = std::panic::catch_unwind(x_win::install_extension)
                .map_err(|_| "x-win's extension installer panicked".to_string())
                .and_then(|result| result.map_err(|err| err.to_string()));
            match result {
                Ok(true) => {
                    eprintln!("Extension installed to {}", extension_dir.display());
                    eprintln!("Log out and back in to load the installed extension, then run:");
                    eprintln!("  gnome-extensions enable {EXTENSION_UUID}");
                    eprintln!("Restart work_log afterwards. The running shell may still use the old extension until you log out.");
                }
                Ok(false) => {
                    eprintln!("GNOME shell extension installation was not performed by x-win.")
                }
                Err(err) => eprintln!("Failed to install GNOME shell extension: {err}"),
            }
            return;
        }
        Err(err) => {
            eprintln!(
                "Cannot inspect GNOME shell extension at {}: {err}",
                extension_dir.display()
            );
            return;
        }
        Ok(false) => {}
    }

    // The extension may still be loading when work_log autostarts at login.
    for attempt in 0..10 {
        if attempt > 0 {
            thread::sleep(Duration::from_secs(2));
        }
        if x_win::get_active_window().is_ok() {
            return;
        }
    }
    eprintln!(
        "GNOME Wayland: the extension is installed but active window detection is not working."
    );
    eprintln!("Make sure it is enabled: gnome-extensions enable {EXTENSION_UUID}");
    eprintln!("If it was just installed or updated, log out and back in, then restart work_log.");
    eprintln!(
        "If detection stopped after a GNOME upgrade, move {} to a backup location outside the extensions directory, then restart work_log to reinstall it. Log out and back in afterwards.",
        extension_dir.display()
    );
    eprintln!("The program will continue but window info may be empty.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_extension(dir: &Path, versions: &[&str]) {
        let metadata = serde_json::json!({
            "uuid": EXTENSION_UUID,
            "version-name": "1.8.0",
            "shell-version": versions,
        });
        fs::write(dir.join("metadata.json"), metadata.to_string()).unwrap();
        fs::write(dir.join("extension.js"), "// extension script").unwrap();
    }

    #[test]
    fn upgrades_old_metadata_even_with_the_same_version_name() {
        let dir = tempfile::tempdir().unwrap();
        write_extension(dir.path(), &["42", "43", "44", "45", "46"]);
        assert!(needs_update(dir.path()).unwrap());

        write_extension(dir.path(), SHELL_VERSIONS);
        assert!(!needs_update(dir.path()).unwrap());
    }

    #[test]
    fn accepts_newer_metadata_without_reinstalling() {
        let dir = tempfile::tempdir().unwrap();
        let mut versions = SHELL_VERSIONS.to_vec();
        versions.push("51");
        write_extension(dir.path(), &versions);
        assert!(!needs_update(dir.path()).unwrap());
    }

    #[test]
    fn repairs_missing_or_incomplete_installations() {
        let dir = tempfile::tempdir().unwrap();
        assert!(needs_update(&dir.path().join("missing")).unwrap());
        assert!(needs_update(dir.path()).unwrap());

        write_extension(dir.path(), SHELL_VERSIONS);
        fs::remove_file(dir.path().join("extension.js")).unwrap();
        assert!(needs_update(dir.path()).unwrap());
        fs::write(dir.path().join("extension.js"), "").unwrap();
        assert!(needs_update(dir.path()).unwrap());
    }

    #[test]
    fn repairs_invalid_metadata() {
        let dir = tempfile::tempdir().unwrap();
        for metadata in ["invalid json", "{}", "null", r#"{"shell-version": "50"}"#] {
            fs::write(dir.path().join("metadata.json"), metadata).unwrap();
            assert!(needs_update(dir.path()).unwrap());
        }
    }

    #[test]
    fn propagates_read_errors_instead_of_treating_them_as_missing() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("metadata.json")).unwrap();
        assert!(needs_update(dir.path()).is_err());
    }
}
