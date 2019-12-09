use bio::io::fasta;
use std::fs::File;
use std::path::Path;
use std::fmt;


/// Parse the filename and produce an iterator over the records
pub fn file_parse(input_filename: &str) -> fasta::Records<File> {
    let path_string = String::from(input_filename);
    let path = Path::new(&path_string);
    let reader = fasta::Reader::from_file(path).unwrap();

    return reader.records()
}

#[derive(Debug, PartialEq)]
pub struct GCResult {
    rosalind_id: String,
    gc_content: f32,
}

impl fmt::Display for GCResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.rosalind_id, self.gc_content)
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

pub fn gc<T>(records: fasta::Records<T>) -> GCResult
    where T: std::io::Read {

    let mut max_gc_content: f32 = 0.0;
    let mut max_id: String = String::new();

    for record in records {
        let r = record.unwrap();
        let r_gc = gc_content(&r);
        if r_gc > max_gc_content {
            max_gc_content = r_gc;
            max_id = String::from(r.id())
        }
    }
    return GCResult{
        rosalind_id: max_id,
        gc_content: max_gc_content,
    }
}
