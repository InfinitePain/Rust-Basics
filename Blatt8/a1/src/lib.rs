use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const FIRSTNAME_LEN: usize = 40;
const LASTNAME_LEN: usize = 40;
const ADDRESS_LEN: usize = 240;

#[derive(Debug, Clone)]
pub struct Record {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub age: i32,
    pub hash: u64,
}

impl Record {
    pub fn new(first_name: String, last_name: String, address: String, age: i32) -> Self {
        let mut record = Record {
            first_name,
            last_name,
            address,
            age,
            hash: 0,
        };
        record.update_hash();
        record
    }

    fn update_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.first_name.hash(&mut hasher);
        self.last_name.hash(&mut hasher);
        self.address.hash(&mut hasher);
        self.age.hash(&mut hasher);
        self.hash = hasher.finish();
    }
}

#[derive(Debug, Default)]
pub struct RecordTable {
    records: Vec<Record>,
    source: Option<String>,
}

impl RecordTable {
    pub fn from_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path.as_ref())?;
        let mut records = Vec::new();

        loop {
            match Self::read_record(&mut file) {
                Ok(record) => records.push(record),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }

        Ok(RecordTable {
            records,
            source: Some(path.as_ref().to_string_lossy().into_owned()),
        })
    }

    fn read_record(file: &mut File) -> io::Result<Record> {
        let mut read_fixed_str = |len: usize| -> io::Result<String> {
            let mut buffer = vec![0u8; len];
            file.read_exact(&mut buffer)?;
            let nul_pos = buffer.iter().position(|&b| b == 0).unwrap_or(len);
            Ok(String::from_utf8_lossy(&buffer[..nul_pos]).into_owned())
        };

        let first_name = read_fixed_str(FIRSTNAME_LEN)?;
        let last_name = read_fixed_str(LASTNAME_LEN)?;
        let address = read_fixed_str(ADDRESS_LEN)?;

        let mut age_buffer = [0u8; 4];
        file.read_exact(&mut age_buffer)?;
        let age = i32::from_le_bytes(age_buffer);

        let mut record = Record::new(first_name, last_name, address, age);
        record.update_hash();
        Ok(record)
    }

    pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for record in &self.records {
            self.write_record(&mut file, record)?;
        }

        Ok(())
    }

    fn write_record(&self, file: &mut File, record: &Record) -> io::Result<()> {
        let write_fixed_str = |file: &mut File, s: &str, len: usize| -> io::Result<()> {
            let mut buffer = vec![0u8; len];
            let bytes = s.as_bytes();
            buffer[..bytes.len()].copy_from_slice(bytes);
            file.write_all(&buffer)
        };

        write_fixed_str(file, &record.first_name, FIRSTNAME_LEN)?;
        write_fixed_str(file, &record.last_name, LASTNAME_LEN)?;
        write_fixed_str(file, &record.address, ADDRESS_LEN)?;
        file.write_all(&record.age.to_le_bytes())?;

        Ok(())
    }

    pub fn save_text<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for record in &self.records {
            writeln!(
                file,
                "{},{},{},{}",
                record.first_name, record.last_name, record.address, record.age
            )?;
        }

        Ok(())
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }

    pub fn get_source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    pub fn modify_by_hash(
        &mut self,
        hash: u64,
        first_name: Option<String>,
        last_name: Option<String>,
        address: Option<String>,
        age: Option<i32>,
    ) -> bool {
        if let Some(record) = self.records.iter_mut().find(|r| r.hash == hash) {
            if let Some(name) = first_name {
                record.first_name = name;
            }
            if let Some(name) = last_name {
                record.last_name = name;
            }
            if let Some(addr) = address {
                record.address = addr;
            }
            if let Some(new_age) = age {
                record.age = new_age;
            }
            record.update_hash();
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct RecordManager {
    tables: HashMap<String, RecordTable>,
}

impl RecordManager {
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let table = RecordTable::from_binary_file(&path)?;
        self.tables.insert(path_str, table);
        Ok(())
    }

    pub fn remove_table<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.tables
            .remove(&path.as_ref().to_string_lossy().into_owned())
            .is_some()
    }

    pub fn list_open_files(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    pub fn get_all_records(&self) -> Vec<(&String, &Vec<Record>)> {
        self.tables
            .iter()
            .map(|(path, table)| (path, table.get_records()))
            .collect()
    }

    pub fn save_table<P: AsRef<Path>>(&self, source: P, target: P, binary: bool) -> io::Result<()> {
        if let Some(table) = self
            .tables
            .get(&source.as_ref().to_string_lossy().into_owned())
        {
            if binary {
                table.save_binary(target)
            } else {
                table.save_text(target)
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Source table not found",
            ))
        }
    }

    pub fn modify_record(
        &mut self,
        source: &str,
        hash: u64,
        first_name: Option<String>,
        last_name: Option<String>,
        address: Option<String>,
        age: Option<i32>,
    ) -> bool {
        if let Some(table) = self.tables.get_mut(source) {
            table.modify_by_hash(hash, first_name, last_name, address, age)
        } else {
            false
        }
    }
}
