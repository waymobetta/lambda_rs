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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lambda macro to invoke handler
    lambda!(handler);

    // handle result
    Ok(())
}

fn handler(event: InputPayload, _c: Context) -> Result<OutputPayload, HandlerError> {
    println!("[handler] received input: {}", event.message);

    // return result
    Ok(OutputPayload {
        message: "bar".to_string(),
    })
}
