use tracing::{span, event, Level};
use tracing_subscriber;

// Call like this e.g.
// RUST_LOG=trace cargo run

pub fn example1() {
    println!("Hello, tracing!");
    tracing_subscriber::fmt::init();

    event!(Level::INFO, "something happened");

    let span = span!(Level::TRACE, "my_span");

    // `enter` returns a RAII guard which, when dropped, exits the span. this
    // indicates that we are in the span for the current lexical scope.
    let _enter = span.enter();
    // perform some work in the context of `my_span`...
    //

    // records an event within "my_span".
    event!(Level::DEBUG, "something happened inside my_span");

}
