#!/usr/bin/env cargo -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
clap = { version = "4.5", features = ["derive"] }
nom = "8"
reqwest = { version = "0.12", features=["blocking"] }
---
/// Taken from https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2025/rust/scripts/get-aoc-input.rs
use clap::{error::ErrorKind, CommandFactory, Parser};
use nom::{
    bytes::complete::tag, character::complete, sequence::preceded, IResult, Parser as NomParser,
};
use reqwest::{blocking::Client, header::COOKIE};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// day is expected to be formatted as
    /// `day-01` to match all other commands in
    /// the repo
    #[clap(short, long)]
    day: String,
    /// a way to pass in the justfile directory
    /// so that we're always in the root without
    /// doing any shenanigans
    #[clap(long)]
    cwd: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32).parse(input)
}

fn main() -> Result<(), reqwest::Error> {
    let session = std::env::var("AOC_SESSION").expect("should have a session token set");
    let args = Args::parse();
    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("day `{}` must be formatted as `day-01`", args.day),
        )
        .exit()
    };

    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    println!("sending to `{}`", url);

    let client = Client::new();
    let input_data = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = args.cwd.join(&args.day).join(filename);
        let mut file = File::create(&file_path).expect("should be able to create a file");

        file.write_all(input_data.as_bytes())
            .expect("should be able to write to input file");
        println!("wrote {}", file_path.display());
    }

    Ok(())
}
