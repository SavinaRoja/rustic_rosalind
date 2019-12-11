use bio::io::fasta;
use std::fs::File;
use std::path::Path;
use std::fmt;
use super::Solvable;

pub type Problem = GCResult;

#[derive(Debug, PartialEq)]
pub struct GCResult {
    rosalind_id: String,
    gc_content: f32,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.rosalind_id, self.gc_content)
    }
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { rosalind_id: String::new(), gc_content: 0.0 }
    }
}

impl Solvable for Problem {
    type Parameters = fasta::Records<File>;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let path_string = String::from(input_filename);
        let path = Path::new(&path_string);
        let reader = fasta::Reader::from_file(path).unwrap();

        return reader.records()
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        for record in params {
            let r = record.unwrap();
            let r_gc = gc_content(&r);
            if r_gc > self.gc_content {
                self.gc_content = r_gc;
                self.rosalind_id = String::from(r.id())
            }
        }
    }
}

fn gc_content(record: &fasta::Record) -> f32 {
    let mut g_or_c_count: u32 = 0;
    let seq = record.seq();
    for val in seq {
        if *val == 67 as u8 || *val == 71 as u8 {
            g_or_c_count += 1
        }
    }
    return g_or_c_count as f32/ seq.len() as f32 * 100 as f32
}
