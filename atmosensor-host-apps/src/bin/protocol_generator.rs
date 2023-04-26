#![allow(unused)]

use clap::Parser;
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const MESSAGE_STRUCT_TMPL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/message_struct.rs.j2"
));
const MODULE_TMPL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/module.rs.j2"));

#[derive(Parser)]
struct Args {
    #[arg(short = 'p')]
    protocol_file: PathBuf,
    #[arg(short = 'o')]
    output_file: PathBuf,
}

#[derive(Clone, Deserialize)]
struct ProtocolFile {
    version: String,
    groups: Vec<CommandGroup>,
}

#[derive(Clone, Deserialize)]
struct CommandGroup {
    group: String,
    number: u8,
    commands: Vec<Command>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Command {
    name: String,
    associated_request: Option<String>,
    number: u8,
    description: String,
    parameters: Vec<Parameter>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Parameter {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let protocol = std::fs::read_to_string(args.protocol_file)?;
    let protocol = json5::from_str::<ProtocolFile>(&protocol)?;

    protocol
        .groups
        .iter()
        .for_each(|g| g.commands.iter().for_each(|cmd| println!("{}", cmd.name)));

    let test_cmd = protocol.groups[0].commands[0].clone();

    let mut env = minijinja::Environment::new();
    env.add_template("message", MESSAGE_STRUCT_TMPL);
    env.add_template("module", MODULE_TMPL);

    let tmpl = env.get_template("message").unwrap();
    println!("{}", tmpl.render(context! { command => test_cmd }).unwrap());

    Ok(())
}
