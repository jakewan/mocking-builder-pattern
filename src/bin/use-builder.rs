use mocking_builder_pattern::{process, Builder, BuilderWrapper, ProductionBuilder};

struct ProductionBuilderWrapper<'a> {
    builder: ProductionBuilder<'a>,
}

/// Production code not exposed to unit tests is minimized in methods that simply
/// make corresponding calls to the wrapped builder value.
impl<'a> BuilderWrapper for ProductionBuilderWrapper<'a> {
    fn set_foo(&mut self, value: &str) {
        self.builder.with_foo(value);
    }

    fn set_bar(&mut self, value: &str) {
        self.builder.with_bar(value);
    }

    fn set_baz(&mut self, value: &str) {
        self.builder.with_baz(value);
    }

    fn set_quux(&mut self, value: &str) {
        self.builder.with_quux(value);
    }

    fn to_string(self) -> String {
        self.builder.to_string()
    }
}

fn main() {
    // Provide a builder instance to `process` as a dependency
    let builder = ProductionBuilder::new();
    let builder_wrapper = ProductionBuilderWrapper { builder };
    let result = process(builder_wrapper);
    if result.len() > 0 {
        println!("{}", result);
    } else {
        println!("Empty result");
    }
}
