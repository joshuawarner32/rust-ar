use std::iter::{Iterator, IntoIterator};
use std::str;

// From https://en.wikipedia.org/wiki/Ar_(Unix):
// Offset	Length	Name	Format
// 0	16	File name	ASCII
// 16	12	File modification timestamp	Decimal
// 28	6	Owner ID	Decimal
// 34	6	Group ID	Decimal
// 40	8	File mode	Octal
// 48	10	File size in bytes	Decimal
// 58	2	File magic	0x60 0x0A

pub struct Reader<'a> {
    data: &'a [u8]
}

pub struct ReaderIter<'a> {
    data: &'a [u8]
}

// pub struct Builder {
//     data: Vec<u8>
// }

pub struct File<'a> {
    inner_name_len: Option<usize>,
    data: &'a [u8]
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Result<Reader<'a>, ()> {
        if data.len() < 8 || &data[0..8] != b"!<arch>\x0a" {
            Err(())
        } else {
            Ok(Reader { data: data })
        }
    }
}

fn read_decimal(data: &[u8]) -> u64 {
    let mut val = 0;
    for b in data {
        match *b {
            b'0'...b'9' => {
                val *= 10;
                val += (*b - b'0') as u64;
            }
            _ => break
        }
    }
    val
}

fn read_octal(data: &[u8]) -> u64 {
    let mut val = 0;
    for b in data {
        match *b {
            b'0'...b'7' => {
                val *= 8;
                val += (*b - b'0') as u64;
            }
            _ => break
        }
    }
    val
}

impl<'a> Iterator for ReaderIter<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() >= 60 {
            let len = read_decimal(&self.data[48..58]) as usize;

            let data = &self.data[..60+len];

            if (len & 1) == 0 && self.data.len() >= 60+len {
                self.data = &self.data[60+len..];
            } else if self.data.len() >= 60+len+1 {
                self.data = &self.data[60+len+1..];
            }

            Some(File::new(data))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for Reader<'a> {
    type Item = File<'a>;
    type IntoIter = ReaderIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIter {
            data: &self.data[8..]
        }
    }
}

impl<'a> File<'a> {
    fn new(data: &'a [u8]) -> File<'a> {
        File {
            inner_name_len:
                if &data[0..3] == b"#1/" {
                    Some(read_decimal(&data[3..16]) as usize)
                } else {
                    None
                },
            data: data
        }
    }

    pub fn name_u8(&self) -> &'a [u8] {
        match self.inner_name_len {
            Some(len) => &self.data[60..60+len],
            None => {
                let mut len = 0;
                while len < 16 && self.data[len] != b'/' {
                    len += 1;
                }

                &self.data[..len]
            }
        }
    }

    pub fn name(&self) -> Option<&str> {
        str::from_utf8(self.name_u8()).ok()
    }

    pub fn contents(&self) -> &'a [u8] {
        &self.data[60+self.inner_name_len.unwrap_or(0)..]
    }

    pub fn modified_timestamp(&self) -> u64 {
        read_decimal(&self.data[16..16+12])
    }

    pub fn owner_id(&self) -> u32 {
        read_decimal(&self.data[28..28+6]) as u32
    }

    pub fn group_id(&self) -> u32 {
        read_decimal(&self.data[34..34+6]) as u32
    }

    pub fn file_mode(&self) -> u32 {
        read_octal(&self.data[40..40+8]) as u32
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
