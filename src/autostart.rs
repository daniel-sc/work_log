// auto-launch 0.6 joins executable/argument strings without quoting on Linux and Windows.
pub(crate) fn quote_argument(value: &str) -> String {
    #[cfg(target_os = "linux")]
    return quote_desktop_argument(value);
    #[cfg(target_os = "windows")]
    return quote_windows_argument(value);
    #[cfg(target_os = "macos")]
    value.to_owned() // LaunchAgent already stores each argument separately.
}

#[cfg(any(target_os = "linux", test))]
fn quote_desktop_argument(value: &str) -> String {
    // Exec quoting is decoded after desktop-entry string escaping, hence doubled escapes.
    // https://specifications.freedesktop.org/desktop-entry/latest/exec-variables.html
    let mut quoted = String::from("\"");
    for c in value.chars() {
        match c {
            '\\' => quoted.push_str(r"\\\\"),
            '"' | '`' | '$' => {
                quoted.push_str(r"\\");
                quoted.push(c);
            }
            '%' => quoted.push_str("%%"),
            '\n' => quoted.push_str(r"\n"),
            '\r' => quoted.push_str(r"\r"),
            '\t' => quoted.push_str(r"\t"),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

#[cfg(any(target_os = "windows", test))]
fn quote_windows_argument(value: &str) -> String {
    // Backslashes must be doubled before a quote, including the closing quote.
    // https://learn.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments
    let mut quoted = String::from("\"");
    let mut backslashes = 0;
    for c in value.chars() {
        if c == '\\' {
            backslashes += 1;
        } else {
            let count = if c == '"' {
                backslashes * 2 + 1
            } else {
                backslashes
            };
            quoted.push_str(&"\\".repeat(count));
            quoted.push(c);
            backslashes = 0;
        }
    }
    quoted.push_str(&"\\".repeat(backslashes * 2));
    quoted.push('"');
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_arguments_preserve_spaces_and_reserved_characters() {
        for (input, expected) in [
            ("", r#""""#),
            (
                "/home/me/My Apps/work_log",
                r#""/home/me/My Apps/work_log""#,
            ),
            (
                "--file=/home/me/Logs & Notes/日本語.csv",
                r#""--file=/home/me/Logs & Notes/日本語.csv""#,
            ),
            (r#"a\b"c$d`e%f"#, r#""a\\\\b\\"c\\$d\\`e%%f""#),
            ("a\nb\rc\td", r#""a\nb\rc\td""#),
        ] {
            assert_eq!(quote_desktop_argument(input), expected, "{input:?}");
        }
    }

    #[test]
    fn windows_arguments_preserve_spaces_quotes_and_backslashes() {
        for (input, expected) in [
            ("", r#""""#),
            (
                r"C:\Program Files\Work Log\work_log.exe",
                r#""C:\Program Files\Work Log\work_log.exe""#,
            ),
            (
                r"--file=C:\My Logs\日本語 & notes.csv",
                r#""--file=C:\My Logs\日本語 & notes.csv""#,
            ),
            (r"C:\My Logs\", r#""C:\My Logs\\""#),
            (r#"say "hello""#, r#""say \"hello\"""#),
            (r#"a\"b"#, r#""a\\\"b""#),
        ] {
            assert_eq!(quote_windows_argument(input), expected, "{input:?}");
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn launch_agent_arguments_are_not_command_line_quoted() {
        let value = "--file=/Users/me/My Logs/work.csv";
        assert_eq!(quote_argument(value), value);
    }
}
