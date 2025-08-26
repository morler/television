use directories::UserDirs;
use std::path::{Path, PathBuf};

pub fn expand_tilde<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.starts_with("~") {
        let home = UserDirs::new()
            .map(|dirs| dirs.home_dir().to_path_buf())
            .unwrap_or_else(|| {
                #[cfg(windows)]
                {
                    std::env::var("USERPROFILE")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| {
                            PathBuf::from("C:\\Users\\Default")
                        })
                }
                #[cfg(not(windows))]
                {
                    PathBuf::from("/")
                }
            });
        home.join(path.strip_prefix("~").unwrap())
    } else {
        path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        if let Some(user_dirs) = UserDirs::new() {
            let expected = if cfg!(windows) {
                format!("{}\\test", user_dirs.home_dir().display())
            } else {
                format!("{}/test", user_dirs.home_dir().display())
            };
            assert_eq!(expand_tilde("~/test").to_str().unwrap(), expected);
        }

        assert_eq!(expand_tilde("test").to_str().unwrap(), "test");

        if cfg!(windows) {
            assert_eq!(
                expand_tilde("C:\\absolute\\path").to_str().unwrap(),
                "C:\\absolute\\path"
            );
        } else {
            assert_eq!(
                expand_tilde("/absolute/path").to_str().unwrap(),
                "/absolute/path"
            );
        }
    }

    #[test]
    fn test_expand_tilde_fallback() {
        // Test fallback behavior when UserDirs fails
        // This is difficult to test directly, but we can at least verify
        // that the function doesn't panic with various inputs
        let test_paths =
            vec!["~/", "~/test", "~/.config", "test", "/absolute"];

        for path in test_paths {
            let result = expand_tilde(path);
            assert!(!result.as_os_str().is_empty());
        }
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_paths() {
        // Test Windows-specific path handling
        assert_eq!(expand_tilde("C:\\test"), PathBuf::from("C:\\test"));
        assert_eq!(
            expand_tilde("relative\\path"),
            PathBuf::from("relative\\path")
        );

        // Test that tilde expansion works with Windows paths
        let result = expand_tilde("~\\Documents");
        assert!(result.is_absolute());
        assert!(result.to_string_lossy().contains("Documents"));
    }
}
