pub trait FromBitSlice {
    fn reduce_bit_slice(slice: &[u8]) -> Self; 
}

impl FromBitSlice for u16 {
    fn reduce_bit_slice(slice: &[u8]) -> Self {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| Self::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl FromBitSlice for u32 {
    fn reduce_bit_slice(slice: &[u8]) -> Self {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| Self::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl FromBitSlice for i32 {
    fn reduce_bit_slice(slice: &[u8]) -> Self {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| Self::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

///
/// Round the value up to the nearest multiple of 4
/// See: https://stackoverflow.com/a/9194117
/// 
pub fn round_to_next_multiple_of_4(value: i32) -> usize {
    ((value + 4 - 1) & -4) as usize
}

pub mod file {
    use std::fs::File;
    use std::io::{Read, BufReader, Write};

    pub fn get_file_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
        //Open the file
        let fs = File::open(path)?;
    
        //Read file to buffer
        let mut br = BufReader::new(fs);
        let mut buffer = Vec::new();
    
        br.read_to_end(&mut buffer)?;
    
        Ok(buffer)
    }

    pub fn write_file_bytes(path: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(bytes)
    }
}