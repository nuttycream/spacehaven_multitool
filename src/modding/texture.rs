use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use image::{image_dimensions, GenericImageView, ImageDecoder, ImageEncoder};
use std::{error::Error, fs::File, io::BufReader};

const HEADER_SIZE: usize = 12;
const RGBA_FORMAT: i32 = 1;
const PIXEL_SIZE: usize = 4;

pub struct Texture {
    width: usize,
    height: usize,
    header: Vec<u8>,
    data: Vec<u8>,
    mode: String,
}

impl Texture {
    fn new(path: &str, create: bool, width: usize, height: usize) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn init_cim(width: usize, height: usize) {}

    fn import_cim(&mut self, path: &std::path::Path) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut buffer)?;

        // Decompress the data using ZlibDecoder
        let mut decoder = ZlibDecoder::new(std::io::Cursor::new(buffer));
        let mut decompressed_data = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut decompressed_data)?;

        //..
        let md5_hash = format!(
            "{:x}",
            md5::Digest(decompressed_data.clone().try_into().unwrap())
        );
        log::info!("{}", md5_hash);

        let mut cursor = std::io::Cursor::new(&decompressed_data);
        let mut header = [0; HEADER_SIZE];
        std::io::Read::read_exact(&mut cursor, &mut header)?;

        let width = cursor.read_i32::<BigEndian>()? as usize;
        let height = cursor.read_i32::<BigEndian>()? as usize;
        let format = cursor.read_i32::<BigEndian>()? as usize;

        if format == RGBA_FORMAT.try_into().unwrap() {
            self.mode = "RGBA".to_string();
        }

        let expected_size = (width * height * PIXEL_SIZE) as usize;
        self.data.resize(expected_size, 0);
        std::io::Read::read_exact(&mut cursor, &mut self.data)?;

        if self.data.len() != expected_size {}

        self.width = width;
        self.height = height;
        self.header = header.to_vec();
        Ok(())
    }

    fn pack_png(
        &mut self,
        path: &std::path::Path,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<(), Box<dyn Error>> {
        let img = image::open(&std::path::Path::new(path)).unwrap();
        let (width, height) = img.dimensions();
        let binding = img.into_rgba8();
        let rows = binding.as_raw();
        let mut row_index = 0;

        for row in rows.chunks(4 * width as usize) {
            let start = (x + (row_index + y) * self.width) * PIXEL_SIZE;
            let end = start + width as usize * PIXEL_SIZE;

            self.data[start..end].copy_from_slice(&row);
            row_index += 1;
        }

        Ok(())
    }

    fn export_cim(&self, path: &std::path::Path) -> Result<(), Box<dyn Error>> {
        let mut export = Vec::new();
        export.extend_from_slice(&self.header);
        export.extend_from_slice(&self.data);
        let md5_hash = format!("{:x}", md5::Digest(export.clone().try_into().unwrap()));
        log::info!("{}", md5_hash);
        Ok(())
    }

    fn export_png(
        &self,
        path: &std::path::Path,
        x: usize,
        y: usize,
        width: Option<usize>,
        height: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        let img_width = self.width;
        let img_height = self.height;

        let export_width = width.unwrap_or(img_width);
        let export_height = height.unwrap_or(img_height);

        let mut rows = Vec::new();
        for row in y..(y + export_height) {
            let start = (x + (row * img_width)) * PIXEL_SIZE;
            let end = start + (export_width * PIXEL_SIZE);

            rows.extend_from_slice(&self.data[start..end]);
        }

        let file = File::create(path)?;
        let encoder = image::codecs::png::PngEncoder::new(file);
        encoder.write_image(
            &rows,
            export_width as u32,
            export_height as u32,
            image::ColorType::Rgba8,
        )?;

        Ok(())
    }

    
}
