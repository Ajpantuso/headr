// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use clap::builder;
use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[arg(default_value = "-")]
    pub files: Vec<String>,
    #[arg(short = 'n', long = "lines", default_value = "10")]
    #[arg(value_parser = builder::ValueParser::new(parse_signed_usize))]
    pub lines: SignedUsize,
    #[arg(short = 'c', long = "bytes", conflicts_with = "lines")]
    #[arg(value_parser = builder::ValueParser::new(parse_signed_usize))]
    pub bytes: Option<SignedUsize>,
}

#[derive(Clone)]
pub enum SignedUsize {
    Positive(usize),
    Negative(usize),
}

fn parse_signed_usize(maybe_num: &str) -> Result<SignedUsize> {
    let (negative, maybe_num) = match maybe_num.strip_prefix('-') {
        Some(n) => (true, n),
        None => (false, maybe_num),
    };

    match maybe_num.parse() {
        Ok(num) if num > 0 => Ok(if negative {
            SignedUsize::Negative(num)
        } else {
            SignedUsize::Positive(num)
        }),
        _ => Err(anyhow!("invalid zero-value")),
    }
}
