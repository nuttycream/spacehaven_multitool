pub fn get_attribute_value_node<T>(
    node: &amxml::dom::NodePtr,
    attr_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{

    let result = node
        .attribute_value(attr_name)
        .and_then(|value| value.parse().ok());

    match result {
        Some(value) => Ok(value),
        None => Err(format!("Attribute {} not found in {} node", attr_name, node.local_name()).into()),
    }
    .map_err(|e: Box<dyn std::error::Error>| {
        format!("Failed to parse attribute value: {}", e).into()
    })
}

pub fn set_attribute_value_node<T>(
    node: &mut amxml::dom::NodePtr,
    attr_name: &str,
    attr_value: T,
) -> Result<(), Box<dyn std::error::Error>> 
where
    T: std::fmt::Display,
{
    let attr_value_str = format!("{}", attr_value);
    if node.attributes().iter().any(|attr| attr.name() == attr_name) {
        node.set_attribute(attr_name, &attr_value_str);
        Ok(())
    } else {
        Err(format!("Attribute {} not found in {} node", attr_name, node.local_name()).into())
    }
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
    let results = std::fs::read_dir(gamedir)?
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

pub fn find_steam_game() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
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
