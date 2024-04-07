use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use tabled::{
    settings::{object::Rows, Alignment, Style},
    Table, Tabled,
};

#[derive(Copy, Clone)]
pub enum Mode {
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
    Human,
}

fn format_byte(value: u64) -> String {
    format!("{}", value)
}

fn format_kilobyte(value: u64) -> String {
    let value = value as f64;
    format!("{:.1}K", value / 1_000.0)
}

fn format_megabyte(value: u64) -> String {
    let value = value as f64;
    format!("{:.1}M", value / 1_000_000.0)
}

fn format_gigabyte(value: u64) -> String {
    let value = value as f64;
    format!("{:.1}G", value / 1_000_000_000.0)
}

fn format_human(value: u64) -> String {
    match value {
        v if v < 1_000 => format_byte(value),
        v if v >= 1_000 && v < 1_000_000 => format_kilobyte(value),
        v if v >= 1_000_000 && v < 1_000_000_000 => format_megabyte(value),
        _ => format_gigabyte(value),
    }
}

fn format(mode: Mode, value: u64) -> String {
    match mode {
        Mode::Byte => format_byte(value),
        Mode::KiloByte => format_kilobyte(value),
        Mode::MegaByte => format_megabyte(value),
        Mode::GigaByte => format_gigabyte(value),
        Mode::Human => format_human(value),
    }
}

#[derive(Tabled)]
struct Entry {
    #[tabled(rename = "")]
    name: &'static str,
    total: String,
    used: String,
    free: String,
}

pub fn free(mode: Mode) {
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));
    sys.refresh_memory();

    //               total        used        free      shared  buff/cache   available
    // Mem:           3.5G      186.9M        3.1G        9.1M      207.3M        3.2G
    // Swap:             0           0           0
    let data = vec![
        Entry {
            name: "Mem",
            total: format(mode, sys.total_memory()),
            used: format(mode, sys.used_memory()),
            free: format(mode, sys.free_memory()),
        },
        Entry {
            name: "Swap",
            total: format(mode, sys.total_swap()),
            used: format(mode, sys.used_swap()),
            free: format(mode, sys.free_swap()),
        },
    ];

    let mut table = Table::new(data);
    table
        .with(Style::sharp())
        .modify(Rows::first(), Alignment::center())
        .modify(Rows::new(1..), Alignment::right());

    println!("{table}");
}
