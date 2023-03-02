// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use input::Input;
use std::io::{self, Write};

mod input;
pub mod options;

pub struct Command<'a> {
    options: &'a options::Options,
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<()> {
        let mut stdout = io::stdout().lock();
        let mut stderr = io::stderr().lock();

        for (idx, file) in self.options.files.iter().enumerate() {
            match input::open(file) {
                Ok(input) => {
                    if self.options.files.len() > 1 {
                        writeln!(stdout, "{}", format_header(&input.name()))?;
                    }

                    if self.options.bytes.is_some() {
                        self.write_bytes(input, &mut stdout, &mut stderr)?
                    } else {
                        self.write_lines(input, &mut stdout, &mut stderr)?
                    }
                }
                Err(e) => writeln!(
                    &mut stderr,
                    "{}: cannot open '{}' for reading: {}",
                    clap::crate_name!(),
                    file,
                    e
                )?,
            }

            if idx < self.options.files.len() - 1 {
                writeln!(stdout)?;
            }
        }

        Ok(())
    }
    fn write_bytes(
        &self,
        input: Box<dyn Input>,
        mut out: impl Write,
        mut err: impl Write,
    ) -> Result<()> {
        match input.to_bytes() {
            Ok(bytes) => match self.options.bytes.as_ref().unwrap() {
                options::SignedUsize::Positive(n) => {
                    let buf: Vec<u8> = bytes.take(*n).flatten().collect();

                    write!(out, "{}", String::from_utf8_lossy(&buf))?;
                }
                options::SignedUsize::Negative(n) => {
                    let buf: Vec<u8> = bytes
                        .flatten()
                        .collect::<Vec<u8>>()
                        .iter()
                        .rev()
                        .skip(*n)
                        .rev()
                        .cloned()
                        .collect();

                    write!(out, "{}", String::from_utf8_lossy(&buf))?;
                }
            },
            Err(e) => writeln!(
                err,
                "{}: error reading '{}': {}",
                clap::crate_name!(),
                input.name(),
                e
            )?,
        }

        Ok(())
    }
    fn write_lines(
        &self,
        input: Box<dyn Input>,
        mut out: impl Write,
        mut err: impl Write,
    ) -> Result<()> {
        match input.to_lines() {
            Ok(lines) => match self.options.lines {
                options::SignedUsize::Positive(n) => {
                    for line in lines.take(n).flatten() {
                        write!(out, "{line}")?;
                    }
                }
                options::SignedUsize::Negative(n) => {
                    for line in lines
                        .flatten()
                        .collect::<Vec<String>>()
                        .iter()
                        .rev()
                        .skip(n)
                        .rev()
                    {
                        write!(out, "{line}")?;
                    }
                }
            },
            Err(e) => writeln!(
                err,
                "{}: error reading '{}': {}",
                clap::crate_name!(),
                input.name(),
                e
            )?,
        }

        Ok(())
    }
}

impl<'a> From<&'a options::Options> for Command<'a> {
    fn from(options: &'a options::Options) -> Self {
        Command { options }
    }
}

fn format_header(name: &str) -> String {
    format!("==> {} <==", name)
}
