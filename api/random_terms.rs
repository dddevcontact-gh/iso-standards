use serde_json::{Value, json}; // lets us use json parsing and value, type to represent json data
use vercel_runtime::{Error, Request, run, service_fn};
use rand::Rng;

const DEFINITIONS : [[&str; 2]; 2] = [ // only have two testing definitions in here for now - outer two here is a hardcoded length and needs adjusting as per
    ["Validation", "Confirmation, through the provision of objective evidence, that the requirements for a specific intended use or application have been fulfilled"],
    ["Verification", "Confirmation, through the provision of objective evidence, that specified requirements have been fulfilled"],
];

// main is the entry point
#[tokio::main] // provides an async runtime, rust crate (rusts package manager is Cargo, self explanatory)
async fn main() -> Result<(), Error> {
    let service = service_fn(handler); // service_fn(handler) wraps handler function so vercel knows how to call it in deployment
    run(service).await // hands handler to runtime and keeps the function alive to receive requests
} // do NOT semi colon the line above it makes an error, line just reads await and kills the program if you do

// return type will either produce a json value type or a lovely error
async fn handler(_req: Request) -> Result<Value, Error> { // receives incoming req as argument, _req because example on vercel doesnt use it and it isnt a taken type
    let num = rand::thread_rng().gen_range(0..DEFINITIONS.len()); //inclusive exclusive, pretty sure theres 33 keywords

    Ok(json!({ // rusts http 200 successful or whatever it is, just written as "Ok", if youre wondering why the ! is there println!("hi") is rusts syntax, just needs it
        "term": DEFINITIONS[num][0], // simple json
        "definition": DEFINITIONS[num][1],
        "random number": format!("random: {}", num) // string formatting in json W rust
    }))
}