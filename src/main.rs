#[cfg(feature = "decode")]
use base1000::decode;
#[cfg(feature = "encode")]
use base1000::encode;

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
    #[cfg(feature = "encode")]
    #[arg(short, long)]
    encode: bool,

    #[cfg(feature = "decode")]
    #[arg(short, long)]
    decode: bool,
}

fn main() {
    let args = Cli::parse();
    #[cfg(feature = "encode")]
    if args.opt.encode {
        let result: String = encode(args.text);
        println!("{}", result);
        return;
    }
    #[cfg(feature = "decode")]
    if args.opt.decode {
        for result in decode(args.text) {
            println!("{}", result);
        }
    }
}
