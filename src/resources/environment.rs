use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Environment {
    pub values: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::collections::BTreeMap;

    use super::Environment;

    #[test]
    fn test_deserialize() {
        let test_env = Environment {
            values: BTreeMap::from([
                (
                    String::from("URL"),
                    String::from("https://api.abhay.rs/blog"),
                ),
                (String::from("API_KEY"), String::from("api-key")),
            ]),
        };

        if let Ok(env_json) = serde_json::to_string(&test_env) {
            if let Ok(parsed_env) = serde_json::from_str(&env_json) {
                assert_eq!(test_env, parsed_env);
            } else {
                panic!("parsing failed");
            }
        } else {
            panic!("de-serialzing failed");
        }
    }
    #[test]
    fn test_serialize() {
        let control_env = Environment {
            values: BTreeMap::from([
                (
                    String::from("URL"),
                    String::from("https://api.abhay.rs/blog"),
                ),
                (String::from("API_KEY"), String::from("api-key")),
            ]),
        };

        if let Ok(parsed_env) = serde_json::from_str::<Environment>(
            r#"{"name":"local","values":{"API_KEY":"api-key","URL":"https://api.abhay.rs/blog"}}"#,
        ) {
            assert_eq!(parsed_env, control_env)
        } else {
            panic!("From Json conversion failed")
        }
    }

    #[test]
    fn test_get_value_from_environment() {
        let control_env = Environment {
            values: BTreeMap::from([
                (
                    String::from("URL"),
                    String::from("https://api.abhay.rs/blog"),
                ),
                (String::from("API_KEY"), String::from("api-key")),
            ]),
        };

        assert_eq!(
            control_env.values.get("URL"),
            Some(&String::from("https://api.abhay.rs/blog")),
        );
        assert_ne!(
            control_env.values.get("URL"),
            Some(&String::from("https://api.rs/blog")),
        );
    }
}
