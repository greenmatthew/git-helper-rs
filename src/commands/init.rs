use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn execute(remotes: Vec<(String, String)>) -> Result<(), String> {
    let current_dir = env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;
    
    // Delete existing .git directory if it exists
    let git_dir = current_dir.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir).map_err(|e| format!("Failed to remove existing .git directory: {e}"))?;
    }
    
    // Initialize git repository
    let init_output = Command::new("git")
        .arg("init")
        .current_dir(&current_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run git init: {e}"))?;
    
    if !init_output.status.success() {
        let error = String::from_utf8_lossy(&init_output.stderr);
        return Err(format!("Git init failed: {error}"));
    }
    
    // Add remotes if provided
    let include_all_remote = remotes.len() > 1;
    
    for (i, (name, url)) in remotes.iter().enumerate() {
        let add_remote_output = Command::new("git")
            .args(["remote", "add", name, url])
            .current_dir(&current_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to add remote {name}: {e}"))?;
        
        if !add_remote_output.status.success() {
            let error = String::from_utf8_lossy(&add_remote_output.stderr);
            // Clean up by removing the .git directory
            let _ = fs::remove_dir_all(&git_dir);
            return Err(format!("Failed to add remote {name}: {error}"));
        }
        
        // Create and update the "all" remote if we have multiple remotes
        if include_all_remote {
            if i == 0 {
                // Create the "all" remote with the first URL
                let create_all_output = Command::new("git")
                    .args(["remote", "add", "all", url])
                    .current_dir(&current_dir)
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(|e| format!("Failed to create 'all' remote: {e}"))?;
                
                if !create_all_output.status.success() {
                    let error = String::from_utf8_lossy(&create_all_output.stderr);
                    // Clean up by removing the .git directory
                    let _ = fs::remove_dir_all(&git_dir);
                    return Err(format!("Failed to create 'all' remote: {error}"));
                }
            }

            // Add additional URLs to the "all" remote
            let update_all_output = Command::new("git")
                .args(["remote", "set-url", "--add", "--push", "all", url])
                .current_dir(&current_dir)
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("Failed to update 'all' remote: {e}"))?;
            
            if !update_all_output.status.success() {
                let error = String::from_utf8_lossy(&update_all_output.stderr);
                // Clean up by removing the .git directory
                let _ = fs::remove_dir_all(&git_dir);
                return Err(format!("Failed to update 'all' remote: {error}"));
            }
        }
    }
    
    Ok(())
}