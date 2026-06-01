use rayon::prelude::*;
use std::fs::{File, OpenOptions};

use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Arc;

const CHUNK_SIZE: usize = 1024 * 1024;

fn main() {
    println!("Multi threaded file copier");

    let src = prompt("enter source file path: ");
    let dest = prompt("enter destination file path: ");
    match copy_file_in_parallel(&src, &dest) {
        Ok(_) => println!("file copied successfully"),
        Err(e) => println!("error: {}", e),
    }
}

fn copy_file_in_parallel(src_path: &str, dest_path: &str) -> std::io::Result<()> {
    let src_file = File::open(src_path)?;
    let src_metadata = src_file.metadata()?;
    let file_size = src_metadata.len() as usize;

    let chunk_count = (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE;
    println!("File size {} bytes | Chunks: {} ", file_size, chunk_count);

    let src_arc = Arc::new(src_path.to_string());
    let dest_file = File::create(dest_path)?;
    dest_file.set_len(file_size as u64)?; // prealocate size

    (0..chunk_count).into_par_iter().try_for_each(|i| {
        let offset = i * CHUNK_SIZE;
        let mut buffer = vec![0u8; CHUNK_SIZE.min(file_size - offset)];
        let mut src = File::open(&*src_arc)?;
        src.seek(SeekFrom::Start(offset as u64))?;
        src.read_exact(&mut buffer)?;

        let mut dest = OpenOptions::new().write(true).open(dest_path)?;
        dest.seek(SeekFrom::Start(offset as u64))?;
        dest.write_all(&buffer)?;
        Ok::<(), std::io::Error>(())
    })?;
    Ok(())
}

fn prompt(msg: &str) -> String {
    let mut input = String::new();
    println!("{}", msg);
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
