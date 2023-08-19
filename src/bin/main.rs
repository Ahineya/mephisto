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
    let compilation_result = mephisto.compile("

import Math from \"./math.auo\";
import Kick from \"./kick.auo\";

param frequency {
    min: 40;
    max: 22000;
    step: 1;
    initial: 220;
}

let a = 1;

buffer b[1024];

buffer moo[10] = |i| {
    return i * 2;
};

output out = 0;

let phase = 0;
let increment = 0;

const SR = 44100;

input gain = 1 + 0.5 * getSin(0.5 + (Math.foo));
input kick = 0;

block {
    increment = frequency / SR;
    return 123;
}

getSaw(phase) {
    return phase * 2 - 1;
}

export const PI = 3.14;

export getSin(phase) {
    let b = 1;
    return sin(phase * 2 * PI);
}

process {
    const PI = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;
    phase = increment + (phase - floor(increment + phase));
    out = (phase > -0.5) * 2 - 1;
    out = out * gain;

    let a = 0;

    const test = floor(2.5);

    getPoo() {
        return 1;
    }

    a = 123;

    //let a = 0;

    return a + 1.1;
}

connect {
    out -> OUTPUTS[0];
    out -> OUTPUTS;

    phase -> Kick.phase;
    gain -> Kick.gain;

    Kick.out -> kick;
}

    ".to_string());

    match compilation_result {
        Ok(_) => println!("Compilation successful!"),
        Err(e) => println!("Compilation failed: {:#?}", e),
    }
}
