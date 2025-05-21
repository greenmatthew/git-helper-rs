use std::process::{Command, Stdio};

pub fn execute(remotes: &[(String, String)]) -> Result<(), String> {
    // Get list of all existing remotes
    let remotes_output = Command::new("git")
        .args(["remote"])
        .output()
        .map_err(|e| format!("Failed to list remotes: {e}"))?;

    // Remove each remote
    for remote in String::from_utf8_lossy(&remotes_output.stdout).lines() {
        if !remote.is_empty() {
            let remove_output = Command::new("git")
                .args(["remote", "remove", remote])
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("Failed to remove remote {remote}: {e}"))?;

            if !remove_output.status.success() {
                let error = String::from_utf8_lossy(&remove_output.stderr);
                return Err(format!("Failed to remove remote {remote}: {error}"));
            }
        }
    }

    // Add remotes if provided
    let include_all_remote = remotes.len() > 1;

    for (i, (name, url)) in remotes.iter().enumerate() {
        let add_remote_output = Command::new("git")
            .args(["remote", "add", name, url])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to add remote {name}: {e}"))?;

        if !add_remote_output.status.success() {
            let error = String::from_utf8_lossy(&add_remote_output.stderr);
            return Err(format!("Failed to add remote {name}: {error}"));
        }

        // Create and update the "all" remote if we have multiple remotes
        if include_all_remote {
            if i == 0 {
                // Create the "all" remote with the first URL
                let create_all_output = Command::new("git")
                    .args(["remote", "add", "all", url])
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(|e| format!("Failed to create 'all' remote: {e}"))?;

                if !create_all_output.status.success() {
                    let error = String::from_utf8_lossy(&create_all_output.stderr);
                    return Err(format!("Failed to create 'all' remote: {error}"));
                }
            }

            // Add additional URLs to the "all" remote
            let update_all_output = Command::new("git")
                .args(["remote", "set-url", "--add", "--push", "all", url])
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("Failed to update 'all' remote: {e}"))?;

            if !update_all_output.status.success() {
                let error = String::from_utf8_lossy(&update_all_output.stderr);
                return Err(format!("Failed to update 'all' remote: {error}"));
            }
        }
    }

    println!("Remote configuration has been successfully reinitialized.");
    Ok(())
}
