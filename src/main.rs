use std::{
    fs::File,
    io::{BufReader, Write},
    ptr::copy_nonoverlapping,
};

use las::{Reader, point};

#[repr(C, packed)]
#[derive(Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PointWithColor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub intensity: u8,
}

pub struct LasConverter;

impl LasConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_to_bin(
        &self,
        input_path: &str,
        output_path: &str,
        include_color: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = match File::open(input_path) {
            Err(err) => {
                eprintln!("an error occurend while opening the file: {}", err);
                return Err(Box::new(err));
            }
            Ok(file) => file,
        };

        let mut reader = match Reader::new(file) {
            Err(err) => {
                println!("couldnt read the .las file, {err}");
                return Err(Box::new(err));
            }
            Ok(reader) => reader,
        };

        println!(".las file opened:");

        let output_file = File::create(output_path)?;
        let mut writer = std::io::BufWriter::new(output_file);

        let total_points = reader.header().number_of_points();
        let mut processed = 0;

        for (i, wrapped_point) in reader.points().enumerate() {
            let point = wrapped_point.unwrap();

            if i % 100000 == 0 {
                println!("Progression: {}/{}", i, total_points);
            }

            let simple_point = Point3D {
                x: point.x as f32,
                y: point.y as f32,
                z: point.z as f32,
            };

            self.write_point_3d(&mut writer, &simple_point)?;

            processed += 1;
        }

        writer.flush()?;
        println!("convertion done ! {} points processed", processed);
        Ok(())
    }

    fn write_point_3d(
        &self,
        writer: &mut std::io::BufWriter<File>,
        point: &Point3D,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                point as *const Point3D as *const u8,
                std::mem::size_of::<Point3D>(),
            )
        };
        writer.write_all(bytes)?;
        Ok(())
    }
}

fn main() {
    let converter = LasConverter::new();
    converter.convert_to_bin("./las/example.las", "./output.bin", false);
}
