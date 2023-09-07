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

    /// Output target, default is js
    #[arg(short, long, default_value = "js")]
    target: String,
}

fn main() {
    let args = Args::parse();

    let loader = NativeFileLoader;
    let codegen = JSCodeGenerator::new();
    let mut mephisto = Mephisto::new(loader);
    let compilation_result = mephisto.compile(&args.input, Box::new(codegen));

    match compilation_result {
        Ok(res) => {
            if let Some(output) = args.output {
                std::fs::write(output, res).expect("Unable to write file");

                println!("Compilation successful");
            } else {
                println!("{}", res);
            }
        },
        Err(e) => println!("Compilation failed: {:#?}", e),
    }
}
