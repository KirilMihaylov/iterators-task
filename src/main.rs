#![forbid(unsafe_code, clippy::pedantic)]
#![deny(warnings)]

use std::env::{args_os, ArgsOs};
use std::ffi::OsString;
use std::iter::Skip;

use iterators_task::iterators_main;

fn main() -> Result<(), &'static str> {
    let a: i32;

    let path: OsString;

    {
        // Skip first argument;
        // First argument is executable's path.
        let mut args: Skip<ArgsOs> = args_os().skip(1);

        a = if let Some(argument) = args.next() {
            if let Ok(a) = argument.to_string_lossy().parse() {
                a
            } else {
                return Err("Passed argument for 'A' is not an UTF-8 representable integer!");
            }
        } else {
            return Err("No 'A' passed as argument!");
        };

        if a < 0 {
            return Err("Passed argument for 'A' is below zero!");
        }

        path = args.next().ok_or("No path passed as second argument!")?;
    }

    println!("Lines: {:?}", iterators_main(a, &path)?);

    Ok(())
}
