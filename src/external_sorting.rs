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

    pub fn as_chunks(self) -> Self {
        // Remove chunks folder if it exists
        if let Ok(true) = fs::exists(Self::CHUNKS_FOLDER) {
            fs::remove_dir_all(Self::CHUNKS_FOLDER)
                .expect(format!("Failed to remove: {}", Self::CHUNKS_FOLDER).as_str());
        }

        // Create chunks folder
        fs::create_dir(Self::CHUNKS_FOLDER)
            .expect(format!("Failed to create folder: {}", Self::CHUNKS_FOLDER).as_str());

        // Open huge file
        let mut f = File::open(self.0).expect(format!("Failed to open file: {}", self.0).as_str());

        // Create buffer
        let mut buf: [u8; 65535] = [0; Self::BUFFER_CAPACITY];

        // Set the iterator limit and move along it
        for i in 0..u16::MAX {
            // Read a piece of file into the buffer
            let bytes_written = f
                .read(&mut buf)
                .expect(format!("Failed to read file: {}", self.0).as_str());

            let chunk = &buf[0..bytes_written];
            let chunk_path = self.get_path_to_chunk_by_idx(i as usize);

            // Writing chunk to new file
            fs::write(chunk_path, chunk)
                .expect(format!("Failed to write file: {}", self.0).as_str());

            // Terminate the loop if the file is finished
            if bytes_written < Self::BUFFER_CAPACITY {
                break;
            }
        }

        self
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

    pub fn sort_each_chunk(self) -> Self {
        // Listen paths from chunks folder
        let paths = fs::read_dir(Self::CHUNKS_FOLDER)
            .expect(format!("Can't read folder: {}", Self::CHUNKS_FOLDER).as_str());

        paths.for_each(|path| {
            // Get path to the file
            let path = path.expect("Cannot find path to file").path();
            let path = path.display();

            // Open file
            let mut f = OpenOptions::new()
                .read(true)
                .open(path.to_string())
                .expect(format!("Cannot open file: {}", path).as_str());

            // Read file to buffer
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("Failed to read chunk");

            // Convert buffer to vector
            let mut buf = buf
                .trim()
                .split(" ")
                .map(|x| {
                    x.parse::<i64>()
                        .expect(format!("Can't convert {} to integer", x).as_str())
                })
                .collect::<Vec<_>>();

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
                .open(path.to_string())
                .expect(format!("Failed to recreate file: {}", path).as_str())
                .write_all(sorted.as_bytes())
                .expect("Failed to write sorted array to file");

            f.flush().expect("Failed to flush");
        });

        self
    }
}
