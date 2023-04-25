pub fn get_attribute_value_xpath<T>(
    node: &amxml::dom::NodePtr,
    xpath: &str,
    attr_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let mut result: Option<T> = None;

    node.each_node(xpath, |n| {
        result = n
            .attribute_value(attr_name)
            .and_then(|value| value.parse().ok());
    })?;

    match result {
        Some(value) => Ok(value),
        None => Err("Attribute not found".into()),
    }
    .map_err(|e: Box<dyn std::error::Error>| {
        format!("Failed to parse attribute value: {}", e).into()
    })
}

pub fn get_attribute_value_node<T>(
    node: &amxml::dom::NodePtr,
    attr_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let mut result: Option<T> = None;

    result = node
        .attribute_value(attr_name)
        .and_then(|value| value.parse().ok());

    match result {
        Some(value) => Ok(value),
        None => Err("Attribute not found".into()),
    }
    .map_err(|e: Box<dyn std::error::Error>| {
        format!("Failed to parse attribute value: {}", e).into()
    })
}

pub fn get_mod_dirs() -> Result<Vec<(String, std::path::PathBuf)>, Box<dyn std::error::Error>> {
    let results = Vec::new();
    let mut gamedir = find_steam_game()?;
    gamedir = gamedir.join("mods");

    if gamedir.is_dir() {
        for _entry in std::fs::read_dir(gamedir)? {}
    }

    log::info!("Found {} mods.", results.len());
    Ok(results)
}

pub fn get_save_dirs() -> Result<Vec<(String, std::path::PathBuf)>, Box<dyn std::error::Error>> {
    let gamedir = find_steam_game()?.join("savegames");
    let results = std::fs::read_dir(&gamedir)?
        .filter_map(|entry| {
            let subpath = entry.ok()?.path();
            subpath.file_name().map(|subdir_name| {
                (
                    subdir_name.to_string_lossy().to_string(),
                    subpath.join("save"),
                )
            })
        })
        .filter(|(_, save_dir)| save_dir.is_dir())
        .collect::<Vec<_>>();
    log::info!("Found {} game saves.", results.len());
    Ok(results)
}

fn find_steam_game() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut steamdir = steamlocate::SteamDir::locate().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find steam directory.",
        )
    })?;
    let path = steamdir
        .app(&979110)
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find steam game.")
        })?
        .path
        .to_path_buf();
    Ok(path)
}
