use image::{ImageBuffer, Rgba};
use std::{
    collections::HashMap,
    error::Error,
    path::Path,
};

use crate::{
    utils::{find_steam_game, get_attribute_value_node},
};

const HEADER_SIZE: usize = 12;
const RGBA_FORMAT: i32 = 4;
const PIXEL_SIZE: usize = 4;

#[derive(Default, Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    format: i32,
    header: Vec<u8>,
    data: Vec<u8>,
    mode: String,
}

impl Texture {
    pub fn new(path: &Path, create: bool, width: Option<usize>, height: Option<usize>) -> Self {
        if create {
            log::info!("Creating cim for {}", &path.display());
            Self::init_cim(width.unwrap(), height.unwrap())
        } else {
            log::info!("Importing cim from {}", &path.display());
            match Self::import_cim(path) {
                Ok(texture) => texture,
                Err(err) => {
                    log::error!(
                        "Failed to create a new texture from {} \nDue to {}",
                        path.display(),
                        err
                    );
                    Texture::default()
                }
            }
        }
    }

    fn init_cim(width: usize, height: usize) -> Texture {
        let mut header = vec![0u8; HEADER_SIZE];
        let data = vec![0u8; width * height * PIXEL_SIZE];
        let format = 0;

        let width_bytes = width.to_be_bytes();
        let height_bytes = height.to_be_bytes();
        let rgba_format_bytes = RGBA_FORMAT.to_be_bytes();

        header[0..4].copy_from_slice(&width_bytes);
        header[4..8].copy_from_slice(&height_bytes);
        header[8..12].copy_from_slice(&rgba_format_bytes);

        Texture {
            width,
            height,
            format,
            header,
            data,
            mode: "".to_string(),
        }
    }

    fn import_cim(path: &Path) -> Result<Texture, Box<dyn Error>> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut buffer)?;

        let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(buffer));

        let mut decompressed_data = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut decompressed_data)?;

        let header = decompressed_data[..HEADER_SIZE].to_vec();
        let width = i32::from_be_bytes(header[0..4].try_into().unwrap()) as usize;
        let height = i32::from_be_bytes(header[4..8].try_into().unwrap()) as usize;
        let format = i32::from_be_bytes(header[8..12].try_into().unwrap());

        let expected_size = width * height * PIXEL_SIZE;

        let mode = if format == RGBA_FORMAT {
            "RGBA".to_string()
        } else {
            return Err(format!("ERROR: Unknown CIM format: {}", format).into());
        };

        let data = decompressed_data[HEADER_SIZE..].to_vec();

        if data.len() != expected_size {
            return Err(format!(
                "ERROR: Wrong size {}: {} vs {}",
                path.display(),
                data.len(),
                expected_size
            )
            .into());
        }

        Ok(Texture {
            width,
            height,
            format,
            header,
            data,
            mode,
        })
    }

    pub fn export_png(
        &self,
        path: &Path,
        x: usize,
        y: usize,
        width: Option<usize>,
        height: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        let width = width.unwrap_or(self.width);
        let height = height.unwrap_or(self.height);

        let mut rows = Vec::new();

        for row in y..(y + height) {
            let start = (x + row * self.width) * PIXEL_SIZE;
            let end = start + (width * PIXEL_SIZE);

            rows.push(self.data[start..end].to_vec());
        }

        let flat_data: Vec<u8> = rows.into_iter().flatten().collect();
        let img: ImageBuffer<Rgba<u8>, _> =
            image::ImageBuffer::from_raw(width as u32, height as u32, flat_data)
                .ok_or("Failed to create image buffer from data".to_string())?;
        img.save(path)?;

        Ok(())
    }
}

/// Single Threaded.
/// TODO: Convert to parallel, and offload img.save from export_png into a worker thread
pub fn explode() -> Result<(), Box<dyn Error>> {
    let core_path = find_steam_game()?.join("mods").join("spacehaven");

    let texture_path = core_path.to_path_buf().join("library").join("textures");
    log::info!(
        "Exploding textures from \n{}",
        &texture_path.as_path().display()
    );

    let xml_string = std::fs::read_to_string(&texture_path)?;
    let textures = amxml::dom::new_document(&xml_string)?;
    let regions = textures.get_nodeset("//re[@n]")?;
    let mut cims = HashMap::new();
    let mut region_count = 0;
    let mut page_count = 0;
    log::info!("Found {} texture regions", regions.len());

    for region in &regions {
        let name: String = get_attribute_value_node(&region, "n")?;

        let x = get_attribute_value_node(&region, "x")?;
        let y = get_attribute_value_node(&region, "y")?;
        let w = get_attribute_value_node(&region, "w")?;
        let h = get_attribute_value_node(&region, "h")?;

        let page: String = get_attribute_value_node(&region, "t")?;

        if !cims.contains_key(&page) {
            let cim_filename = format!("{}.cim", page);
            log::info!("Unpacking textures {}", cim_filename);
            cims.insert(
                page.clone(),
                Texture::new(
                    &core_path.join("library").join(&cim_filename),
                    false,
                    None,
                    None,
                ),
            );
        }

        let texture_exploded_path = core_path
            .join("library")
            .join("textures.exploded")
            .join(&page);
        if let Err(_) = std::fs::create_dir_all(&texture_exploded_path) {
            log::error!("What happened?");
        }

        let png_filename = core_path
            .join("library")
            .join("textures.exploded")
            .join(&page)
            .join(format!("{}.png", name));

        cims.get(&page)
            .unwrap()
            .export_png(png_filename.as_path(), x, y, Some(w), Some(h))?;

        region_count += 1;
    }

    for (page, _) in &cims {
        let path = core_path
            .join("library")
            .join("textures.exploded")
            .join(format!("{}.png", page));
        cims.get(page).ok_or("Could not find page")?.export_png(
            path.as_path(),
            0,
            0,
            None,
            None,
        )?;

        page_count += 1;
    }

    log::info!(
        "Created {}/{} regions, and {}/{} cims",
        region_count,
        regions.len(),
        page_count,
        cims.len()
    );

    Ok(())
}
