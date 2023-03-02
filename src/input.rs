// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use std::fs;
use std::io::{self, BufRead, Read};

pub trait Input {
    fn name(&self) -> String;
    fn to_bytes(&self) -> Result<Box<dyn Iterator<Item = io::Result<u8>>>>;
    fn to_lines(&self) -> Result<Box<dyn Iterator<Item = io::Result<String>>>>;
}

pub fn open(path: &str) -> Result<Box<dyn Input>> {
    if path == "-" {
        return Ok(Box::new(BufferedInput::new(
            "standard input",
            StdinOpener::new(),
        )));
    }

    Ok(Box::new(BufferedInput::new(path, FileOpener::new(path))))
}

struct BufferedInput<O: Opener> {
    name: String,
    opener: O,
}

impl<O: Opener> BufferedInput<O> {
    pub fn new(name: &str, opener: O) -> Self {
        BufferedInput {
            name: String::from(name),
            opener,
        }
    }
}

impl<O: Opener> Input for BufferedInput<O> {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn to_bytes(&self) -> Result<Box<dyn Iterator<Item = io::Result<u8>>>> {
        Ok(Box::new(self.opener.open()?.bytes()))
    }
    fn to_lines(&self) -> Result<Box<dyn Iterator<Item = io::Result<String>>>> {
        Ok(Box::new(LineReader::new(self.opener.open()?)))
    }
}

struct LineReader<B: BufRead> {
    buffer: B,
}

impl<B: BufRead> LineReader<B> {
    pub fn new(buffer: B) -> Self {
        LineReader { buffer }
    }
}

impl<B: BufRead> Iterator for LineReader<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut s = String::new();

        match self.buffer.read_line(&mut s) {
            Ok(0) => None,
            Ok(_) => Some(Ok(s)),
            Err(e) => Some(Err(e)),
        }
    }
}

trait Opener {
    type Output: 'static + BufRead;

    fn open(&self) -> Result<Self::Output>;
}

struct FileOpener {
    path: String,
}

impl FileOpener {
    pub fn new(path: &str) -> Self {
        FileOpener {
            path: String::from(path),
        }
    }
}

impl Opener for FileOpener {
    type Output = io::BufReader<fs::File>;
    fn open(&self) -> Result<Self::Output> {
        if fs::metadata(&self.path)?.is_dir() {
            return Err(anyhow!("Is a Directory"));
        }

        Ok(io::BufReader::new(fs::File::open(&self.path)?))
    }
}

struct StdinOpener {}

impl StdinOpener {
    pub fn new() -> Self {
        StdinOpener {}
    }
}

impl Opener for StdinOpener {
    type Output = io::BufReader<io::Stdin>;
    fn open(&self) -> Result<Self::Output> {
        Ok(io::BufReader::new(io::stdin()))
    }
}
