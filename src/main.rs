use clap::Parser;

mod free;
mod quotes;

#[derive(Parser)]
#[command(version, about = "Display free and used memory with freeze ❄️", long_about = None)]
struct Args {
    /// Display memory in human readable format
    #[arg(short = 'H', long)]
    human: bool,
    /// Display memory in bytes
    #[arg(short = 'b', long)]
    byte: bool,
    /// Display memory in kilobytes
    #[arg(short = 'k', long)]
    kilo: bool,
    /// Display memory in megabytes
    #[arg(short = 'm', long)]
    mega: bool,
    /// Display memory in gigabytes
    #[arg(short = 'g', long)]
    giga: bool,
}

fn main() {
    let args = Args::parse();

    let mode = if args.human {
        free::Mode::Human
    } else if args.giga {
        free::Mode::GigaByte
    } else if args.mega {
        free::Mode::MegaByte
    } else if args.kilo {
        free::Mode::KiloByte
    } else if args.byte {
        free::Mode::Byte
    } else {
        free::Mode::Human
    };
    free::free(mode);
    println!();
    quotes::quote();
}
