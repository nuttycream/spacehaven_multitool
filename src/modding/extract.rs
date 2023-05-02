use std::{error::Error, fs::File, path::Path};
use zip::{ZipArchive, ZipWriter};

use super::{get_patchable_cim_files, get_patchable_xml_files};
use crate::utils::find_steam_game;

fn extract(jar_path: &Path, core_path: &Path) -> Result<(), Box<dyn Error>> {
    let full_path = find_steam_game()?.join(core_path);
    if !full_path.join(core_path).exists() {
        std::fs::create_dir_all(&full_path).expect("Failed to create directory");
    }

    let zip_file = File::open(jar_path)?;
    let mut zip_archive = ZipArchive::new(zip_file)?;

    for i in 0..zip_archive.len() {
        let mut zip_file = zip_archive.by_index(i)?;
        let file_path = std::path::PathBuf::from(zip_file.name());

        if file_path.starts_with("library/") && !file_path.ends_with("/") {
            let target_path = core_path.join(&file_path);
            std::fs::create_dir_all(target_path.parent().unwrap())?;
            let mut target_file = File::create(&target_path)?;
            std::io::copy(&mut zip_file, &mut target_file)?;
        }
    }
    Ok(())
}

fn patch(
    jar_path: &Path,
    core_path: &Path,
    result_path: &Path,
    extra_assets: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let original_zip = File::open(jar_path)?;
    let mut original_archive = ZipArchive::new(original_zip)?;
    let patched_zip = File::create(result_path)?;
    let mut patched_archive = ZipWriter::new(patched_zip);

    for i in 0..original_archive.len() {
        let mut original_file = original_archive.by_index(i)?;
        let file_path = std::path::PathBuf::from(original_file.name());
        let file_path_string = file_path.clone().into_os_string().into_string().unwrap();

        if !file_path.ends_with("/")
            && !get_patchable_xml_files().contains(&file_path_string)
            && !get_patchable_cim_files().contains(&file_path_string)
        {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut original_file, &mut buf)?;
            std::io::Write::write_all(&mut patched_archive, &buf)?;
        }
    }

    if let Some(extra_assets) = extra_assets {
        for file in extra_assets {
            let path = core_path.join(file.replace("/", &std::path::MAIN_SEPARATOR.to_string()));
            let file_path = std::path::PathBuf::from(&file);
            let file_path_string = &file_path.into_os_string().into_string().unwrap();
            let mut buf = Vec::new();
            let mut extra_file = File::open(path)?;
            std::io::Read::read_to_end(&mut extra_file, &mut buf)?;
            patched_archive.start_file(file_path_string, Default::default())?;
            std::io::Write::write_all(&mut patched_archive, &buf)?;
        }
    }

    patched_archive.finish()?;
    Ok(())
}
