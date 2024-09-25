use std::path::PathBuf;
use clap::Parser;

pub const VERSION: &str = "0.1.0";

#[derive(Parser, Debug, Clone)]
#[command(
    name = "nemo",
    author = "sharkLoc",
    version = VERSION,
    about = "A tool for quality overview of long-read sequencing data",
    long_about = None,
    next_line_help = false,
    disable_help_flag = false,
    disable_version_flag = false,
    propagate_version = true,
    before_help = None)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
    \nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/nemo.git\
    \n\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"nemo --help\" for more information")]
pub struct Args {
    /// Input long reads sequence data, or read data from stdin.
    #[arg(global = true, help_heading = Some("Global Arguments"), value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Specify the output HTML report file name
    #[arg(short='r',long="html", value_name="str", default_value_t = String::from("report.html"))]
    pub html: String,
}


pub fn get_cmd(args: Args) -> String {
    let cmd = args;
    let mut opt = vec![String::from("./nemo")];
    
    let html = cmd.html;
    opt.push(String::from("--html"));
    opt.push(html);

    let input = if let Some(v) = cmd.input { v.to_str().unwrap().to_string() } else { String::from("-") };
    opt.push(input);

    opt.join("  ")
}