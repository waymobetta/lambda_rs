use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
// { "message": "foo" }
struct InputPayload {
    message: String,
}

#[derive(Serialize, Debug)]
// { "message": "bar" }
struct OutputPayload {
    message: String,
}

impl OutputPayload {
    fn new(message: String) -> Self {
        Self { message }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lambda macro to invoke handler
    lambda!(handler);

    // handle result
    Ok(())
}

fn handler(event: InputPayload, _c: Context) -> Result<OutputPayload, HandlerError> {
    println!("[handler] received input: {}", event.message);

    // init new instance of OutputPayload
    let output_payload: OutputPayload = OutputPayload::new("bar".to_string());

    // return result
    Ok(output_payload)
}
