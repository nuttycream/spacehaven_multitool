use super::{get_patchable_xml_files, Mod, texture::Texture};

use amxml::dom::NodePtr;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
    path::PathBuf,
};

pub fn get_patchable_xml_files() -> Vec<String> {
    vec![
        "library/haven",
        "library/texts",
        "library/animations",
        "library/textures",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn get_patchable_cim_files() -> Vec<String> {
    let mut patchable_cim_files = Vec::with_capacity(26);
    for i in 0..26 {
        let filename = format!("library/{}.cim", i);
        patchable_cim_files.push(filename);
    }
    patchable_cim_files
}

#[derive(Default)]
pub struct CoreLibrary {
    node_dictionary: HashMap<String, NodePtr>,
    region_id_dictionary: HashMap<String, i32>,
    modded_textures: HashMap<String, Vec<String>>,
    custom_textures: HashMap<String, HashMap<String, HashMap<String, i64>>>,
}

fn build_library(
    location: &str,
    mod_dir: &str,
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut location_library = HashMap::new();

    let mod_path = Path::new(mod_dir);

    let location_path = mod_path.join(location);
    let location_files = match location_path.read_dir() {
        Ok(files) => files
            .map(|entry| entry.unwrap().path().display().to_string())
            .collect(),
        Err(_) => Vec::new(),
    };

    // we allow breaking down mod xml files into smaller pieces for readability
    for target in get_patchable_xml_files() {
        let target_in_location = target.replace("library", location);
        for mod_file in &location_files {
            if !mod_file.starts_with(&target_in_location) {
                continue;
            }
            let key = target.to_string();
            let entry = location_library.entry(key).or_insert_with(Vec::new);

            let mut file = File::open(mod_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            entry.push(contents);
        }

        let mut mod_file = mod_path.join(&target);
        // try again with the extension ?
        if !mod_file.exists() {
            mod_file.set_extension("xml");
            if !mod_file.exists() {
                continue;
            }
        }
        let entry = location_library.entry(target).or_insert_with(Vec::new);

        let mut file = File::open(mod_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        entry.push(contents);
    }

    Ok(location_library)
}

fn detect_textures(
    core_library: &CoreLibrary,
    mod_library: HashMap<&str, Vec<NodePtr>>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn add_texture(filename: PathBuf, core_library: &CoreLibrary) -> Result<bool, Box<dyn Error>> {
    let region_id = filename
        .file_stem()
        .ok_or("Failed to extract region id from filename")?
        .to_str()
        .ok_or("Failed to conver to string")?;

    let is_core_region = region_id.chars().all(|c| c.is_digit(10));
    let region_id_int = region_id.parse::<i32>();

    if is_core_region && region_id_int.is_ok() {
        let region_id_int = region_id_int.unwrap();

        let last_core_region_id: i32 = core_library.region_id_dictionary["_last_core_region_id"];

        if region_id_int <= last_core_region_id {
            return Ok(true);
        }
    }

    Ok(false)
}

fn core_path(core_path: &str, filename: &str) -> String {
    let filename = filename.replace('/', std::path::MAIN_SEPARATOR.to_string().as_str());
    format!("{}{}{}", core_path, std::path::MAIN_SEPARATOR, filename)
}

fn init_mods(
    corepath: &Path,
    active_mods: Vec<Mod>,
    mod_paths: Vec<&Path>,
) -> Result<(), Box<dyn Error>> {
    let mut core_library = CoreLibrary::default();

    let corepath_str = corepath
        .to_str()
        .ok_or("Failed to convert path to string.")?;

    for filename in get_patchable_xml_files() {
        let path = core_path(corepath_str, &filename);
        let mut file = File::open(&path).expect("Failed to open file");

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read file");

        let xml = amxml::dom::new_document(&file_content)?;
        core_library.node_dictionary.insert(filename, xml);
    }

    //find the last region in the texture file and remember its index
    //we will need this to add mod textures with consecutive indexes...
    let last_core_region_id_int = core_library.node_dictionary["library/textures"]
        .get_first_node("//re[@n][last()]")
        .ok_or("Could not find last region id for library/textures")?
        .attribute_value("n")
        .ok_or("Could not find attribute value n")?
        .parse::<i32>()?;

    core_library
        .region_id_dictionary
        .insert("_last_core_region_id".to_string(), last_core_region_id_int);
    core_library
        .region_id_dictionary
        .insert("_next_region_id".to_string(), last_core_region_id_int + 1);
    /*
    coreLibrary['_all_modded_textures'] = {}
    coreLibrary['_custom_textures_cim'] = {}
    */

    // Merge in modded files
    for mod_path in mod_paths {
        //modLibrary = buildLibrary('library', mod)
        let mod_library = build_library("library", mod_path.to_str().unwrap())?;
        //doMerges(coreLibrary, modLibrary, mod)
    }

    // patches after merge to avoid clobbers
    for active_mod in active_mods {
        //modPatchesLibrary = buildLibrary('patches', mod.path)
        let mod_library = build_library("patches", active_mod.path.to_str().unwrap())?;
        //doPatches(coreLibrary, modPatchesLibrary, mod)
    }

    for filename in get_patchable_xml_files() {
        let xml = core_library.node_dictionary[&filename].clone().to_string();
        let path = core_path(corepath_str, &filename);

        let mut file = File::create(&path)?;
        file.write_all(xml.as_bytes())?;
    }

    let region_nodeset =
        core_library.node_dictionary["library/textures"].get_nodeset("//re[@n]")?;

    for region in region_nodeset {
        let name = region
            .attribute_value("n")
            .ok_or("Could not get value for n")?;
        if !core_library.modded_textures["_all_modded_textures"].contains(&name) {
            continue;
        }

        let page = region.attribute_value("t")
            .ok_or("Could not get value for t")?;
        
        let cims = Vec::new();
        let mut extra_assets = Vec::new();

        if !cims.contains(&page) {
            let cim_name = format!("{}.cim", page);
            let mut create = false;
            let mut width = 0;
            let mut height = 0;

            if !get_patchable_xml_files().contains(&format!("library/{}", cim_name)) {
                width = core_library.custom_textures["_custom_textures_cim"][&page]["w"];
                height = core_library.custom_textures["_custom_textures_cim"][&page]["h"];
                extra_assets.push(format!("library/{}", cim_name));
            } 
            
            //cims[page] = Texture::new(corepath_str, create, width, height);
        }



    }
    Ok(())
}


/*
enum TextureSize {
    Custom(i64, i64), // Represents custom width and height
    Default, // Represents default width and height
}

let custom_textures: HashMap<String, TextureSize> = HashMap::new();

// Access width and height
if let Some(texture_size) = custom_textures.get(&page) {
    match texture_size {
        TextureSize::Custom(width, height) => {
            // Use custom width and height
        }
        TextureSize::Default => {
            // Use default width and height
        }
    }
}

// Update width and height
custom_textures.insert(page.clone(), TextureSize::Custom(new_width, new_height));


 */
