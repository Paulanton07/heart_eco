use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::utils::error::{AppError, AppResult};

/// Read a file line by line into a vector
pub fn read_lines<P>(filename: P) -> AppResult<Vec<String>> 
where P: AsRef<Path> {
    let file = File::open(filename)
        .map_err(|e| AppError::IoError(e))?;
    
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

/// Read a file line by line and process each line with a callback function
pub fn process_lines<P, F>(filename: P, mut callback: F) -> AppResult<()> 
where 
    P: AsRef<Path>,
    F: FnMut(&str) -> AppResult<()>,
{
    let file = File::open(filename)
        .map_err(|e| AppError::IoError(e))?;
    
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        callback(&line)?;
    }
    
    Ok(())
}

/// Check if a file exists
pub fn file_exists<P>(path: P) -> bool 
where P: AsRef<Path> {
    path.as_ref().exists() && path.as_ref().is_file()
}

/// Write a string to a file, creating the file if it doesn't exist or overwriting if it does
pub fn write_string_to_file<P>(path: P, content: &str) -> AppResult<()> 
where P: AsRef<Path> {
    let mut file = File::create(path)
        .map_err(|e| AppError::IoError(e))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| AppError::IoError(e))?;
    
    Ok(())
}

/// Write lines to a file, creating the file if it doesn't exist or overwriting if it does
pub fn write_lines<P, I, S>(path: P, lines: I) -> AppResult<()> 
where 
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut file = File::create(path)
        .map_err(|e| AppError::IoError(e))?;
    
    for line in lines {
        file.write_all(line.as_ref().as_bytes())
            .map_err(|e| AppError::IoError(e))?;
        file.write_all(b"\n")
            .map_err(|e| AppError::IoError(e))?;
    }
    
    Ok(())
}

/// Append a string to a file, creating the file if it doesn't exist
pub fn append_to_file<P>(path: P, content: &str) -> AppResult<()> 
where P: AsRef<Path> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| AppError::IoError(e))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| AppError::IoError(e))?;
    
    Ok(())
}
