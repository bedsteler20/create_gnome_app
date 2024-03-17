pub struct Engine {
    variables: Vec<(String, String)>,
}

impl Engine {
    pub fn format(&self, input: &String) -> String {
        let mut result = input.clone();
        for (name, value) in &self.variables {
            result = result.replace(&format!("{{{{{}}}}}", name), value);
        }

        result
    }
}

pub struct EngineBuilder {
    pub formatters: Vec<(String, Box<dyn Fn(String) -> String>)>,
    variables: Vec<(String, String)>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        EngineBuilder {
            formatters: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn add_formatter<F>(mut self, name: &str, formatter: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        self.formatters
            .push((name.to_string(), Box::new(formatter)));
        self
    }

    pub fn add_variable(mut self, name: &str, value: String) -> Self {
        self.variables.push((name.to_string(), value));
        self
    }

    pub fn build(self) -> Engine {
        let mut variables = self.variables.clone();

        for (name, formatter) in self.formatters {
            for (var_name, var_value) in self.variables.clone() {
                let new_value = formatter(var_value.clone());
                variables.push((format!("{} {}", name, var_name), new_value));
            }
        }

        Engine { variables }
    }
}

#[cfg(test)]
mod test {
    use crate::formatters;

    use super::*;
    #[test]
    fn test_variable_formatting() {
        let engine = EngineBuilder::new()
            .add_variable("message", "Hello".to_string())
            .build();
        assert_eq!(
            engine.format(&"{{message}} World".to_string()),
            "Hello World"
        );
    }

    #[test]
    fn test_formatter_formatting() {
        let engine = EngineBuilder::new()
            .add_variable("message", "HelloWorld".to_string())
            .add_formatter("snakeCase", formatters::snake_case)
            .build();
        assert_eq!(
            engine.format(&"{{snakeCase message}} World".to_string()),
            "hello_world World"
        );
    }
}
