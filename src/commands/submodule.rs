use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn purge(path: &str) -> Result<(), String> {
    let submodule_path = Path::new(path);
    
    // Step 1: Deinitialize the submodule
    let deinit_output = Command::new("git")
        .args(["submodule", "deinit", "-f", path])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to deinitialize submodule: {e}"))?;
    
    if !deinit_output.status.success() {
        let error = String::from_utf8_lossy(&deinit_output.stderr);
        return Err(format!("Failed to deinitialize submodule: {error}"));
    }
    
    // Step 2: Remove the submodule from .git/modules
    // First, get the git root directory
    let git_dir_output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to get git directory: {e}"))?;
    
    if !git_dir_output.status.success() {
        let error = String::from_utf8_lossy(&git_dir_output.stderr);
        return Err(format!("Failed to get git directory: {error}"));
    }
    
    let git_dir = String::from_utf8_lossy(&git_dir_output.stdout)
        .trim()
        .to_string();
    
    let modules_path = PathBuf::from(&git_dir).join("modules").join(path);
    
    // Remove the modules directory if it exists
    if modules_path.exists() {
        let remove_modules_output = Command::new("rm")
            .args(["-rf", modules_path.to_str().unwrap()])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to remove submodule from .git/modules: {e}"))?;
        
        if !remove_modules_output.status.success() {
            let error = String::from_utf8_lossy(&remove_modules_output.stderr);
            return Err(format!("Failed to remove submodule from .git/modules: {error}"));
        }
    }
    
    // Step 3: Remove the submodule from the working tree
    let remove_submodule_output = Command::new("git")
        .args(["rm", "-f", path])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to remove submodule from working tree: {e}"))?;
    
    if !remove_submodule_output.status.success() {
        let error = String::from_utf8_lossy(&remove_submodule_output.stderr);
        return Err(format!("Failed to remove submodule from working tree: {error}"));
    }
    
    println!("Submodule '{}' has been successfully purged.", path);
    Ok(())
}