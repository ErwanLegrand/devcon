/// Safe flag prefixes that are allowed in `runArgs`.
pub(crate) const ALLOWED_FLAGS: &[&str] = &[
    "--env",
    "--env-file",
    "--label",
    "--name",
    "--hostname",
    "--network",
    "--expose",
    "--publish",
    "--dns",
    "--add-host",
    "--workdir",
    "--user",
    "--memory",
    "--cpus",
    "--memory-swap",
    "--cpu-shares",
    "--shm-size",
    "--ulimit",
    "--tmpfs",
    "--read-only",
    "--init",
    "-e",
    "-p",
    "-u",
    "-w",
    "-h",
];

/// Privilege-escalating flags that are denied in `runArgs`.
pub(crate) const DENIED_FLAGS: &[&str] = &[
    "--privileged",
    "--cap-add",
    "--cap-drop",
    "--security-opt",
    "--device",
    "--pid",
    "--ipc",
    "--userns",
    "--cgroupns",
    "--systctl",
    "--apparmor",
];

/// Validate a list of `runArgs` against the allowlist and denylist.
///
/// - Denied flags → returns `Err` with the offending flag name.
/// - Unknown flags (not in either list) → prints a warning to stderr, does not abort.
/// - Allowed flags → pass silently.
/// - Value arguments (non-flag tokens after a flag) → pass silently.
///
/// # Errors
/// Returns `Err(String)` naming the denied flag if any denied flag is found.
pub(crate) fn validate_run_args(args: &[String]) -> Result<(), String> {
    for arg in args {
        // Only validate tokens that look like flags (start with `-`)
        if !arg.starts_with('-') {
            continue;
        }

        // Extract flag name: the part before `=` (or the whole arg if no `=`)
        let flag = match arg.find('=') {
            Some(pos) => &arg[..pos],
            None => arg.as_str(),
        };

        if DENIED_FLAGS.contains(&flag) {
            return Err(format!(
                "runArgs contains denied flag '{flag}': privilege-escalating flags are not allowed"
            ));
        }

        if !ALLOWED_FLAGS.contains(&flag) {
            eprintln!(
                "warning: runArgs contains unknown flag '{flag}'; allowing but not validated"
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn empty_args_ok() {
        assert!(validate_run_args(&[]).is_ok());
    }

    #[test]
    fn allowed_env_flag_passes() {
        let args = vec![s("--env"), s("FOO=bar")];
        assert!(validate_run_args(&args).is_ok());
    }

    #[test]
    fn allowed_env_flag_with_equals_passes() {
        let args = vec![s("--env=FOO=bar")];
        assert!(validate_run_args(&args).is_ok());
    }

    #[test]
    fn allowed_network_flag_passes() {
        let args = vec![s("--network"), s("bridge")];
        assert!(validate_run_args(&args).is_ok());
    }

    #[test]
    fn allowed_short_flags_pass() {
        let args = vec![s("-e"), s("FOO=bar"), s("-p"), s("8080:8080")];
        assert!(validate_run_args(&args).is_ok());
    }

    #[test]
    fn denied_privileged_aborts() {
        let args = vec![s("--privileged")];
        let result = validate_run_args(&args);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(
            msg.contains("privileged"),
            "error message should contain 'privileged', got: {msg}"
        );
    }

    #[test]
    fn denied_cap_add_aborts() {
        let args = vec![s("--cap-add"), s("SYS_ADMIN")];
        let result = validate_run_args(&args);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(
            msg.contains("cap-add"),
            "error message should contain 'cap-add', got: {msg}"
        );
    }

    #[test]
    fn denied_cap_add_with_equals_aborts() {
        let args = vec![s("--cap-add=SYS_ADMIN")];
        let result = validate_run_args(&args);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(
            msg.contains("cap-add"),
            "error message should contain 'cap-add', got: {msg}"
        );
    }

    #[test]
    fn denied_security_opt_aborts() {
        let args = vec![s("--security-opt"), s("apparmor=unconfined")];
        let result = validate_run_args(&args);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("security-opt"));
    }

    #[test]
    fn unknown_flag_warns_but_passes() {
        // --unknown-flag is not in either list → should warn but return Ok
        let args = vec![s("--unknown-flag")];
        let result = validate_run_args(&args);
        assert!(
            result.is_ok(),
            "unknown flags should warn but not abort, got: {result:?}"
        );
    }

    #[test]
    fn mixed_allowed_and_value_args_pass() {
        // Value args (non-flag tokens) should be skipped
        let args = vec![
            s("--env"),
            s("MY_VAR=hello"),
            s("--network"),
            s("host"),
            s("--label"),
            s("project=myapp"),
        ];
        assert!(validate_run_args(&args).is_ok());
    }

    #[test]
    fn denied_flag_among_allowed_aborts() {
        let args = vec![
            s("--env"),
            s("FOO=bar"),
            s("--privileged"),
            s("--network"),
            s("bridge"),
        ];
        let result = validate_run_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("privileged"));
    }
}
