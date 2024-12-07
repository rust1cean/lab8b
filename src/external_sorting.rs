use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

type PathToFile = &'static str;

pub struct ProcessHugeFile(pub PathToFile);

impl ProcessHugeFile {
    const CHUNKS_FOLDER: &'static str = "chunks";
    const BUFFER_CAPACITY: usize = u16::MAX as usize;

    fn get_path_to_chunk_by_idx(&self, idx: usize) -> String {
        format!("{}/chunk_{idx}.txt", Self::CHUNKS_FOLDER)
    }

    pub fn as_chunks(self) -> std::io::Result<Self> {
        // Remove chunks folder if it exists
        if let Ok(true) = fs::exists(Self::CHUNKS_FOLDER) {
            fs::remove_dir_all(Self::CHUNKS_FOLDER)?;
        }

        // Create chunks folder
        fs::create_dir(Self::CHUNKS_FOLDER)?;

        // Open huge file
        let mut f = File::open(self.0)?;

        // Create buffer
        let mut buf: [u8; 65535] = [0; Self::BUFFER_CAPACITY];

        // Set the iterator limit and move along it
        for i in 0..u16::MAX {
            // Read a piece of file into the buffer
            let bytes_written = f.read(&mut buf)?;

            let chunk = &buf[0..bytes_written];
            let chunk_path = self.get_path_to_chunk_by_idx(i as usize);

            // Writing chunk to new file
            fs::write(chunk_path, chunk)?;

            // Terminate the loop if the file is finished
            if bytes_written < Self::BUFFER_CAPACITY {
                break;
            }
        }

        Ok(self)
    }

    fn internal_sort<T: PartialOrd>(&self, array: &mut Vec<T>) {
        for i in 0..array.len() {
            let mut swapped = false;

            for j in 0..array.len() - i - 1 {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }

    pub fn sort_each_chunk(self) -> Result<Self, Box<dyn std::error::Error>> {
        // Listen paths from chunks folder
        let paths = fs::read_dir(Self::CHUNKS_FOLDER)?;

        for path in paths {
            // Get path to the file
            let path = path?.path();
            let path = path.display();

            // Open file
            let mut f = OpenOptions::new().read(true).open(path.to_string())?;

            // Read file to buffer
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;

            // Convert buffer to vector
            let mut buf = buf
                .trim()
                .split(" ")
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?;

            // Sort array
            self.internal_sort(&mut buf);

            // Convert vector to string
            let sorted = buf
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            // Recreate the file and write the sorted array into it
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(path.to_string())?
                .write_all(sorted.as_bytes())?;
        }

        Ok(self)
    }
}
