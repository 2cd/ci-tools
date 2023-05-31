use std::env;

#[derive(Debug)]
pub(crate) enum EnvName {
    Env,
    Output,
}

impl EnvName {
    pub(crate) fn get_env_value(&self) -> String {
        let name = self.defaut_env_name();

        env::var(name).unwrap_or_else(|e| {
            eprintln!("Failed to get: ${name}\nErr: {e}");
            self.default_fname()
        })
    }

    /// Gets the corresponding env name based on the variant of the enum
    pub(crate) const fn defaut_env_name(&self) -> &'static str {
        use EnvName::*;
        match self {
            Output => "GITHUB_OUTPUT",
            _ => "GITHUB_ENV",
        }
    }

    /// If the value for the environment variable is not found, call this function to get the default file name.
    pub(crate) fn default_fname(&self) -> String {
        use EnvName::*;
        match self {
            Output => "gh-output.tmp",
            _ => "gh-env.tmp",
        }
        .into()
    }
}
