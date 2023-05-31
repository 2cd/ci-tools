use crate::env_name::EnvName;
use std::path::{Path, PathBuf};

/// Gets the GitHub environment name and its value based on the command name.
///
/// - Assuming the command is `.\set-gh-env.exe`, then the bin name is `set-gh-env` and env is `GITHUB_ENV`.
/// - Assuming the command is `/bin/set-gh-output`, then the bin name is `set-gh-output` and env is `GITHUB_OUTPUT`.
///
/// When calling `GhVar::new()` to create an instance, the corresponding environment variable value will be automatically obtained.
///
/// # Example
///
/// ```no_run
/// use crate::gh_var::get_gh_var;
///
/// let cmd = ".\\set-gh-env.exe";
/// let gh_var = get_gh_var(cmd);
/// ```
pub(crate) fn get_gh_var(cmd: &str) -> GhVar {
    let bin = Path::new(cmd)
        .file_stem()
        .expect("Invalid file name");

    let is_eq = |s| bin.eq_ignore_ascii_case(s);
    let is_bin_gh_output = is_eq("set-gh-op") || is_eq("set-gh-output");

    let env_name = if is_bin_gh_output { EnvName::Output } else { EnvName::Env };
    let gh_var = GhVar::new(env_name);
    eprintln!("{:?}", gh_var);
    gh_var
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct GhVar {
    pub(crate) env: EnvName,
    pub(crate) file: PathBuf,
}

impl GhVar {
    pub(crate) fn new(env: EnvName) -> Self {
        let file = PathBuf::from(env.get_env_value());
        Self { env, file }
    }
}
