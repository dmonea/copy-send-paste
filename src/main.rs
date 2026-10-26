extern crate copypasta;
use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// "rcv" or "send" -> the mode in which the program is supposed to run
    #[arg(short, long)]
    mode: String,

    /// "clip" or "file" + "file_path" -> what's the target for the mode: send/receive to/from clipboard or to/from a file
    #[arg(short, long)]
    target: String,

    /// Optional file_path argument
    #[arg(short, long)]
    file_path: Option<String>,
}

fn main() {
    //     let mut ctx = ClipboardContext::new().unwrap();
    //
    //     let msg = "Hello, world!";
    //     ctx.set_contents(msg.to_owned()).unwrap();
    //
    //     let content = ctx.get_contents().unwrap();
    //
    //     println!("{}", content);
    //
}
