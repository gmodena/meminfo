= meminfo

A process that scans its own memory address space and prints basic info.

Inspired by Listing 6.16 of [Rust in Action](https://github.com/rust-in-action/code/tree/1st-edition/ch6/ch6-meminfo-win),
but implemented on Linux with the [procfs](https://docs.rs/procfs/latest/procfs/) crate.

