#![allow(dead_code)]

use infrastructure::{command_line::CommandLine};

use anyhow::Result;

mod application;
mod domain;
mod infrastructure;
mod interface;

struct Program {
    command_line: CommandLine,
}

impl Program {
    fn new() -> Program {
        let command_line = CommandLine::new();
        Program {
            command_line
        }
    }

    async fn run(&self) -> Result<()> {
        self.command_line.start().await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let program = Program::new();
    if let Err(e) = program.run().await {
        eprintln!("Error: {}", e);
    }
    Ok(())
}