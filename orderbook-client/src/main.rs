use orderbook_common::*;

use anyhow::{anyhow, Context, Result};
use futures::SinkExt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio::io::AsyncWrite;
use tokio::net::UnixStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::SymmetricallyFramed;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let commands = parse_input()?;
    let stream = UnixStream::connect(SOCKET).await?;
    send(stream, &commands).await?;
    Ok(())
}

async fn send<T: AsyncWrite + Unpin>(io: T, commands: &[Command]) -> Result<()> {
    let transport = FramedWrite::new(io, LengthDelimitedCodec::new());
    let mut framed = SymmetricallyFramed::new(transport, SymmetricalJson::default());
    for command in commands {
        framed.send(command).await?;
    }
    Ok(())
}

// Primitive csv reader. Rewrite with regexps or parser combinators.
fn parse_input() -> Result<Vec<Command>> {
    let mut commands = Vec::new();
    let input_file = std::env::args().nth(1).context("Missing input file")?;
    for line in read_lines(input_file)? {
        let line = line?;
        let words = line.split(',').collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }
        // The very first character of non-empty line determines the command.
        let command = match words[0].chars().next() {
            Some('#') | None => continue,
            Some('N') => parse_new_command(&words),
            Some('C') => parse_cancel_command(&words),
            Some('F') => parse_flush_command(),
            _ => return Err(anyhow!("Unecognized command")),
        }?;
        commands.push(command);
    }
    Ok(commands)
}

fn parse_new_command(words: &[&str]) -> Result<Command> {
    let user_id = words[1].trim().parse()?;
    let symbol = words[2].trim().to_string();
    let price = words[3].trim().parse()?;
    let quantity = words[4].trim().parse()?;
    let side = words[5].trim().chars().next().unwrap();
    let user_order_id = words[6].trim().parse()?;
    Ok(Command::New(NewOrder {
        user_id,
        user_order_id,
        symbol,
        price,
        quantity,
        side,
    }))
}

fn parse_cancel_command(words: &[&str]) -> Result<Command> {
    let user_id = words[1].trim().parse()?;
    let user_order_id = words[2].trim().parse()?;
    Ok(Command::Cancel(CancelOrder {
        user_id,
        user_order_id,
    }))
}

fn parse_flush_command() -> Result<Command> {
    Ok(Command::Flush)
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
