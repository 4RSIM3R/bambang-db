use std::{
    fs::File,
    io::{self, Read},
    vec,
};

use byteorder::{BigEndian, ReadBytesExt};
use tabled::{builder::Builder, settings::Style};

const SQLITE_HEADER_SIZE: usize = 100;
const SQLITE_SCHEMA_TABLE: &str = "sqlite_schema";

pub struct Bambang {
    pub file: File,
    pub page_size: u16,
    pub file_format_write_version: u8,
    pub file_format_read_version: u8,
    pub reserved_space: u8,
    pub max_embedded_payload_fraction: u8,
    pub min_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8,
    pub change_counter: u32,
    pub number_of_pages: u32,
    pub root_page_main_table: u32,
    pub root_page_schema: u32,
    pub root_page_freelist: u32,
    pub schema_format_number: u32,
    pub default_page_cache_size: u32,
    pub largest_btree_leaf_page_number: u32,
    pub text_encoding: String,
    pub user_version: u32,
    pub incremental_vacuum_mode: u32,
    pub application_id: u32,
    pub version_valid_for: u32,
    pub sqlite_version_number: u32,
}

impl Bambang {
    pub fn new(path: &str) -> io::Result<Bambang> {
        let mut file = File::open(path)?;

        let mut header_bytes = [0u8; 100];

        file.read_exact(&mut header_bytes)?;

        let mut cursor = io::Cursor::new(header_bytes);

        let mut header_string_bytes = [0u8; 16];

        cursor.read_exact(&mut header_string_bytes)?;

        let header_string = String::from_utf8(header_string_bytes.to_vec()).unwrap();

        println!("{}", header_string);

        if header_string != "SQLite format 3\0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header"));
        }

        // read 2 bytes 16..18
        let page_size = cursor.read_u16::<BigEndian>()?;
        // read 1 bytes 18..19
        let file_format_write_version = cursor.read_u8()?;
        let file_format_read_version = cursor.read_u8()?;
        let reserved_space = cursor.read_u8()?;
        let max_embedded_payload_fraction = cursor.read_u8()?;
        let min_embedded_payload_fraction = cursor.read_u8()?;
        let leaf_payload_fraction = cursor.read_u8()?;
        let change_counter = cursor.read_u32::<BigEndian>()?;
        let number_of_pages = cursor.read_u32::<BigEndian>()?;
        let root_page_main_table = cursor.read_u32::<BigEndian>()?;
        let root_page_schema = cursor.read_u32::<BigEndian>()?;
        let root_page_freelist = cursor.read_u32::<BigEndian>()?;
        let schema_format_number = cursor.read_u32::<BigEndian>()?;
        let default_page_cache_size = cursor.read_u32::<BigEndian>()?;
        let largest_btree_leaf_page_number = cursor.read_u32::<BigEndian>()?;

        // 17. Text Encoding (4 bytes)
        let text_encoding = cursor.read_u32::<BigEndian>()?;
        let text_encoding = text_encoding.to_string();

        let user_version = cursor.read_u32::<BigEndian>()?;
        let incremental_vacuum_mode = cursor.read_u32::<BigEndian>()?;
        let application_id = cursor.read_u32::<BigEndian>()?;
        let mut reserved_space_2 = [0u8; 4];
        cursor.read_exact(&mut reserved_space_2)?;
        let version_valid_for = cursor.read_u32::<BigEndian>()?;
        let sqlite_version_number = cursor.read_u32::<BigEndian>()?;

        Ok(Bambang {
            file: file,
            page_size: page_size,
            file_format_write_version: file_format_write_version,
            file_format_read_version: file_format_read_version,
            reserved_space: reserved_space,
            max_embedded_payload_fraction: max_embedded_payload_fraction,
            min_embedded_payload_fraction: min_embedded_payload_fraction,
            leaf_payload_fraction: leaf_payload_fraction,
            change_counter: change_counter,
            number_of_pages: number_of_pages,
            root_page_main_table: root_page_main_table,
            root_page_schema: root_page_schema,
            root_page_freelist: root_page_freelist,
            schema_format_number: schema_format_number,
            default_page_cache_size: default_page_cache_size,
            largest_btree_leaf_page_number: largest_btree_leaf_page_number,
            text_encoding: text_encoding,
            user_version: user_version,
            incremental_vacuum_mode: incremental_vacuum_mode,
            application_id: application_id,
            version_valid_for: version_valid_for,
            sqlite_version_number: sqlite_version_number,
        })
    }

    pub fn print_header(&self) {
        let mut builder = Builder::new();

        builder.push_record(vec!["Field", "Value"]);

        // Add rows for each field (excluding the `file` field)
        builder.push_record(vec!["page_size", &self.page_size.to_string()]);
        builder.push_record(vec![
            "file_format_write_version",
            &self.file_format_write_version.to_string(),
        ]);
        builder.push_record(vec![
            "file_format_read_version",
            &self.file_format_read_version.to_string(),
        ]);
        builder.push_record(vec!["reserved_space", &self.reserved_space.to_string()]);
        builder.push_record(vec![
            "max_embedded_payload_fraction",
            &self.max_embedded_payload_fraction.to_string(),
        ]);
        builder.push_record(vec![
            "min_embedded_payload_fraction",
            &self.min_embedded_payload_fraction.to_string(),
        ]);
        builder.push_record(vec![
            "leaf_payload_fraction",
            &self.leaf_payload_fraction.to_string(),
        ]);
        builder.push_record(vec!["change_counter", &self.change_counter.to_string()]);
        builder.push_record(vec!["number_of_pages", &self.number_of_pages.to_string()]);
        builder.push_record(vec![
            "root_page_main_table",
            &self.root_page_main_table.to_string(),
        ]);
        builder.push_record(vec!["root_page_schema", &self.root_page_schema.to_string()]);
        builder.push_record(vec![
            "root_page_freelist",
            &self.root_page_freelist.to_string(),
        ]);
        builder.push_record(vec![
            "schema_format_number",
            &self.schema_format_number.to_string(),
        ]);
        builder.push_record(vec![
            "default_page_cache_size",
            &self.default_page_cache_size.to_string(),
        ]);
        builder.push_record(vec![
            "largest_btree_leaf_page_number",
            &self.largest_btree_leaf_page_number.to_string(),
        ]);
        builder.push_record(vec!["text_encoding", &self.text_encoding.to_string()]);
        builder.push_record(vec!["user_version", &self.user_version.to_string()]);
        builder.push_record(vec![
            "incremental_vacuum_mode",
            &self.incremental_vacuum_mode.to_string(),
        ]);
        builder.push_record(vec!["application_id", &self.application_id.to_string()]);
        builder.push_record(vec![
            "version_valid_for",
            &self.version_valid_for.to_string(),
        ]);
        builder.push_record(vec![
            "sqlite_version_number",
            &self.sqlite_version_number.to_string(),
        ]);


        let mut binding = builder.build();
        let table = binding.with(Style::psql());

        // Print the table
        println!("{}", table);
    }
}
