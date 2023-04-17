pub enum Input<'a> {
    Document(&'a sxd_document::dom::Document<'a>),
    Node(&'a sxd_xpath::nodeset::Node<'a>),
}

pub fn evaluate_nodeset<'a>(
    xpath: &'a str,
    factory: &'a sxd_xpath::Factory,
    context: &'a sxd_xpath::Context<'a>,
    input: Input<'a>,
) -> Result<sxd_xpath::nodeset::Nodeset<'a>, Box<dyn std::error::Error>> {
    let node = match input {
        Input::Document(document) => document.root().try_into()?,
        Input::Node(node) => *node,
    };

    let path = factory.build(xpath)?;

    let result = path
        .map(|path| path.evaluate(context, node))
        .transpose()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to evaluate XPath expression: '{}'", xpath),
            )
        })?;

    match result {
        Some(sxd_xpath::Value::Nodeset(nodes)) => Ok(nodes),

        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to evaluate: Unknown Value type",
        ))),
    }
}

pub fn parse_attribute<T>(
    element: &sxd_document::dom::Element<'_>,
    attr_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let value = element.attribute_value(attr_name).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Attribute not found: '{}'", attr_name),
        )
    })?;

    value
        .parse::<T>()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to parse attribute '{}': {:?}", attr_name, e),
            )
        })
        .map_err(|e| e.into())
}

pub fn set_attribute<T>(
    element: &sxd_document::dom::Element<'_>,
    attr_name: &str,
    attr_value: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: std::fmt::Display,
{
    let string_value = format!("{}", attr_value);
    if let Some(_attr) = element.attribute(attr_name) {
        element.set_attribute_value(attr_name, &string_value);
        Ok(())
    } else {
        let err = std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Attribute not found: '{}'", attr_name),
        );
        Err(Box::new(err))
    }
}

pub fn get_child_node<'a>(
    node: &'a sxd_xpath::nodeset::Node<'a>,
    child_name: &str,
) -> Result<sxd_xpath::nodeset::Node<'a>, Box<dyn std::error::Error>> {
    for child_node in node.children() {
        if let Some(name) = child_node.prefixed_name() {
            if name.as_str() == child_name {
                return Ok(child_node);
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Child node '{}' not found.", child_name),
    )))
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
