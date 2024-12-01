use std::error::Error;
use std::{fs, io};

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if !path.contains(".db") {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid file type",
        )));
    }

    let data = fs::read(path)?;
    Ok(data)
}

pub fn write_file(path: &str) -> io::Result<()> {
    if !path.contains(".db") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid file type",
        ));
    }

    const HEADER_PREFIX: &[u8] = b"SQLite format 3\0"; // 16 bytes

    let mut file_data: Vec<u8> = Vec::new();

    // Add the SQLite header
    file_data.extend_from_slice(HEADER_PREFIX);

    // Page size: 4096 bytes (0x1000)
    file_data.extend_from_slice(&0x1000u16.to_be_bytes()); // Offset 16-17

    // File format write version and read version
    file_data.push(1); // Write version (Offset 18)
    file_data.push(1); // Read version (Offset 19)

    // Reserved bytes per page
    file_data.push(0); // Offset 20

    // Maximum embedded payload fraction
    file_data.push(64); // Offset 21

    // Minimum embedded payload fraction
    file_data.push(32); // Offset 22

    // Leaf payload fraction
    file_data.push(32); // Offset 23

    // File change counter
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 24-27

    // Reserved space (must be zero)
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 28-31

    // Database size in pages (we have at least one page)
    file_data.extend_from_slice(&1u32.to_be_bytes()); // Offset 32-35

    // First freelist trunk page
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 36-39

    // Total number of freelist pages
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 40-43

    // Schema cookie (used for schema versioning)
    file_data.extend_from_slice(&1u32.to_be_bytes()); // Offset 44-47

    // Schema format number (1 is the original format)
    file_data.extend_from_slice(&1u32.to_be_bytes()); // Offset 48-51

    // Default page cache size
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 52-55

    // Largest root b-tree page number
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 56-59

    // Text encoding (1: UTF-8)
    file_data.extend_from_slice(&1u32.to_be_bytes()); // Offset 60-63

    // User version
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 64-67

    // Incremental vacuum mode
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 68-71

    // Application ID
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 72-75

    // Reserved for expansion (20 bytes)
    file_data.extend_from_slice(&[0u8; 20]); // Offset 76-95

    // Version-valid-for number (set to file change counter)
    file_data.extend_from_slice(&0u32.to_be_bytes()); // Offset 96-99

    // SQLite version number (e.g., 3037000 for version 3.37.0)
    file_data.extend_from_slice(&3037000u32.to_be_bytes()); // Offset 100-103

    // Ensure the header is exactly 100 bytes
    if file_data.len() < 100 {
        file_data.resize(100, 0);
    }

    // Add an empty 4KB page to the file
    file_data.resize(4096, 0);

    // Write to the file
    fs::write(path, &file_data)
}
