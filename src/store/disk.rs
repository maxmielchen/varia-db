use core::panic;
use std::{path::Path, fs::{File, OpenOptions}, io::{Error, Write, ErrorKind, Read, Seek, SeekFrom}};

use super::Value;

pub struct Disk {
    pub buf_stream: File
}

impl Disk {

    pub fn new(path: &Path) -> Result<Self, Error> {
        let file = Self::initilize_signed_file(path)?;
        Ok(Self {
            buf_stream: file
        })
    }

    fn initilize_signed_file(path: &Path) -> Result<File, Error> {
        let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(path)?;

        file.seek(SeekFrom::Start(0))?;
        if file.metadata()?.len() == 0 {
            file.write(&Self::signed_buffer())?;
        }

        file.seek(SeekFrom::Start(0))?;
        let mut buf: [u8; 16] = [0; 16];
        file.read(&mut buf)?;
        Self::is_signed_buffer_valid(buf)?;
    
        Ok(file)
    }

    fn signed_buffer() -> [u8; 16] {
        let mut buf: [u8; 16] = [0; 16];
        buf[0] = 'v' as u8;
        buf[1] = 'a' as u8;
        buf[2] = 'r' as u8;
        buf[3] = 'i' as u8;
        buf[4] = 'a' as u8;

        buf[5] = '-' as u8;
        buf[6] = '-' as u8;
        buf[7] = '-' as u8;
        buf[8] = '-' as u8;
        buf[9] = '-' as u8;
        buf[10] = '-' as u8;
        buf[11] = '-' as u8;
        buf[12] = '-' as u8;
        buf[13] = '-' as u8;

        buf[14] = 'd' as u8;
        buf[15] = 'b' as u8;
        buf
    }

    fn is_signed_buffer_valid(buf: [u8; 16]) -> Result<(), Error> {
        if buf[0] != 'v' as u8 ||
        buf[1] != 'a' as u8 ||
        buf[2] != 'r' as u8 ||
        buf[3] != 'i' as u8 ||
        buf[4] != 'a' as u8 ||
        buf[5] != '-' as u8 ||
        buf[6] != '-' as u8 ||
        buf[7] != '-' as u8 ||
        buf[8] != '-' as u8 ||
        buf[9] != '-' as u8 ||
        buf[10] != '-' as u8 ||
        buf[11] != '-' as u8 ||
        buf[12] != '-' as u8 ||
        buf[13] != '-' as u8 ||
        buf[14] != 'd' as u8 ||
        buf[15] != 'b' as u8 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid signature"));
        }
        Ok(())
    }

    fn read_sign(&mut self) -> Result<(), Error> {
        self.buf_stream.seek(SeekFrom::Start(0))?;
        let mut signed_buf: [u8; 16] = [0; 16];
        self.buf_stream.read(&mut signed_buf)?;
        Self::is_signed_buffer_valid(signed_buf)?;
        Ok(())
    }

    fn entry_frame(key: &String, value: &Value) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(
            0 as u8
        );

        let key_buf = postcard::to_allocvec(&key);
        if let Err(_) = key_buf {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid key"));
        }
        let key_buf = key_buf.unwrap();

        let value_buf = postcard::to_allocvec(&value);
        if let Err(_) = value_buf {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid value"));
        }
        let value_buf = value_buf.unwrap();

        let key_len = key_buf.len() as u128;
        let value_len = value_buf.len() as u128;

        if let None = key_len.checked_add(value_len) {
            return Err(Error::new(ErrorKind::InvalidData, "Key and value too big"));
        }

        buf.extend_from_slice(&key_len.to_be_bytes());
        buf.extend_from_slice(&value_len.to_be_bytes());

        buf.extend_from_slice(&key_buf);
        buf.extend_from_slice(&value_buf);

        Ok(buf)
    }

    fn gap_frame(len: u128) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        if len == 0 {
            panic!("Gap frame len is 0");
        }

        if len < 17 {
            
            buf.push(
                len as u8
            );

            for _ in 0..len-1 {
                buf.push(
                    0 as u8
                );
            }
      
            buf
        } else {
            buf.push(
                17 as u8
            );
    
            buf.extend_from_slice(&len.to_be_bytes());

            for _ in 0..len-17 {
                buf.push(
                    0 as u8
                );
            }
    
            buf
        }
    }

    fn entry_frame_len(buf: [u8; 32]) -> Result<(u128, u128, u128), Error> {
        let mut len_buf_key: [u8; 16] = [0; 16];
        len_buf_key.copy_from_slice(&buf[0..16]);
        let mut len_buf_value: [u8; 16] = [0; 16];
        len_buf_value.copy_from_slice(&buf[16..32]);
        let len_key = u128::from_be_bytes(len_buf_key);
        let len_value = u128::from_be_bytes(len_buf_value);
        let frame_len = len_key + len_value + 33;

        Ok((frame_len, len_key, len_value))
    }

    fn big_gap_frame_len(buf: [u8; 16]) -> Result<u128, Error> {
        let len = u128::from_be_bytes(buf);
        Ok(len)
    }

    fn positive_seek(&mut self, skip: usize) -> Result<(), Error> {
        let mut unskiped = skip;
        let directon_max: usize = 100000;
        while unskiped > 0 {
            if unskiped > directon_max {
                self.buf_stream.seek(SeekFrom::Current(directon_max as i64))?;
                unskiped -= directon_max;
            } else {
                self.buf_stream.seek(SeekFrom::Current(unskiped as i64))?;
                unskiped = 0;
            }
        }
        Ok(())
    }

    fn negative_seek(&mut self, skip: usize) -> Result<(), Error> {
        let mut unskiped = skip;
        let directon_max: usize = 100000;
        while unskiped > 0 {
            if unskiped > directon_max {
                self.buf_stream.seek(SeekFrom::Current(-(directon_max as i64)))?;
                unskiped -= directon_max;
            } else {
                self.buf_stream.seek(SeekFrom::Current(-(unskiped as i64)))?;
                unskiped = 0;
            }
        }
        Ok(())
    }

    fn seek_gap(&mut self, opt: u8) -> Result<(), Error> {
        if opt == 0 {
            let mut buf: [u8; 32] = [0; 32];
            self.buf_stream.read(&mut buf)?;
            let (_, key_len, value_len) = Self::entry_frame_len(buf).unwrap();

            self.positive_seek(key_len as usize)?;
            self.positive_seek(value_len as usize)?;

            return Ok(());
        }
        if opt == 17 {
            let mut buf: [u8; 16] = [0; 16];
            self.buf_stream.read(&mut buf)?;
            let frame_len = Self::big_gap_frame_len(buf)?;
  
            self.positive_seek(frame_len as usize)?;
            self.negative_seek(17)?;

            return Ok(());
        }
        if opt < 17 {
            let frame_len = opt;
            self.positive_seek(frame_len as usize)?;
            self.negative_seek(1)?;
            return Ok(());
        }
        return Ok(());
    }

    fn read_vec(&mut self, vec: &mut Vec<u8>, len: u128) -> Result<(), Error> {
        let mut unfill = len;
        while unfill > 0 {
            let mut  buf: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut buf)?;
            if bytes_read == 0 {
                panic!("Unexpected end of file");
            }
            vec.push(buf[0]);
            unfill -= 1;
        }
        Ok(())
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<(), Error> {

        if let Some(_) = self.get(key.clone())? {
            self.del(key.clone())?;
        }

        self.read_sign()?;

        let entry_buf: Vec<u8> = Self::entry_frame(&key, &value)?;

        loop {
            let mut opt: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut opt)?;

            if bytes_read == 0 {
                self.buf_stream.write(entry_buf.as_slice())?;
                break;
            }

            if opt[0] == 0 {
                let mut buf: [u8; 32] = [0; 32];
                self.buf_stream.read(&mut buf)?;
                let (_, key_len, value_len) = Self::entry_frame_len(buf)?;
                self.positive_seek(key_len as usize)?;
                self.positive_seek(value_len as usize)?;
                continue;
            }

            if opt[0] == 17 {
                let mut buf: [u8; 16] = [0; 16];
                self.buf_stream.read(&mut buf)?;
                let frame_len = Self::big_gap_frame_len(buf)?;
                if frame_len > entry_buf.len() as u128 {
                    self.negative_seek(17)?;
                    self.buf_stream.write(&Self::gap_frame(frame_len - entry_buf.len() as u128))?;
                    self.buf_stream.write(entry_buf.as_slice())?;
                    break;
                }
                if frame_len == entry_buf.len() as u128 {
                    self.negative_seek(17)?;
                    self.buf_stream.write(entry_buf.as_slice())?;
                    break;
                }
                self.positive_seek(frame_len as usize)?;
                self.negative_seek(17)?;
                continue;
            }

            if opt[0] < 17 {
                let frame_len = opt[0];
                self.positive_seek(frame_len as usize)?;
                self.negative_seek(1)?;
                continue;
            }
        }
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<Value>, Error> {
        self.read_sign()?;

        let key_buf = postcard::to_allocvec(&key);
        if let Err(_) = key_buf {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid key"));
        }
        let key_buf = key_buf.unwrap();

        loop {
            let mut opt: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut opt)?;

            if bytes_read == 0 {
                return Ok(None);
            }

            if opt[0] == 0 {
                let mut buf: [u8; 32] = [0; 32];
                self.buf_stream.read(&mut buf)?;
                let (_, key_len, value_len) = Self::entry_frame_len(buf)?;

                let mut key_buf_read: Vec<u8> = Vec::new();
                self.read_vec(&mut key_buf_read, key_len)?;

                if key_buf_read != key_buf {
                    self.positive_seek(value_len as usize)?;
                    continue;
                }

                let mut value_buf_read: Vec<u8> = Vec::new();
                self.read_vec(&mut value_buf_read, value_len)?;

                let value = postcard::from_bytes(&value_buf_read);
                if let Err(_) = value {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid value"));
                }
                let value: Value = value.unwrap();

                return Ok(Some(value));
            }

            self.seek_gap(opt[0])?;
        }
    }

    pub fn del(&mut self, key: String) -> Result<(), Error> {
        self.read_sign()?;

        let key_buf = postcard::to_allocvec(&key);
        if let Err(_) = key_buf {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid key"));
        }
        let key_buf = key_buf.unwrap();

        loop {
            let mut opt: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut opt)?;

            if bytes_read == 0 {
                return Ok(());
            }

            if opt[0] == 0 {
                let mut buf: [u8; 32] = [0; 32];
                self.buf_stream.read(&mut buf)?;
                let (frame_len, key_len, value_len) = Self::entry_frame_len(buf)?;

                let mut key_buf_read: Vec<u8> = Vec::new();

                self.read_vec(&mut key_buf_read, key_len)?;

                if key_buf_read != key_buf {
                    self.positive_seek(value_len as usize)?;
                    continue;
                }

                self.positive_seek(value_len as usize)?;
                self.negative_seek(frame_len as usize)?;

                self.buf_stream.write(&Self::gap_frame(frame_len))?;
                return Ok(());
            }
            
            self.seek_gap(opt[0])?;
        }
    }

    pub fn list(&mut self) -> Result<Vec<String>, Error> {
        self.read_sign()?;

        let mut keys: Vec<String> = Vec::new();

        loop {
            let mut opt: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut opt)?;
            if bytes_read == 0 {
                return Ok(keys);
            }
            if opt[0] == 0 {
                let mut buf: [u8; 32] = [0; 32];
                self.buf_stream.read(&mut buf)?;
                let (_, key_len, value_len) = Self::entry_frame_len(buf)?;

                let mut key_buf_read: Vec<u8> = Vec::new();
                self.read_vec(&mut key_buf_read, key_len)?;

                let key = postcard::from_bytes(&key_buf_read);
                if let Err(_) = key {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid key"));
                }
                let key: String = key.unwrap();

                keys.push(key);

                self.positive_seek(value_len as usize)?;
                continue;
            }

            self.seek_gap(opt[0])?;
        }
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.read_sign()?;

        self.buf_stream.set_len(0)?;

        self.buf_stream.seek(SeekFrom::Start(0))?;
        self.buf_stream.write(&Self::signed_buffer())?;

        Ok(())
    }

    pub fn len(&mut self) -> Result<usize, Error> {
        self.read_sign()?;
        
        let mut len: usize = 0;

        loop {
            let mut opt: [u8; 1] = [0; 1];
            let bytes_read = self.buf_stream.read(&mut opt)?;
            if bytes_read == 0 {
                return Ok(len);
            }
            if opt[0] == 0 {
                let mut buf: [u8; 32] = [0; 32];
                self.buf_stream.read(&mut buf)?;
                let (_, key_len, value_len) = Self::entry_frame_len(buf)?;

                len = len + 1;

                self.positive_seek(key_len as usize)?;
                self.positive_seek(value_len as usize)?;
                continue;
            }

            self.seek_gap(opt[0])?
        }
    }

    pub fn is_empty(&mut self) -> Result<bool, Error> {
        self.read_sign()?;
        if self.len()? == 0 {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn defrag(&mut self) -> Result<(), Error> {
        self.read_sign()?;

        Ok(())
    }
}