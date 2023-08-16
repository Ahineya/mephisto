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
output out = 0;

let phase = 0;
let increment = 0;

const SR = 44100;

input gain = 1 + 0.5 * getSin(0.5 + (moo.foo));
input kick = 0;

block {
    increment = frequency / SR;
}

process {
    phase = increment + (phase - floor(increment + phase, -2));
    out = (phase > -0.5) * 2 - 1;
    out = out * gain.value;

    test = floor(2.5);

    a = foo.value;
}

    ".to_string());

    let ast = mephisto.parse(tokens);

    println!("{:#?}", ast);
}
