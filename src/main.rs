mod cmd;
mod utils;
mod process;
mod error;
mod report;
use clap::Parser;
use cmd::{get_cmd,Args};
use error::NemoError;
use process::statfq;
use report::summary;


fn main() -> Result<(), NemoError>{
    let cmd = Args::parse();
    let cmd_txt = get_cmd(cmd.clone());
    
    match cmd {
        Args {input, html, } => {
            let (content, length_hash, gc_hash )= statfq(input)?;
            summary(content, length_hash, gc_hash, &html, cmd_txt)?;
        }
    }

    Ok(())
}
