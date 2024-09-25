use std::{collections::HashMap, path::PathBuf};
use crate::{error::NemoError, utils::file_reader};
use bio::io::fastq::Reader;

#[derive(Debug)]
pub struct Rec {
    pub file_name: String,
    pub nt_a: usize,
    pub nt_g: usize,
    pub nt_t: usize,
    pub nt_c: usize,
    pub nt_n: usize,
    pub reads: usize,
    pub bases: usize,
    pub max_len: usize,
    pub min_len: usize,
    pub average_len: f64,
    pub gc_content: f64,
    pub less1k: usize,
    pub less2k: usize,
    pub less5k: usize,
    pub less10k: usize,
    pub less20k: usize,
    pub less50k: usize,
    pub less1k_r: f64,
    pub less2k_r: f64,
    pub less5k_r: f64,
    pub less10k_r: f64,
    pub less20k_r: f64,
    pub less50k_r: f64,
}

impl Rec {
    fn new() -> Self {
        Rec { file_name: "-".to_string(), nt_a: 0, nt_g: 0, nt_t: 0, nt_c: 0, nt_n: 0, reads: 0, bases: 0, max_len: 0, min_len: 0, average_len: 0., gc_content: 0., less1k: 0, less2k: 0, less5k: 0, less10k: 0, less20k: 0, less50k: 0, less1k_r: 0., less2k_r: 0., less5k_r: 0., less10k_r:0., less20k_r: 0., less50k_r: 0. }
    }

    fn update(&mut self) {
        self.average_len = self.bases as f64 / self.reads as f64;
        self.gc_content = (self.nt_g + self.nt_c) as f64 / self.bases as f64;
        self.less1k_r = self.less1k as f64 / self.reads as f64;
        self.less2k_r = self.less2k as f64 / self.reads as f64;
        self.less5k_r = self.less5k as f64 / self.reads as f64;
        self.less10k_r = self.less10k as f64 / self.reads as f64;
        self.less20k_r = self.less20k as f64 / self.reads as f64;
        self.less50k_r = self.less50k as f64 / self.reads as f64;
    }
}


pub fn statfq(
    file: Option<PathBuf>,
) -> Result<(Rec, HashMap<usize, usize>, HashMap<u64,u64>), NemoError> {
    
    let file_name = file.clone();
    let mut info = Rec::new();
    let mut min_len: Option<usize> = None;
    if let Some(name) = file_name {
        info.file_name = name.file_name().unwrap().to_str().unwrap().to_string();
    }
    let mut length_hash: HashMap<usize, usize> = HashMap::new();
    let mut gc_hash: HashMap<u64,u64> = HashMap::new();

    let reader = Reader::new(file_reader(file)?);
    for rec in reader.records().map_while(Result::ok) {
        let len = rec.seq().len();
        *length_hash.entry(len).or_insert(0usize) += 1;
        info.reads += 1;
        info.bases += len;

        if len > info.max_len {
            info.max_len = len;
        }
         
        if let Some(min) = min_len {
            if min > len { min_len = Some(len); }
        } else {
            min_len = Some(len);
        }

        if len < 1000 {info.less1k += 1; }
        if len < 2000 {info.less2k += 1; }
        if len < 5000 {info.less5k += 1; }
        if len < 10000 {info.less10k += 1; }
        if len < 20000 {info.less20k += 1; }
        if len < 50000 {info.less50k += 1; }
        
        let gc_count = rec
            .seq()
            .iter()
            .filter(|x| *x == &b'G'|| *x == &b'C')
            .count();
        let gc_ratio = (gc_count as f64 / len as f64 * 100.0).round() as u64;
        *gc_hash.entry(gc_ratio).or_insert(0u64) += 1;

        for (_idx, nt) in rec.seq().iter().enumerate() {
            match *nt {
                b'A' => info.nt_a += 1,
                b'T' => info.nt_t += 1,
                b'G' => info.nt_g += 1,
                b'C' => info.nt_c += 1,
                b'N' => info.nt_n += 1,
                _ => eprintln!("error base in read: {}",rec.id())
            }
        }
    }

    info.min_len = min_len.unwrap();
    info.update();

    Ok((info,length_hash, gc_hash))
}

