extern crate mephisto;

use clap::Parser;
use mephisto::codegen::codegen_js::JSCodeGenerator;
use mephisto::module_loader::NativeFileLoader;
use crate::mephisto::Mephisto;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,

    /// Output file, stdout if not present
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Input file: {}", args.input);

    let loader = NativeFileLoader;
    let codegen = JSCodeGenerator::new();
    let mut mephisto = Mephisto::new(loader);
    let compilation_result = mephisto.compile(&args.input, Box::new(codegen));

    match compilation_result {
        Ok(res) => {
            println!("Compilation successful!");
            println!("Result: {}", res);
        },
        Err(e) => println!("Compilation failed: {:#?}", e),
    }
}
