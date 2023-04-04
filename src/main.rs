use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::string::String;

mod types;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_path: Option<String>,
    #[arg(short, long)]
    output_path: Option<String>,
}

enum GenerateHTMLError {
    NoFilePathGiven,
}

enum SaveHTMLError {
    NoFilePathGiven,
    NoContentGiven,
}

fn generate_html(input_path: String) -> Result<String, GenerateHTMLError> {
    if input_path.is_empty() {
        return Err(GenerateHTMLError::NoFilePathGiven);
    }

    let body = util::file::read_file(input_path).unwrap();
    let profile: types::Profile = util::toml::parse_toml_to_config(body).unwrap();
    let page = util::askama::generate_page(profile);

    Ok(page)
}

fn write_html(output_path: String, contents: String) -> Result<(), SaveHTMLError> {
    if output_path.is_empty() {
        return Err(SaveHTMLError::NoFilePathGiven);
    }

    if contents.is_empty() {
        return Err(SaveHTMLError::NoContentGiven);
    }

    let mut output = File::create(output_path).unwrap();
    let _ = writeln!(output, "{}", contents);

    Ok(())
}

fn main() {
    let args = Args::parse();

    let page: String = match generate_html(args.input_path.unwrap()) {
        Ok(page) => page,
        Err(GenerateHTMLError::NoFilePathGiven) => {
            panic!("no filepath given.")
        }
    };

    match write_html(args.output_path.unwrap(), page) {
        Ok(()) => (),
        Err(SaveHTMLError::NoFilePathGiven) => {
            panic!("no output path given.")
        }
        Err(SaveHTMLError::NoContentGiven) => {
            panic!("no contents given")
        }
    };
}
