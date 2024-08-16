use anyhow::Result;
use const_format::concatcp;

const ENV_PREFIX: &str = "PROXY";
const ENV_NAME: &str = concatcp!(ENV_PREFIX, "_NAME");
const ENV_TOKEN_TTL: &str = concatcp!(ENV_PREFIX, "_TOKENTTL");

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tokenttl: i64,
}
impl Config {
    pub fn new_from_env() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix(ENV_PREFIX)
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;
        let c: Self = config.try_deserialize()?;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::ffi::OsStr;

    struct TestEnv {
        name: Option<String>,
        token_ttl: Option<i64>,
    }
    impl Drop for TestEnv {
        fn drop(&mut self) {
            match &self.name {
                Some(name) => env::set_var(ENV_NAME, name),
                None => env::remove_var(ENV_NAME),
            };
            match &self.token_ttl {
                Some(duration) => env::set_var(ENV_TOKEN_TTL, duration.to_string()),
                None => env::remove_var(ENV_TOKEN_TTL),
            }
        }
    }
    impl TestEnv {
        pub fn new_unset_env() -> Result<Self> {
            let env = TestEnv {
                name: env::var(ENV_NAME).ok(),
                token_ttl: env::var(ENV_TOKEN_TTL).ok().map(|f| f.parse().unwrap()),
            };
            env::remove_var(ENV_NAME);
            env::remove_var(ENV_TOKEN_TTL);
            Ok(env)
        }
        pub fn new_set_env<V: AsRef<OsStr>>(name: V, token_ttl: V) -> Result<Self> {
            let res = Self::new_unset_env()?;
            Self::set_name(name);
            Self::set_token_ttl(token_ttl);
            Ok(res)
        }

        pub fn set_name<V: AsRef<OsStr>>(value: V) {
            env::set_var(ENV_NAME, value)
        }

        pub fn set_token_ttl<V: AsRef<OsStr>>(value: V) {
            env::set_var(ENV_TOKEN_TTL, value)
        }
    }

    #[test]
    #[serial]
    fn blank_loads_with_default() -> Result<()> {
        let _t = TestEnv::new_unset_env()?;
        let config = Config::new_from_env()?;
        assert_eq!(
            config,
            Config {
                name: String::from(""),
                tokenttl: 0,
            }
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn loads_ok() -> Result<()> {
        let _t = TestEnv::new_set_env("something", "123")?;
        let config = Config::new_from_env()?;
        assert_eq!(
            config,
            Config {
                name: String::from("something"),
                tokenttl: 123,
            }
        );
        Ok(())
    }
}
