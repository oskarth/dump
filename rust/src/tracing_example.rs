use tracing::{span, Level};

pub fn example1() {
    let span = span!(Level::TRACE, "my_span");

    // `enter` returns a RAII guard which, when dropped, exits the span. this
    // indicates that we are in the span for the current lexical scope.
    let _enter = span.enter();
    // perform some work in the context of `my_span`...
    //

    println!("Hello, tracing!");

}
