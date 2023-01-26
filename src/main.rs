use clap::Parser;
use std::fs::File;
use std::io::Write;

mod util;
mod types;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_path: Option<String>,
    #[arg(short, long)]
    output_path: Option<String>,
}

fn generate_html(input_path: String) -> std::string::String {
    let body = util::file::read_file(input_path).unwrap();
    let profile: types::Profile = util::toml::parse_toml_to_config(body).unwrap();
    let page = util::askama::generate_page(profile);
    page
}

fn write_html(output_path: String, contents: String) {
    let mut output = File::create(output_path).unwrap();
    let _ = writeln!(output, "{}", contents);
}

fn main() {
    let args = Args::parse();

    if let Some(input_path) = args.input_path.as_deref() {
        let page = generate_html(input_path.to_string());

        if let Some(output_path) = args.output_path.as_deref() {
            write_html(output_path.to_string(), page);
        } else {
            println!("no output path given.");
        }
    } else {
        println!("no filepath given.");
    }
}
