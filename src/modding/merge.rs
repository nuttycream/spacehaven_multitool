use super::get_patchable_xml_files;
use std::{collections::HashMap, error::Error, fs::File, io::Read, path::Path};

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
