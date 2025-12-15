use mx_error::Error;

pub mod path;
pub mod prelude;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context<Key: ToString> {
    fn get(&self, key: Key) -> Result<&dyn ToString>;
}

#[cfg(test)]
mod test {
    use mx_error::NotFoundError;

    use crate::{Context, Result};

    #[derive(Debug, Clone, Copy)]
    enum CustomContextKey {
        Host,
        Port,
    }

    impl ToString for CustomContextKey {
        fn to_string(&self) -> String {
            match self {
                Self::Host => String::from("host"),
                Self::Port => String::from("port"),
            }
        }
    }

    struct CustomContext {
        host: String,
        port: usize,
    }

    impl Context<CustomContextKey> for CustomContext {
        fn get(&self, key: CustomContextKey) -> Result<&dyn ToString> {
            match key {
                CustomContextKey::Host => Ok(&self.host),
                CustomContextKey::Port => Ok(&self.port),
            }
        }
    }

    impl Context<&str> for CustomContext {
        fn get(&self, key: &str) -> Result<&dyn ToString> {
            match key {
                "host" => Ok(&self.host),
                "port" => Ok(&self.port),
                v => Err(NotFoundError::from(format!("key '{}' not found", v)).into()),
            }
        }
    }

    #[test]
    pub fn should_create_context() {
        let context = CustomContext {
            host: String::from("test"),
            port: 3000,
        };

        let mut host = context.get(CustomContextKey::Host).unwrap();
        let mut port = context.get(CustomContextKey::Port).unwrap();

        debug_assert_eq!(host.to_string(), "test");
        debug_assert_eq!(port.to_string(), "3000");

        host = context.get("host").unwrap();
        port = context.get("port").unwrap();

        debug_assert_eq!(host.to_string(), "test");
        debug_assert_eq!(port.to_string(), "3000");
    }
}
