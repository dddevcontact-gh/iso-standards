use serde_json::{Value, json}; // lets us use json parsing and value, type to represent json data
use vercel_runtime::{Error, Request, run, service_fn};
use rand::Rng;

const DEFINITIONS : [[&str; 2]; 32] = [ // only have two testing definitions in here for now - outer two here is a hardcoded length and needs adjusting as per
    ["Attribute", "Inherent property or characteristic of an entity that can be distinguished quantitatively or qualitatively by human or automated means"],
    ["Component", "Entity with discrete structure, such as an assembly or software module, within a system considered at a particular level of analysis"],
    ["Context of Use", "Users, tasks, equipment (hardware, software and materials), and the physical and social environments in which a product (3.15) is used"],
    ["Data Quality", "Capability of the characteristics of data to satisfy stated and implied needs when used under specified conditions"],
    ["Developer", "Individual or organization that performs development activities (including requirements analysis, design, testing through acceptance) during the system or software life cycle process"],
    ["Direct User", "Person who interacts with the product"],
    ["Evaluator", "Individual or organization that performs an evaluation"],
    ["ICT Product", "Product which uses Information and Communication Technologies (ICTs) and can be part of information system"],
    ["Indirect User", "Person who receives output from a system but does not interact with the system"],
    ["Information System", "System that comprises software, hardware, communication facility, data, and the people who use it in a given environment to satisfy their information processing needs."],
    ["IT Service", "Service that makes use of IT systems as tools to provide value to an individual user or a business by facilitating results the user of business wants to achieve"],
    ["IT Service System", "System that is comprised of an IT Service and the people who use it in a given (user) environment to satisfy their service needs"],
    ["IT Service Quality", "Capability of an IT service to satisfy stated and implied quality needs when delivered under specified conditions"],
    ["Maintainer", "Individual or organisation that performs maintenance activities"],
    ["Product", "Artefact that is produced, is quantifiable, and is deliverable to user as either an end item in itself or a component item"],
    ["Product Quality", "Capability of a system or its components to satisfy stated and implied quality needs when used under specific conditions"],
    ["Quality Characteristics", "Category of quality attributes that bears on the quality of the ICT product or information system"],
    ["Quality-in-use", "Extent to which the system or product, when it is used in a specified context or use satisfies or exceeds stakeholder’s needs to achieve specified beneficial goals or outcomes"],
    ["Quality Measure", "Derived measure that is defined as a measurement function of two or more values of quality measure elements"],
    ["Quality Measure Element", "Measure defined in terms of a property and the measurement method for quantifying it, including optionally the transformation by a mathematical function"],
    ["Quality Model", "Defined set of characteristics and of relationships between them, which provides a framework for specifying quality requirements and evaluating the quality"],
    ["Quality Property", "Property of a target entity that is related to a quality measure element and which can be quantified by a measurement method"],
    ["Quality Requirement", "Requirement for quality properties or attributes of an ICT product, data, or service that satisfy needs which ensue from the purpose for which that ICT product, data or service is to be used"],
    ["Quality Sub-Characteristic", "Set of one or more quality properties that represent a unique aspect of a quality characteristic"],
    ["Requirement", "Individual or organisation having a right, share, claim, or interest in a system or in its possession of characteristics that meet their needs and expectations"],
    ["Stakeholder Need", "Prerequesite identified as necessary for a stakeholder, or a set of stakeholders, to achieve an intended outcome, implied or stated within a specific context of use"],
    ["Sub-Sub-Characteristic", "Subdivision of a quality sub-characteristic established by the user of a quality model to provide more granular representation of the quality attributes of a target entity"],
    ["System", "Arrangement of parts or elements that together exhibit a stated behaviour or meaning that the individual constituents do not."],
    ["Target Entity", "Fundamental thing of relevance to the user, about which information is kept, and which needs to be measured"],
    ["User", "Individual user that interacts with a system or benefits from a system during its utilization"],
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