use base1000::{decode, encode};
use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    opt: Opt,
    text: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Opt {
    #[arg(short, long)]
    encode: bool,

    #[arg(short, long)]
    decode: bool,
}

fn main() {
    let args = Cli::parse();
    if args.opt.encode {
        let result: String = encode(args.text);
        println!("{}",result);
    } else {
        for result in decode(args.text) {
        println!("{}",result);

        }
    }
}
