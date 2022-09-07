use clap::Parser;
use json_to_class::generators::dart_generator::DartClassGenerator;
use json_to_class::generators::NamingRule;
use json_to_class::json_to_class;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    input: Option<PathBuf>,

    #[clap(short, long, default_value_t = String::from("Untitled"), value_parser)]
    name: String,

    /// the naming rule of json key
    #[clap(short = 'r', long, default_value = "none", value_enum, value_parser)]
    naming_rule: NamingRule,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    if cli.input.is_none() {
        println!("No input file");
        exit(1);
    }

    let file_content = fs::read_to_string(cli.input.unwrap())?;
    let class = json_to_class(
        file_content.as_str(),
        DartClassGenerator::new(
            cli.name.as_str(),
            cli.naming_rule,
        ),
    )?;
    println!("{}", class);

    Ok(())
}