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
    mephisto.tokenize("
import {getSin, smockva} from \"./math.auo\";
import Kick from \"./kick.auo\";

param frequency {
    min: 40,
    max: 22000,
    step: 1,
    initial: 220
}

output out = 0;

let phase = 0;
let increment = 0;

const SR = 44100;

input gain = 1;
input kick = 0;

block {
   increment = frequency / SR;
}

    process {
phase = increment + (phase - floor(increment + phase));
out = (phase > 0.5) * 2 - 1;
out = out * gain;
    }

export getSaw(phase) {
    return phase * 2 - 1;
}

connect {
    out -> OUTPUTS[0];
    out -> OUTPUTS[1];

    phase -> Kick.phase;
    gain -> Kick.gain;

    Kick.out -> kick;
}

    ".to_string());
}
