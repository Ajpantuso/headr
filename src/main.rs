// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use headr::options;
use headr::Command;
use std::process;

fn main() {
    let opts = options::Options::parse();

    if let Err(e) = Command::from(&opts).run() {
        eprintln!("{e}");

        process::exit(1);
    }
}
