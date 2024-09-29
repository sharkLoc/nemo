mod cmd;
mod utils;
mod process;
mod loger;
mod error;
mod report;

use clap::Parser;
use log::info;
use cmd::{get_cmd, VERSION};
use error::NemoError;
use process::statfq;
use report::summary;
use std::time::Instant;


fn main() -> Result<(), NemoError>{
    let cmd = cmd::Args::parse();
    let cmd_txt = get_cmd(cmd.clone());
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;

    let start = Instant::now();
    info!("nemo version: {}",VERSION);
    
    let (content, length_hash, gc_hash, qual_relative_vec)= statfq(cmd.input, Some(&cmd.json), cmd.compression_level)?;
    summary(content, length_hash, gc_hash, qual_relative_vec, &cmd.html, cmd_txt)?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
