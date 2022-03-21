/// simply contains a string detailing what happened.
#[derive(Debug, Clone)]
pub struct EnvSearcherError {
    pub reason: String,
}
use std::fmt;
impl fmt::Display for EnvSearcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

use std::env;

/// gets a environmental variable with the key string.
/// if the variable is not found, returns an EnvSearcherError
/// checks for actual environmental variables, then if none is found, goes to find a ".env" file
/// 
/// example:
/// the .env file:
/// ```env
///     # this is a comment. This is the .env file
///     MY_VERY_COOL_KEY_VALUE=wow
/// ```
/// the program:
/// ```rust
///     use env_searcher::get_env;
///     let value = get_env("MY_VERY_COOL_KEY_VALUE");
///     assert_eq!(value.unwrap(), "wow");
/// ```
pub fn get_var(key: String) -> Result<String, EnvSearcherError> {
    let real_env_var_res = env::var(&key);
    if real_env_var_res.is_ok() {
        return Ok(real_env_var_res.ok().unwrap());
    }
    let dotenv_res = dotenv::var(key);
    if dotenv_res.is_ok() {
        return Ok(dotenv_res.unwrap());
    }
    Err(EnvSearcherError{reason: "No such env var".to_string()})
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_test() {
        let result = crate::get_var("GOOD_TEST_STRING".to_string());
        assert_eq!(result.unwrap(), "hello_world");
    }
    #[test]
    fn number_test() {
        let result = crate::get_var("GOOD_TEST_NUMBER".to_string());
        assert_eq!(result.unwrap(), "12345");
    }
}
