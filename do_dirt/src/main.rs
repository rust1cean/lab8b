use std::fs::File;
use std::io::prelude::*;

const CHUNKS_COUNT: u64 = 2_u64.pow(25);
const CHUNK_SIZE: u32 = 2_u32.pow(20);

fn main() -> std::io::Result<()> {
    let mut file = File::create("huge.txt")?;

    for i in (0..CHUNKS_COUNT).step_by(CHUNK_SIZE as usize) {
        let random_numbers = (0..=CHUNK_SIZE)
            .map(|_| rand::random::<i16>().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        file.write(random_numbers.as_bytes())?;

        println!("{i} of {CHUNKS_COUNT}");
    }

    Ok(())
}
