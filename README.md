# nemo
🦀 A tool for quality overview of long-read sequencing data

#### **This tool is under active development**

## install
##### setp1： install cargo first 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### step2:  on linux or windows
```bash
cargo install nemo
# or

git clone https://github.com/sharkLoc/nemo.git
cd nemo
cargo b --release
# mv target/release/nemo to anywhere you want 
```
##### install latest version

```bash
cargo install --git https://github.com/sharkLoc/nemo.git
```

## usage

```bash
nemo -- A tool for quality overview of long-read sequencing data

Version: 0.1.0
Authors: sharkLoc <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/nemo.git


Usage: nemo [OPTIONS] [FILE]

Options:
  -r, --html <str>  Specify the output HTML report file name [default: report.html]
  -h, --help        Print help
  -V, --version     Print version

Global Arguments:
  [FILE]  Input long reads sequence data, or read data from stdin

Use "nemo --help" for more information
```

#### ** any bugs please report issues **💖