use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

mod ntl;

#[derive(StructOpt)]
enum UscriptNtl {
    /// Print information about an NTL file
    Info(InfoCommand),
    /// Convert an NTL file to JSON
    #[structopt(name = "ntl2json")]
    NtlToJson(NtlToJsonCommand),
    /// Convert a JSON file to NTL
    #[structopt(name = "json2ntl")]
    JsonToNtl(JsonToNtlCommand),
}

#[derive(StructOpt)]
struct InfoCommand {
    /// NTL file
    #[structopt(name = "NTL file")]
    ntl_file: PathBuf,
}

impl InfoCommand {
    fn execute(&self) -> Result<()> {
        let mut items = ntl::read_ntl(&self.ntl_file)?;
        items.sort_by(|a, b| a.opcode.cmp(&b.opcode));

        for item in items {
            println!("{:?}", item);
        }
        Ok(())
    }
}

#[derive(StructOpt)]
struct NtlToJsonCommand {
    /// NTL file
    #[structopt(name = "NTL file")]
    ntl_file: PathBuf,

    /// Output file
    output_file: Option<PathBuf>,
}

impl NtlToJsonCommand {
    fn execute(&self) -> Result<()> {
        println!("Reading NTL file...");
        let items = ntl::read_ntl(&self.ntl_file)?;

        println!("Converting to JSON...");
        let json = serde_json::to_string_pretty(&items)?;

        let out_path = match &self.output_file {
            Some(p) => p.to_str().unwrap(),
            None => "output.json",
        };
        let mut out_file = File::create(out_path)?;
        out_file.write_all(json.as_bytes())?;

        println!("Output written to {}", out_path);

        Ok(())
    }
}

#[derive(StructOpt)]
struct JsonToNtlCommand {
    /// JSON file
    #[structopt(name = "JSON file")]
    json_file: PathBuf,

    /// Output file
    output_file: Option<PathBuf>,
}

impl JsonToNtlCommand {
    fn execute(&self) -> Result<()> {
        println!("Reading JSON file...");
        let file = File::open(&self.json_file)?;
        let reader = BufReader::new(file);

        println!("Parsing JSON...");
        let items = serde_json::from_reader(reader)?;

        let out_path = match &self.output_file {
            Some(p) => p.to_str().unwrap(),
            None => "output.NTL",
        };
        println!("Writing to NTL file...");
        ntl::write_ntl(&items, &PathBuf::from(out_path))?;

        println!("Output written to {}", out_path);
        Ok(())
    }
}

fn main() -> Result<()> {
    match UscriptNtl::from_args() {
        UscriptNtl::Info(command) => command.execute(),
        UscriptNtl::NtlToJson(command) => command.execute(),
        UscriptNtl::JsonToNtl(command) => command.execute(),
    }
}
