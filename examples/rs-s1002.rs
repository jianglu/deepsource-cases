// Potentially unsafe usage of std::fs::remove_dir_all RS-S1002
// Security
// In the standard library in Rust before 1.58.1, there is a race condition that enables symlink following. An attacker could take advantage of this security vulnerability to trick a privileged program into deleting files and directories the attacker couldn't otherwise access or delete.

// std::fs::remove_dir_all includes checks to ensure that symlinks are not followed to avoid recursively deleting symlinks. However, the check was implemented incorrectly, resulting in a TOCTOU (Time-of-check Time-of-use) race condition:

// attacker creates a directory
// system checks if directory is a symlink
// attacker replaces directory with a symlink
// system proceeds to delete the directory
// This bug has since been fixed, consider upgrading to a newer version of Rust to mitigate this issue. To let DeepSource know which version of Rust your project builds against, set the msrv field under analyzers.meta in your .deepsource.toml file.

// BAD PRACTICE
// // vulnerable to attacks on all versions of Rust before 1.58.1
// std::fs::remove_dir_all("/some/path");
// REFERENCES
// Security Advisory by the Rust Core Team
// CWE-367
// CVE-2022-21658
// DeepSource Rust Docs

fn main() {
    // vulnerable to attacks on all versions of Rust before 1.58.1
    std::fs::remove_dir_all("/some/path");
}
