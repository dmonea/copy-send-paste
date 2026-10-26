extern crate copypasta;
use clap::{Arg, ArgAction, Command, Parser};
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
    let matches = Command::new("csp")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .action(ArgAction::Set)
                .value_name("MODE")
                .help("Chooses how the program should work: in receive or send mode."),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .action(ArgAction::Set)
                .value_name("TARGET")
                .help("What 'csp' should be working with: the clipboard or a file."),
        )
        .arg(
            Arg::new("file_path")
                .short('p')
                .long("file_path")
                .action(ArgAction::Set)
                .value_name("FILE_PATH")
                .help("The file path where 'csp' will be reading data from or writing data to, depending on where the command is executed."),
        ).get_matches();

    println!("{:?}", matches);
}
