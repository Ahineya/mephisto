extern crate mephisto;

use clap::Parser;
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

struct Test {}

#[derive(Debug)]
struct Context {
    to_process: Box<Vec<String>>,
    loaded_modules: Box<Vec<String>>,
}

impl Test {
    fn new() -> Self {
        Test {}
    }

    fn test(&mut self) {

        let mut context = Context {
            to_process: Box::new(Vec::new()),
            loaded_modules: Box::new(Vec::new()),
        };

        context.to_process.push("test".to_string());
        context.to_process.push("test2".to_string());
        context.to_process.push("test3".to_string());

        let result = self.test_recursive(&mut context);

        println!("Result: {:#?}", result);

        println!("Context: {:#?}", context);
    }

    fn test_recursive(&mut self, context: &mut Context) -> Result<(), String> {
        let module_name = context.to_process.pop().unwrap();

        if context.to_process.len() > 0 {
            self.test_recursive(context)?;
        }

        if context.loaded_modules.contains(&module_name) {
            return Ok(());
        }

        context.loaded_modules.push(module_name.clone());

        Ok(())
    }
}

fn main() {

    // let mut test = Test::new();
    // test.test();
    //
    // return;
    let args = Args::parse();

    println!("Input file: {}", args.input);
    // println!("Output file: {:?}", args.output);

    let loader = NativeFileLoader;
    let mut mephisto = Mephisto::new(loader);
    let compilation_result = mephisto.compile(&args.input);

    match compilation_result {
        Ok(_) => println!("Compilation successful!"),
        Err(e) => println!("Compilation failed: {:#?}", e),
    }
}
