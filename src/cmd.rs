use std::path::PathBuf;
use clap::{value_parser, ArgAction, Parser};

pub const VERSION: &str = "0.1.0";

#[derive(Parser, Debug, Clone)]
#[command(
    name = "nemo",
    author = "sharkLoc",
    version = VERSION,
    about = "A tool for quality overview of long-read sequencing data",
    long_about = None,
    next_line_help = false,
    disable_help_flag = true,
    disable_version_flag = true,
    // propagate_version = false,
    before_help = None)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
    \nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/nemo.git\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"nemo --help\" for more information")]
pub struct Args {
    /// Input long reads sequence data, or read data from stdin.
    #[arg(global = true, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Specify the output HTML report file name
    #[arg(short='r',long="html", value_name="FILE", default_value_t = String::from("report.html"))]
    pub html: String,

    /// If file name specified, write log message to this file, or write to stderr
    #[arg(long = "log", global = true, value_name = "FILE")]
    pub logfile: Option<String>,

    /// Set compression level 1 (compress faster) - 9 (compress better) for gzip/bzip2/xz output file
    #[arg(long = "compress-level", default_value_t = 6, global = true, value_parser = value_parser!(u32).range(1..=9), 
        value_name = "INT"
    )]
    pub compression_level: u32,

    /// Control verbosity of logging, [-v: Error, -vv: Warn, -vvv: Info, -vvvv: Debug, -vvvvv: Trace, defalut: Debug]
    #[arg(short = 'v', long = "verbosity", action = ArgAction::Count, global = true,
        default_value_t = 4
    )]
    pub verbose: u8,

    /// Be quiet and do not show any extra information
    #[arg(short = 'q', long = "quiet", global= true)]
    pub quiet: bool,

    /// prints help information
    #[arg(short = 'h', long, action = ArgAction::Help, global= true)]
    pub help: Option<bool>,

    /// prints version information
    #[arg(short = 'V', long, action = ArgAction::Version, global= true)]
    pub version: Option<bool>,
}


pub fn get_cmd(args: Args) -> String {
    let cmd = args;
    let mut opt = vec![String::from("./nemo")];
    
    let html = cmd.html;
    opt.push(String::from("--html"));
    opt.push(html);

    let input = if let Some(v) = cmd.input { v.to_str().unwrap().to_string() } else { String::from("-") };
    opt.push(input);

    if cmd.quiet { 
        opt.push(String::from("--quit")); 
    } else {
        let verbose = match cmd.verbose {
            1 => String::from("-v"),
            2 => String::from("-vv"),
            3 => String::from("-vvv"),
            4 => String::from("-vvvv"),
            5 => String::from("-vvvvv"),
            _ => String::from("")
          };
          opt.push(verbose);
    }

    opt.join("  ")
}