use mocking_builder_pattern::{process, ProductionBuilder};

fn main() {
    // Provide a builder instance to `process` as a dependency
    let builder = ProductionBuilder::new();
    let result = process(builder);
    if result.len() > 0 {
        println!("{}", result);
    } else {
        println!("Empty result");
    }
}
