#[cfg(all(feature = "clap", feature = "decode"))]
use base1000::decode;
#[cfg(all(feature = "clap", feature = "encode"))]
use base1000::encode;
#[cfg(feature = "clap")]
use clap::{Args, Parser};

#[cfg(feature = "clap")]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    opt: Opt,
    text: String,
}

#[cfg(feature = "clap")]
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
    #[cfg(feature = "clap")]
    let args = Cli::parse();
    #[cfg(all(feature = "clap", feature = "encode"))]
    if args.opt.encode {
        let result: String = encode(args.text);
        println!("{}", result);
        return;
    }
    #[cfg(all(feature = "clap", feature = "decode"))]
    if args.opt.decode {
        for result in decode(args.text) {
            println!("{}", result);
        }
    }
}
