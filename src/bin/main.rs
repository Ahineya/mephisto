extern crate mephisto;

use clap::Parser;
use mephisto::codegen::codegen_js::JSCodeGenerator;
use mephisto::module_loader::NativeFileLoader;
use crate::mephisto::Mephisto;
use colored::Colorize;
use mephisto::codegen::codegen_wat::WATCodeGenerator;
use mephisto::codegen::CodeGenerator;

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
    let codegen: Box<dyn CodeGenerator> = match args.target.as_str() {
        "js" => Box::new(JSCodeGenerator::new()),
        "wasm" => Box::new(WATCodeGenerator::new()),
        _ => panic!("Unknown target: {}", args.target),
    };

    let mut mephisto = Mephisto::new(loader);

    if args.target.as_str() == "wasm" {
        println!("{}", "WASM compilation is not ready yet, the result module will not work".red().bold());
    }

    println!("{} {}", "Compiling".green(), args.input);
    println!("{} {}", "Target".green(), args.target);

    // We want to calculate elapsed time
    let start = std::time::Instant::now();

    let compilation_result = mephisto.compile(&args.input, codegen);

    match compilation_result {
        Ok(res) => {
            let elapsed = start.elapsed();

            // if subsec_millis is closer to 1000, we want to show seconds

            let elapsed = format!("{}m {}s {}ms", elapsed.as_secs() / 60, elapsed.as_secs() % 60, elapsed.subsec_millis());

            if let Some(output) = args.output {
                std::fs::write(output, res).expect("Unable to write file");

                println!("{} in {}", "Finished".green().bold(), elapsed);
            } else {
                println!("{}", res);
                println!("{} in {}", "Finished".green().bold(), elapsed);
            }
        },
        Err(e) => println!("{}: {:#?}", "Compilation failed".red().bold(), e),
    }
}
