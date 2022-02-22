mod tracing_example;

pub use tracing_example::example1;

fn main() {
    println!("Hello, world!");
    tracing_example::example1();
}
