use clap::Parser;

fn main() {
    let opts = projector_rust::opts::Opts::parse();
    println!("{:?}", opts);
}
