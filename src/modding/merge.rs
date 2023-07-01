use amxml::dom::{new_document, NodePtr};
use image::io::Reader as ImageReader;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::Path,
};

use crate::utils::{get_attribute_value_node, set_attribute_value_node};

use super::{database::Mod, texture::Texture};

fn get_haven_id_table() -> HashMap<&'static str, &'static str> {
    let mut lookup_table = HashMap::new();
    lookup_table.insert("/data/BackPack", "mid");
    lookup_table.insert("/data/BackStory", "id");
    lookup_table.insert("/data/CelestialObject", "id");
    lookup_table.insert("/data/Character", "cid");
    lookup_table.insert("/data/CharacterCondition", "id");
    lookup_table.insert("/data/CharacterSet", "cid");
    lookup_table.insert("/data/CharacterTrait", "id");
    lookup_table.insert("/data/CostGroup", "id");
    lookup_table.insert("/data/Craft", "cid");
    lookup_table.insert("/data/DataLog", "id");
    lookup_table.insert("/data/DataLogFragment", "id");
    lookup_table.insert("/data/DefaultStuff", "id");
    lookup_table.insert("/data/DialogChoice", "id");
    lookup_table.insert("/data/DifficultySettings", "id");
    lookup_table.insert("/data/Effect", "id");
    lookup_table.insert("/data/Element", "mid");
    lookup_table.insert("/data/Encounter", "id");
    lookup_table.insert("/data/Explosion", "id");
    lookup_table.insert("/data/Faction", "id");
    lookup_table.insert("/data/FloorExpPackage", "id");
    lookup_table.insert("/data/GameScenario", "id");
    lookup_table.insert("/data/GOAPAction", "id");
    lookup_table.insert("/data/IdleAnim", "id");
    lookup_table.insert("/data/IsoFX", "id");
    lookup_table.insert("/data/Item", "mid");
    lookup_table.insert("/data/MainCat", "id");
    lookup_table.insert("/data/Monster", "cid");
    lookup_table.insert("/data/Notes", "id");
    lookup_table.insert("/data/ObjectiveCollection", "nid");
    lookup_table.insert("/data/PersonalitySettings", "id");
    lookup_table.insert("/data/Plan", "id");
    lookup_table.insert("/data/Product", "eid");
    lookup_table.insert("/data/Randomizer", "id");
    lookup_table.insert("/data/RandomShip", "id");
    lookup_table.insert("/data/Robot", "cid");
    lookup_table.insert("/data/RoofExpPackage", "id");
    lookup_table.insert("/data/Room", "rid");
    lookup_table.insert("/data/Sector", "id");
    lookup_table.insert("/data/Ship", "rid");
    lookup_table.insert("/data/SubCat", "id");
    lookup_table.insert("/data/Tech", "id");
    lookup_table.insert("/data/TechTree", "id");
    lookup_table.insert("/data/TradingValues", "id");

    lookup_table
}

fn get_patchable_xml_files() -> Vec<String> {
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

fn _get_patchable_cim_files() -> Vec<String> {
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
    modded_textures: HashMap<String, TextureMetadata>,
    custom_textures_cim: HashMap<String, Vec<NodePtr>>,

    last_core_region_id: i32,
    next_region_id: i32,
}

pub fn init_mods(active_mods: &Vec<Mod>) -> Result<(), Box<dyn Error>> {
    let core_path = crate::utils::find_steam_game()?
        .join("mods")
        .join("spacehaven");
    let mut core_library = CoreLibrary::default();

    for filename in get_patchable_xml_files() {
        let file_path = core_path.join(&filename);
        let xml_string = std::fs::read_to_string(&file_path)?;
        let node = amxml::dom::new_document(&xml_string)?;
        core_library.node_dictionary.insert(filename, node);
    }

    // find the last region in the texture file and remember its index
    // we will need this to add mod textures with consecutive indexes...
    let last_core_region_id;
    {
        let texture_lib = core_library
            .node_dictionary
            .get("library/textures")
            .ok_or("For some reason library/textures doesnt exist!")?;

        // find the last region in the texture file and remember its index
        // we will need this to add mod textures with consecutive indexes...
        last_core_region_id = get_attribute_value_node(
            &texture_lib
                .get_first_node("//re[@n][last()]")
                .ok_or("invalid region id")?,
            "n",
        )?;
    }

    core_library.last_core_region_id = last_core_region_id;
    core_library.next_region_id = core_library.last_core_region_id + 1;

    // Merges
    for active_mod in active_mods {
        let lib_path = &active_mod.path;
        let library_path = lib_path.join("library");
        if library_path.exists() {
            log::info!("Installing {} to the core library", active_mod.name);

            // Load the mod's library
            let mod_lib = build_library("library", lib_path)?;
            merge(&mut core_library, mod_lib, active_mod)?;
        }
    }

    // Patches
    for active_mod in active_mods {
        let lib_path = &active_mod.path;
        let patches_path = lib_path.join("patches");
        if patches_path.exists() {
            log::info!("Patching {}", active_mod.name);

            let patches = build_library("patches", lib_path)?;
            // doPatches(&core_library, patches, active_mod)
        }
    }

    // Write new game library
    for filename in get_patchable_xml_files() {
        // get the NodePtr from your CoreLibrary
        let node = core_library
            .node_dictionary
            .get(&filename)
            .ok_or_else(|| format!("Failed to find '{}' in core library.", filename))?;

        // generate the pretty string representation
        let xml_string = node.to_string();

        // write the string to the file
        std::fs::write(core_path.join(filename), xml_string.as_bytes())?;
    }

    // Add or overwrite textures from mods
    let cims: HashMap<String, Texture> = HashMap::new();
    let re_export_cims: HashMap<String, Texture> = HashMap::new();
    let extra_assets: Vec<String> = Vec::new();

    let texture_lib = core_library
        .node_dictionary
        .get("library/textures")
        .ok_or("For some reason library/textures doesnt exist!")?;

    for region in texture_lib.get_nodeset("//re[@n]")? {
        let name: String = get_attribute_value_node(&region, "n")?;

        if !core_library.modded_textures.contains_key(&name) {
            continue;
        }

        let png_file = core_library.modded_textures.get(&name).unwrap().clone();

        let page: String = get_attribute_value_node(&region, "t")?;

        if !cims.contains_key(&page) {
            let cim_name = format!("{}.cim", page);
        }
    }

    Ok(())
}

/// Build up a library dictionary of files in location
fn build_library(
    location: &str,
    mod_path: &Path,
) -> Result<HashMap<String, Vec<NodePtr>>, Box<dyn Error>> {
    let mut lib = HashMap::new();

    let mod_path = mod_path.join(location);

    let location_files = if mod_path.exists() {
        std::fs::read_dir(&mod_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    for target in get_patchable_xml_files() {
        let target_in_location = target.replace("library", location);

        for mod_file in &location_files {
            let mod_file_target = format!("{}/{}", location, mod_file);
            if !mod_file_target.starts_with(&target_in_location) {
                continue;
            }

            if !lib.contains_key(&target) {
                lib.insert(target.clone(), vec![]);
            }

            log::info!("{} => {}", &mod_file, &target);

            let xml_path = mod_path.join(&mod_file);
            let xml_string = std::fs::read_to_string(&xml_path)?;
            let node = amxml::dom::new_document(&xml_string)?;

            lib.get_mut(&target).unwrap().push(node);
        }
    }

    Ok(lib)
}

//
fn merge(
    core_library: &mut CoreLibrary,
    mod_lib: HashMap<String, Vec<NodePtr>>,
    active_mod: &Mod,
) -> Result<(), Box<dyn Error>> {
    // Do an element-wise merge (replacing conflicts)
    let mut current_file = "library/haven";

    if mod_lib.contains_key(current_file) {
        let base_file = get_base_file(core_library, current_file)?;

        for (path, loc_id) in get_haven_id_table() {
            //log::info!("xpath: {}, attribute: {}", path, loc_id);
            let mut base_root = get_base_root(base_file, &path)?;
            let xpath = format!("/{}", path);
            match merge_definitions(
                &mut base_root,
                mod_lib.to_owned(),
                current_file.to_string(),
                xpath,
                loc_id.to_string(),
            ) {
                Ok(_) => (),
                Err(_e) => {
                    continue;
                }
            }
        }
    }

    current_file = "library/texts";
    if mod_lib.contains_key(current_file) {
        let base_file = get_base_file(core_library, current_file)?;
        let mut base_root = get_base_root(base_file, "//t")?;
        match merge_definitions(
            &mut base_root,
            mod_lib.to_owned(),
            current_file.to_string(),
            "//t".to_string(),
            "id".to_string(),
        ) {
            Ok(_) => (),
            Err(_e) => {}
        }
    }

    core_library.modded_textures = detect_textures(core_library, mod_lib, active_mod)?;

    Ok(())
}

fn merge_definitions(
    base_root: &mut NodePtr,
    mod_lib: HashMap<String, Vec<NodePtr>>,
    file: String,
    xpath: String,
    id_attribute: String,
) -> Result<(), Box<dyn Error>> {
    if !mod_lib.contains_key(&file) {
        return Err(format!("Cannot find {} in mod library", file).into());
    }

    let mod_root_list = mod_lib
        .get(&file)
        .ok_or_else(|| format!("Mod library does not contain file: {}", file))?;

    for mod_root in mod_root_list {
        let mut merged = 0;

        // Iterate over elements in the mod root
        for mod_element in mod_root
            .root_element()
            .get_nodeset(&format!("{}/*", xpath))?
        {
            let id: String = get_attribute_value_node(&mod_element, &id_attribute)?;

            // Get conflicting elements in the base root
            let conflicts = base_root.get_nodeset(&format!("*[@{}='{}']", id_attribute, id))?;

            // Remove conflicts from base root
            for conflict in conflicts {
                base_root.delete_child(&conflict);
            }

            // Deep copy mod element to base root
            let copied_element = mod_element.clone();
            base_root.append_child(&copied_element);
            merged += 1;
            //log::info!("{}", mod_element);
        }

        if merged > 0 {
            log::info!("{}: Merged {} elements into {}", file, merged, xpath);
        }
    }

    Ok(())
}

fn get_base_file<'a>(
    base_lib: &'a mut CoreLibrary,
    file: &'a str,
) -> Result<&'a mut NodePtr, Box<dyn Error>> {
    base_lib
        .node_dictionary
        .get_mut(file)
        .ok_or_else(|| format!("Base library does not contain file: {}", file).into())
}

fn get_base_root(base_file: &mut NodePtr, xpath: &str) -> Result<NodePtr, Box<dyn Error>> {
    base_file
        .get_first_node(xpath)
        .ok_or_else(|| format!("Base library has no node at xpath: {}", xpath).into())
}

struct TextureMetadata {
    region_id: String,
    filename: String,
    path: std::path::PathBuf,
}

fn detect_textures(
    core_library: &mut CoreLibrary,
    mut mod_lib: HashMap<String, Vec<NodePtr>>,
    current_mod: &Mod,
) -> Result<HashMap<String, TextureMetadata>, Box<dyn Error>> {
    let mut modded_textures = HashMap::new();

    let textures_path = current_mod.path.join("textures");
    if !textures_path.is_dir() {
        return Ok(modded_textures);
    }

    let mut auto_animations = false;
    let animation_chunks = mod_lib.get("library/animations").ok_or(format!(
        "Could not find animations library in {}",
        current_mod.name
    ))?;

    for animation_chunk in animation_chunks {
        let filename_asset_pos = animation_chunk.get_first_node("//assetPos[@filename]");
        if filename_asset_pos.is_some() {
            auto_animations = true;
        }
    }

    // no textures file and no auto animations, we're done
    if !mod_lib.contains_key("library/textures") && !auto_animations {
        return Ok(modded_textures);
    }

    // This checks if 'library/textures' exists in the mod library
    // If it doesn't and autoAnimations is true, then create a new XML structure
    if !mod_lib.contains_key("library/textures") && auto_animations {
        let xml_string = r#"<AllTexturesAndRegions><textures/><regions/></AllTexturesAndRegions>"#;
        let doc = new_document(&xml_string).unwrap();

        // Wrap the NodePtr in a vec and insert it into the mod library hashmap
        mod_lib.insert("library/textures".to_string(), vec![doc]);
    }

    // This line gets the first NodePtr associated with the key 'library/textures'
    let textures_mod = &mod_lib["library/textures"][0];

    let mut mapping_n_region = HashMap::new();
    let mut seen_textures = HashSet::new();

    // This block iterates over all 't' elements with an 'i' attribute in textures_mod
    for texture_pack in textures_mod.get_nodeset("//t[@i]")? {
        let cim_id = get_attribute_value_node(&texture_pack, "i")?;
        core_library
            .custom_textures_cim
            .insert(cim_id, texture_pack.attributes());
    }

    // Map manually defined regions in textures file to autoIDs
    for region in textures_mod.get_nodeset("//re[@n]")? {
        let region_id: String = get_attribute_value_node(&region, "n")?;
        add_texture(
            &region_id,
            &textures_path,
            core_library,
            &mut modded_textures,
            &mut mapping_n_region,
            &mut seen_textures,
        );
    }

    // No custom mod textures, no need to remap ids
    if mapping_n_region.is_empty() && !auto_animations {
        return Ok(modded_textures);
    }

    let mut needs_autogeneration = HashSet::new();
    let animation_chunks = mod_lib.get("library/animations").ok_or(format!(
        "Could not find animations library in {}",
        current_mod.name
    ))?;

    for animation_chunk in animation_chunks {
        // iterate on autogeneration nodes
        let mut asset_pos_nodes = animation_chunk.get_nodeset("//assetPos[@filename]")?;
        for mut asset in asset_pos_nodes {
            let mut mod_local_id: String = get_attribute_value_node(&asset, "filename")?;
            mod_local_id = mod_local_id.trim_start_matches('/').to_string();

            if !mod_local_id.contains(".png") {
                mod_local_id += ".png";
            }
            if !needs_autogeneration.contains(&mod_local_id) {
                needs_autogeneration.insert(mod_local_id.clone());
                add_texture(
                    &mod_local_id,
                    &textures_path,
                    core_library,
                    &mut modded_textures,
                    &mut mapping_n_region,
                    &mut seen_textures,
                );
            }

            if !mapping_n_region.contains_key(&mod_local_id) {
                continue;
            }

            let new_id = mapping_n_region
                .get(&mod_local_id)
                .ok_or(format!("Could not retrieve mod local id: {}", mod_local_id))?;
            asset.set_attribute("a", &new_id);
        }

        // iterate on manually defined nodes
        asset_pos_nodes = animation_chunk.get_nodeset("//assetPos[@a and not(@filename)]")?;
        for mut asset in asset_pos_nodes {
            let mut mod_local_id: String = get_attribute_value_node(&asset, "a")?;
            // check if mod_local_id is numeric
            match mod_local_id.parse::<u32>() {
                Ok(_) => {
                    mod_local_id += ".png";
                    add_texture(
                        &mod_local_id,
                        &textures_path,
                        core_library,
                        &mut modded_textures,
                        &mut mapping_n_region,
                        &mut seen_textures,
                    );
                    match mapping_n_region.get(&mod_local_id) {
                        Some(new_id) => {
                            asset.set_attribute("a", new_id);
                        }
                        None => continue,
                    }
                }
                Err(_) => {
                    log::warn!("Cannot specify a non-numerical 'a' attribute {}. Specify in 'filename' attribute instead.",
                        mod_local_id);
                }
            }
        }
    }

    if !needs_autogeneration.is_empty() {
        let image_count = needs_autogeneration.len();

        let regions_node = textures_mod
            .get_first_node("//regions")
            .ok_or(format!("No regions node found for {}", &current_mod.name))?;

        let textures_node = textures_mod
            .get_first_node("//textures")
            .ok_or(format!("No textures node found for {}", &current_mod.name))?;

        let texture_id = current_mod.prefix;


        //let mut packer = Packer::new();
        let mut sum_a = 0;
        let mut sum_w = 0;
        let mut sum_h = 0;
        let mut min_required_dimension = 2048;

        for region_name in needs_autogeneration {
            let path = textures_path.join(region_name);
            let img = ImageReader::open(&path)?.decode()?.into_rgba8();
            let (w, h) = img.dimensions();
            //packer.rect_pack
            //min_required_dimension = max(min_required_dimension, w, h)
            sum_a += w * h;
            sum_w += w;
            sum_h += h;
        }

        
    }

    

    Ok(modded_textures)
}

fn add_texture(
    filename: &str,
    textures_path: &Path,
    core_library: &mut CoreLibrary,
    modded_textures: &mut HashMap<String, TextureMetadata>,
    mapping_n_region: &mut HashMap<String, String>,
    seen_textures: &mut HashSet<String>,
) {
    let split_filename: Vec<&str> = filename.split('.').collect();
    let region_id = split_filename[0..split_filename.len() - 1].join(".");
    let is_core_region = region_id.parse::<usize>().is_ok()
        && region_id.parse::<usize>().unwrap()
            <= core_library.last_core_region_id.try_into().unwrap();

    if modded_textures.contains_key(&region_id) || mapping_n_region.contains_key(&region_id) {
        return;
    }

    let path = textures_path.join(filename);

    if is_core_region && !path.exists() {
        return;
    }

    let core_region_id;
    if !is_core_region {
        core_region_id = core_library.next_region_id;
        mapping_n_region.insert(filename.to_string(), core_region_id.to_string());
        core_library.next_region_id = core_region_id + 1;
    // log: allocated new core region idx to file
    } else {
        core_region_id = region_id.parse::<i32>().unwrap();
        // log: mod updated texture region
    }

    seen_textures.insert(filename.to_string());

    modded_textures.insert(
        core_region_id.to_string(),
        TextureMetadata {
            region_id,
            filename: filename.to_string(),
            path,
        },
    );
}
