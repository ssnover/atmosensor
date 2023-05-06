#![allow(unused)]

use clap::Parser;
use convert_case::{Case, Casing};
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf, process::Stdio};

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

#[derive(Clone, Serialize, Deserialize)]
struct ProtocolFile {
    version: String,
    groups: Vec<CommandGroup>,
}

#[derive(Clone, Serialize, Deserialize)]
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

    let mut env = minijinja::Environment::new();
    env.add_template("message", MESSAGE_STRUCT_TMPL);
    env.add_template("module", MODULE_TMPL);
    env.add_filter("param_case", |v: minijinja::value::Value| {
        let value = serde_json::to_value(v).unwrap();
        let parameter_name = value.as_str().unwrap().to_case(Case::Snake);
        minijinja::value::Value::from_serializable(&parameter_name)
    });
    let tmpl = env.get_template("message").unwrap();

    let command_definitions = protocol
        .groups
        .iter()
        .flat_map(|grp| {
            grp.commands.iter().map(|cmd| {
                tmpl.render(context! { group => grp.number, command => cmd })
                    .unwrap()
            })
        })
        .collect::<Vec<_>>();

    let tmpl = env.get_template("module").unwrap();
    let module_definition = tmpl
        .render(context! { protocol => protocol, commands => command_definitions })
        .unwrap();
    let module_definition = format_rust_source(&module_definition);

    std::fs::write(&args.output_file, module_definition.as_bytes())?;

    Ok(())
}

fn format_rust_source(source: &str) -> std::borrow::Cow<'_, str> {
    if let Ok(mut process) = std::process::Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        {
            let stdin = process.stdin.as_mut().unwrap();
            stdin.write_all(source.as_bytes()).unwrap()
        }
        if let Ok(output) = process.wait_with_output() {
            if output.status.success() {
                return std::str::from_utf8(&output.stdout[..])
                    .unwrap()
                    .to_owned()
                    .into();
            }
        }
    }
    std::borrow::Cow::Borrowed(source)
}
