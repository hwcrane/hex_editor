use crate::byte::Byte;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct HexFile {
    filepath: String,
    data: Vec<Byte>,
    pub length: usize,
}

impl HexFile {
    pub fn load_from_file(filepath: String) -> io::Result<HexFile> {
        let mut f = File::open(&filepath)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        let data: Vec<Byte> = buffer.iter().map(|b| Byte::new(*b)).collect();
        let length = data.len();

        Ok(HexFile { filepath, data , length})
    }

    pub fn get_data(&self) -> &Vec<Byte> {
        &self.data
    }

    pub fn print(&self) {
    }
}


