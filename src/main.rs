use clap::Parser;
use rand::prelude::SliceRandom;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use tabled::{
    settings::{object::Rows, Alignment, Style},
    Table, Tabled,
};

mod quotes;

#[derive(Parser)]
#[command(version, about = "Display free and used memory with freeze ❄️", long_about = None)]
struct Args {
    /// Display memory in human readable format
    #[arg(short = 'H', long, default_value = "true")]
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

#[derive(Tabled)]
struct Entry {
    #[tabled(rename = "")]
    name: &'static str,
    total: String,
    used: String,
    free: String,
}

fn main() {
    let args = Args::parse();

    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));
    sys.refresh_memory();

    //               total        used        free      shared  buff/cache   available
    // Mem:           3.5G      186.9M        3.1G        9.1M      207.3M        3.2G
    // Swap:             0           0           0
    let mem_total = if args.human {
        format!("{}M", sys.total_memory() / 1_000_000)
    } else {
        sys.total_memory().to_string()
    };
    let mem_used = if args.human {
        format!("{}M", sys.used_memory() / 1_000_000)
    } else {
        sys.used_memory().to_string()
    };
    let mem_free = if args.human {
        format!("{}M", sys.free_memory() / 1_000_000)
    } else {
        sys.free_memory().to_string()
    };
    let swap_total = if args.human {
        format!("{}M", sys.total_swap() / 1_000_000)
    } else {
        sys.total_swap().to_string()
    };
    let swap_used = if args.human {
        format!("{}M", sys.used_swap() / 1_000_000)
    } else {
        sys.used_swap().to_string()
    };
    let swap_free = if args.human {
        format!("{}M", sys.free_swap() / 1_000_000)
    } else {
        sys.free_swap().to_string()
    };
    let data = vec![
        Entry {
            name: "Mem",
            total: mem_total,
            used: mem_used,
            free: mem_free,
        },
        Entry {
            name: "Swap",
            total: swap_total,
            used: swap_used,
            free: swap_free,
        },
    ];

    let mut table = Table::new(data);
    table
        .with(Style::sharp())
        .modify(Rows::first(), Alignment::center())
        .modify(Rows::new(1..), Alignment::right());
    println!("{table}\n");

    let mut rnd = rand::thread_rng();
    println!("❄️  > {}", quotes::QUOTES.choose(&mut rnd).unwrap());
}
