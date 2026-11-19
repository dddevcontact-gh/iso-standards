use serde_json::{Value, json}; // lets us use json parsing and value, type to represent json data
use vercel_runtime::{Error, Request, run, service_fn};

// main is the entry point
#[tokio::main] // provides an async runtime, rust crate (rusts package manager is Cargo, self explanatory)
async fn main() -> Result<(), Error> {
    let service = service_fn(handler); // service_fn(handler) wraps handler function so vercel knows how to call it in deployment
    run(service).await; // hands handler to runtime and keeps the function alive to receive requests
}

// return type will either produce a json value type or a lovely error
async fn handler(_req: Request) -> Result<Value, Error> { // receives incoming req as argument, _req because example on vercel doesnt use it and it isnt a taken type
    Ok(json!({ // rusts http 200 successful or whatever it is, just written as "Ok", if youre wondering why the ! is there println!("hi") is rusts syntax, just needs it
        "term": "testing, rust backend sent this buddy", // simple json
    }))
}