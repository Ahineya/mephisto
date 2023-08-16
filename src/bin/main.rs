extern crate mephisto;

use clap::Parser;
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
    // let args = Args::parse();

    // println!("Input file: {}", args.input);
    // println!("Output file: {:?}", args.output);

    let mephisto = Mephisto::new();
    let tokens = mephisto.tokenize("

    process {
phase = increment + (phase - floor(increment + phase));
out = (phase > 0.5) * 2 - 1;
out = out * gain;

test = floor(2.5);
    }

    ".to_string());

    let ast = mephisto.parse(tokens);

    println!("{:#?}", ast);
}
