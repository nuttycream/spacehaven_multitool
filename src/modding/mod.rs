pub mod view;
pub mod annotate;
pub mod extract;
pub mod load;
pub mod merge;
pub mod patch;

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
    let mut patchable_cim_files = Vec::with_capacity(24);
    for i in 0..24 {
        let filename = format!("library/{}.cim", i);
        patchable_cim_files.push(filename);
    }
    patchable_cim_files
}