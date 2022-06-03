use std::collections::HashMap;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub trait Builder {
    fn with_foo(&mut self, value: &str) -> &Self;
    fn with_bar(&mut self, value: &str) -> &Self;
    fn with_baz(&mut self, value: &str) -> &Self;
    fn with_quux(&mut self, value: &str) -> &Self;
    /// To be called at the end of a chained sequence of method calls.
    ///
    /// This consumes self in the process, so the builder may not be used
    /// again afterward.
    fn to_string(self) -> String;
}

pub struct ProductionBuilder<'a> {
    items: HashMap<&'a str, String>,
}

impl<'a> ProductionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<'a> ProductionBuilder<'a> {
    fn with_key(&mut self, key: &'a str, value: &str) -> &Self {
        self.items.insert(key, String::from(value));
        self
    }
}

impl<'a> Builder for ProductionBuilder<'a> {
    fn with_foo(&mut self, value: &str) -> &Self {
        self.with_key("foo", value)
    }

    fn with_bar(&mut self, value: &str) -> &Self {
        self.with_key("bar", value)
    }

    fn with_baz(&mut self, value: &str) -> &Self {
        self.with_key("baz", value)
    }

    fn with_quux(&mut self, value: &str) -> &Self {
        self.with_key("quux", value)
    }

    fn to_string(self) -> String {
        let mut result = String::new();
        let get_line_start = |current_result: &String| {
            if current_result.len() > 0 {
                return "\n";
            }
            ""
        };
        let mut add = |label: &str, value: &String| {
            result.push_str(&format!("{}{}: {}", get_line_start(&result), label, value));
        };
        if let Some(value) = self.items.get("foo") {
            add("foo", value);
        }
        if let Some(value) = self.items.get("bar") {
            add("bar", value);
        }
        if let Some(value) = self.items.get("baz") {
            add("baz", value);
        }
        if let Some(value) = self.items.get("quux") {
            add("quux", value);
        }
        result
    }
}

#[cfg_attr(test, automock)]
pub trait BuilderWrapper {
    fn set_foo(&mut self, value: &str);
    fn set_bar(&mut self, value: &str);
    fn set_baz(&mut self, value: &str);
    fn set_quux(&mut self, value: &str);
    fn to_string(self) -> String;
}

/// Takes a BuilderWrapper and returns the result of a sequence
/// of method calls.
pub fn process<A: BuilderWrapper>(mut builder_wrapper: A) -> String {
    builder_wrapper.set_foo("some foo");
    builder_wrapper.set_bar("some bar");
    builder_wrapper.set_baz("some baz");
    builder_wrapper.set_quux("some quux");
    builder_wrapper.to_string()
}

#[test]
fn test_process() {
    let mut mock_wrapper = MockBuilderWrapper::new();
    mock_wrapper
        .expect_set_foo()
        .with(eq("some foo"))
        .return_once(|_| ());
    mock_wrapper
        .expect_set_bar()
        .with(eq("some bar"))
        .return_once(|_| ());
    mock_wrapper
        .expect_set_baz()
        .with(eq("some baz"))
        .return_once(|_| ());
    mock_wrapper
        .expect_set_quux()
        .with(eq("some quux"))
        .return_once(|_| ());
    mock_wrapper
        .expect_to_string()
        .return_once(|| String::from("awesome!"));
    let result = process(mock_wrapper);
    assert_eq!(result, String::from("awesome!"));
}
