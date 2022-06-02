use std::collections::HashMap;

pub trait Builder {
    fn with_foo(self, value: &str) -> Self;
    fn with_bar(self, value: &str) -> Self;
    fn with_baz(self, value: &str) -> Self;
    fn with_quux(self, value: &str) -> Self;
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
    fn with_key(mut self, key: &'a str, value: &str) -> Self {
        self.items.insert(key, String::from(value));
        self
    }
}

impl<'a> Builder for ProductionBuilder<'a> {
    fn with_foo(self, value: &str) -> Self {
        self.with_key("foo", value)
    }

    fn with_bar(self, value: &str) -> Self {
        self.with_key("bar", value)
    }

    fn with_baz(self, value: &str) -> Self {
        self.with_key("baz", value)
    }

    fn with_quux(self, value: &str) -> Self {
        self.with_key("quux", value)
    }

    fn to_string(self) -> String {
        let mut result = String::new();
        let get_line_start = |result: &String| {
            if result.len() > 0 {
                return "\n";
            }
            ""
        };
        if let Some(value) = self.items.get("foo") {
            result.push_str(&format!("{}foo: {}", get_line_start(&result), value));
        }
        if let Some(value) = self.items.get("bar") {
            result.push_str(&format!("{}bar: {}", get_line_start(&result), value));
        }
        if let Some(value) = self.items.get("baz") {
            result.push_str(&format!("{}baz: {}", get_line_start(&result), value));
        }
        if let Some(value) = self.items.get("quux") {
            result.push_str(&format!("{}quux: {}", get_line_start(&result), value));
        }
        result
    }
}

/// Takes a Builder implementation and returns the result of chained sequence
/// of method calls.
pub fn process<A: Builder>(builder: A) -> String {
    builder
        .with_foo("some foo")
        .with_bar("some bar")
        .with_baz("some baz")
        .with_quux("some quux")
        .to_string()
}
