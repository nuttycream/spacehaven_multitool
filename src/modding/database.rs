use std::{collections::HashMap, error::Error, path::PathBuf};

use amxml::dom::{new_document, NodePtr};

use crate::utils::{get_attribute_value_node, get_mod_dirs};
use std::str::FromStr;

pub enum ConfigVarValue {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl FromStr for ConfigVarValue {
    type Err = (); //todo, implement a generic error from std lib

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s.starts_with("str") || s.starts_with("text") || s.starts_with("txt") || s.is_empty() {
            Ok(ConfigVarValue::Str(s.to_string()))
        } else if s.starts_with("int") {
            s[3..]
                .parse::<i32>()
                .map(ConfigVarValue::Int)
                .map_err(|_| ())
        } else if s.starts_with("float") {
            s[5..]
                .parse::<f64>()
                .map(ConfigVarValue::Float)
                .map_err(|_| ())
        } else if s.starts_with("bool") {
            match s[4..].trim() {
                "1" | "-1" | "t" | "y" | "true" | "yes" | "on" => Ok(ConfigVarValue::Bool(true)),
                _ => Ok(ConfigVarValue::Bool(false)),
            }
        } else {
            Err(())
        }
    }
}

#[derive(Default)]
pub struct ModConfigVar {
    pub text: String,
    pub value: Option<String>,
    pub var_type: Option<ConfigVarValue>,
    pub size: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default: Option<String>,
}

#[derive(Default)]
pub struct Mod {
    pub name: String,
    pub min_loader_ver: String,
    pub supported_game_vers: Vec<String>,

    //in the future all mods should have these mandatory
    pub version: Option<String>,
    pub min_multitool_ver: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,

    //optional, permanently
    pub preview: Option<PathBuf>, // Should be image, will only use path for now
    pub website: Option<String>,
    pub nexusmods: Option<String>,
    pub order: Option<i32>,

    pub path: PathBuf,
    pub enabled: bool,

    pub mapped_ids: HashMap<String, i32>,
    pub config_variables: Option<HashMap<String, ModConfigVar>>,
    pub info_path: PathBuf,
}

pub fn load_mods() -> Result<Vec<Mod>, Box<dyn Error>> {
    let mut mods = Vec::new();

    let mod_dirs = get_mod_dirs()?;

    for mod_dir in &mod_dirs {
        let mut info_path = mod_dir.join("info");
        if !info_path.exists() {
            info_path.set_extension("xml");
        }
        if info_path.is_file() {
            let content = std::fs::read_to_string(info_path.as_path())?;
            let doc = new_document(&content)?;
            let root = doc
                .root_element()
                .get_first_node("/mod")
                .ok_or("Invalid mod info file")?;

            let name = root
                .get_first_node("//name/text()")
                .ok_or(format!("Could not find mod name for {}", mod_dir.display()))?
                .value();

            let mut author = None;
            if let Some(author_node) = root.get_first_node("//author/text()") {
                author = Some(author_node.value());
            }

            let mut description = None;
            if let Some(desc_node) = root.get_first_node("//description/text()") {
                description = Some(desc_node.value());
            }

            let mut version = None;
            if let Some(ver_node) = root.get_first_node("//version/text()") {
                version = Some(ver_node.value());
            }

            let min_loader_ver = root
                .get_first_node("//minimumLoaderVersion/text()")
                .ok_or(format!(
                    "Could not find min loader version for {}",
                    mod_dir.display()
                ))?
                .value();


            let mut supported_game_vers = Vec::new();
            for game_version in root.get_nodeset("//gameVersions/v/text()")? {
                supported_game_vers.push(game_version.value());
            }

            let mut order = None;
            if let Some(order_node) = root.get_first_node("//order/text()") {
                order = Some(order_node.value().parse::<i32>()?);
            }

            let mut config_variables = None;
            if let Some(config_node) = root.get_first_node("//config") {
                config_variables = Some(read_vars(&config_node)?);
            }

            

            mods.push(Mod {
                name,
                author,
                description,
                version,
                min_loader_ver,
                supported_game_vers,
                
                //None for now
                min_multitool_ver: None,

                //TODO: Implement
                preview: None,
                website: None,
                nexusmods: None,
                order,

                path: mod_dir.to_path_buf(),
                enabled: true,
                mapped_ids: HashMap::new(),
                config_variables,
                info_path,
            })
        }
    }

    mods.sort_by(|mod_a, mod_b| {
        match (mod_a.order, mod_b.order) {
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => mod_a.order.cmp(&mod_b.order),
        }
    });

    log::info!("Loaded {}/{} mods", mods.len(), mod_dirs.len());

    Ok(mods)
}

fn read_vars(config_node: &NodePtr) -> Result<HashMap<String, ModConfigVar>, Box<dyn Error>> {
    let mut mod_config_var = HashMap::new();

    for var_node in config_node.children() {
        if !(var_node.name() == "var") {
            continue;
        }
        let name = get_attribute_value_node(&var_node, "name")?;
        let text = var_node
            .get_first_node(".//text()")
            .ok_or(format!("No Text Value name for {}", &name))?
            .value();



        mod_config_var.insert(
            name,
            ModConfigVar {
                text,
                value: None,
                var_type: None,
                size: None,
                min: None,
                max: None,
                default: None,
            },
        );
    }

    Ok(mod_config_var)
}
