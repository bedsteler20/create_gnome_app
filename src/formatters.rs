use inflector::Inflector;

pub fn snake_case(input: String) -> String {
    return input.to_snake_case();
}

pub fn dash_case(input: String) -> String {
    return input.to_kebab_case();
}

pub fn camel_case(input: String) -> String {
    return input.to_camel_case();
}

pub fn pascal_case(input: String) -> String {
    return input.to_pascal_case();
}

pub fn path_case(input: String) -> String {
    return format!("/{}", input.replace(".", "/"));
}

pub fn const_case(input: String) -> String {
    return input.to_snake_case().to_uppercase();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert_eq!(snake_case("Hello World".to_string()), "hello_world");
    }

    #[test]
    fn test_dash_case() {
        assert_eq!(dash_case("Hello World".to_string()), "hello-world");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("Hello World".to_string()), "helloWorld");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(pascal_case("Hello World".to_string()), "HelloWorld");
    }

    #[test]
    fn test_path_case() {
        assert_eq!(path_case("hello.world".to_string()), "/hello/world");
    }
}