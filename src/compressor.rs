use std::{fs::File, io::Cursor};

use las::{Reader, Writer, point::Format};

pub struct Compressor<'a> {
    pub output_file: &'a str,
    pub input_file: &'a str,
}

impl<'a> Compressor<'a> {
    pub fn new(output_file: &'a str, input_file: &'a str) -> Self {
        Self {
            output_file,
            input_file,
        }
    }

    pub fn compress(&self) -> () {
        println!("compressing .las to .laz");
        let mut builder = las::Builder::from((1, 4));
        println!("the builder is {}", builder.file_source_id);
        let header = builder.into_header().unwrap();

        let output_file = match File::create(self.output_file) {
            Err(err) => {
                eprint!("{err}");
                return;
            }
            Ok(it) => it
        };

        let write = std::io::BufWriter::new(output_file);
        let result = Writer::new(write, header);
        println!("result is {}", result.is_ok());
        // if cfg!(feature = "laz") {
        //     println!("here");
        //     assert!(result.is_ok());
        // } else {
        //     assert!(result.is_err());
        // }
    }
}

