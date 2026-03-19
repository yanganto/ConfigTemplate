/// A flexible way to handle config
pub struct Config {
    pub name: String,
    pub value: String,
}

impl Config {
    pub fn new(name: &str, value: &str) -> Self {
        Config {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new("key", "value");
        assert_eq!(config.name, "key");
        assert_eq!(config.value, "value");
    }
}
