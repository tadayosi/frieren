# Free for any platform - Display free and used memory with freeze ❄️

A `free` command alternative written in Rust. You can run it in any platform including macOS, Linux and Windows.

## Install

```console
cargo install frieren
```

## Usage

```console
$ frieren -h
Display free and used memory with freeze ❄️

Usage: frieren [OPTIONS]

Options:
  -H, --human    Display memory in human readable format
  -b, --byte     Display memory in bytes
  -k, --kilo     Display memory in kilobytes
  -m, --mega     Display memory in megabytes
  -g, --giga     Display memory in gigabytes
  -j, --ja       Display original Japanese quotes
  -h, --help     Print help
  -V, --version  Print version
```

Showing memory status:

```console
$ frieren
┌──────┬─────────────┬─────────────┬─────────────┬─────────────┐
│      │    total    │     used    │     free    │  available  │
├──────┼─────────────┼─────────────┼─────────────┼─────────────┤
│  Mem │       36.0G │       23.1G │       37.3M │        7.3G │
│ Swap │           0 │           0 │           0 │             │
└──────┴─────────────┴─────────────┴─────────────┴─────────────┘

❄️  > You are in front of a mage who has lived for more than one thousand years.
```

Showing memory status (in the original Japanese):

```console
$ frieren -j
┌──────┬─────────────┬─────────────┬─────────────┬─────────────┐
│      │    total    │     used    │     free    │  available  │
├──────┼─────────────┼─────────────┼─────────────┼─────────────┤
│  Mem │       36.0G │       22.9G │       56.8M │        8.0G │
│ Swap │           0 │           0 │           0 │             │
└──────┴─────────────┴─────────────┴─────────────┴─────────────┘

❄️  > そうだね、ヒンメルならそう言う。
```

## Build

```console
cargo build
```

## Release

```console
cargo build --release
```
