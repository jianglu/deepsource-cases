// Hardcoded temporary file or directory detected RS-S1003
// Security
// This issue is raised when a hardcoded temporary file or directory is detected. Creating and using insecure temporary files can leave the application vulnerable to attacks. Lack of uniqueness in temporary files allows attackers to predict the filename and inject dangerous data into the application through the temporary file.

// Consider using a crate such as tempfile to generate temporary files or directories securely. Apart from uniqueness, tempfile always cleans up the temporary resources used as it relies on the OS to remove the file when the last handle is closed.

// BAD PRACTICE
// let dir = std::fs::create_dir_all("/tmp/my_app_temp_dir")?;
// RECOMMENDED
// use tempfile::tempdir;

// let dir = tempdir()?;
// REFERENCES
// tempfile
// CWE-377
// OWASP: A01:2021 Broken Access Control

fn main() {
    let dir = std::fs::create_dir_all("/tmp/my_app_temp_dir").unwrap();
}
