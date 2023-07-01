use egui_extras::RetainedImage;
use indexmap::IndexMap;
use std::{collections::HashMap, error::Error, io::Read, path::PathBuf};

use amxml::dom::{new_document, NodePtr};

use crate::utils::{get_attribute_value_node, get_mod_dirs};
use std::str::FromStr;

#[derive(Default)]
pub struct Mod {
    pub name: String,
    pub min_loader_ver: String,
    pub supported_game_vers: Vec<String>,
    pub info_path: PathBuf,

    //in the future all mods should have these mandatory
    pub version: Option<String>,
    pub min_multitool_ver: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,

    //optional, permanently
    pub preview: Option<RetainedImage>, // Should be image, will only use path for now
    pub website: Option<String>,
    pub nexusmods: Option<String>,
    pub order: Option<i32>,

    pub path: PathBuf,
    pub enabled: bool,

    //for merge
    pub mapped_ids: HashMap<String, i32>,
    pub config_variables: Option<IndexMap<String, ModConfigVar>>,
    pub prefix: u64,
}

#[derive(Debug, Clone)]
pub enum ConfigVarValue {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl FromStr for ConfigVarValue {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s.is_empty() {
            Ok(ConfigVarValue::Str(s))
        } else {
            // Try to parse as int
            if let Ok(int) = s.parse::<i32>() {
                return Ok(ConfigVarValue::Int(int));
            }
            // Try to parse as float
            if let Ok(float) = s.parse::<f64>() {
                return Ok(ConfigVarValue::Float(float));
            }
            // Try to parse as bool
            match s.as_str() {
                "1" | "-1" | "t" | "y" | "true" | "yes" | "on" => Ok(ConfigVarValue::Bool(true)),
                "0" | "f" | "n" | "false" | "no" | "off" => Ok(ConfigVarValue::Bool(false)),
                // If all else fails, treat it as a string
                _ => Ok(ConfigVarValue::Str(s)),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModConfigVar {
    pub text: String,
    pub value: String,
    pub var_type: ConfigVarValue,
    pub size: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default: Option<String>,
}

impl Mod {
    pub fn reset_to_defaults(&mut self) {
        if let Some(config_variables) = &mut self.config_variables {
            log::info!("Resetting config variables for {} to its default values", self.name);
            for (_, vars) in config_variables {
                vars.set_default();
            }
        }
    }

    pub fn save_to_file(&mut self) {
        
    }
}

impl ModConfigVar {
    fn set_default(&mut self) {
        if let Some(default) = &self.default {
            match &mut self.var_type {
                ConfigVarValue::Str(_) => {
                    self.var_type = ConfigVarValue::Str(default.clone());
                },
                ConfigVarValue::Int(_var) => {
                    if let Ok(default_int) = default.parse::<i32>() {
                        self.var_type = ConfigVarValue::Int(default_int);
                    }
                },
                ConfigVarValue::Float(_var) => {
                    if let Ok(default_float) = default.parse::<f64>() {
                        self.var_type = ConfigVarValue::Float(default_float);
                    }
                },
                ConfigVarValue::Bool(_var) => {
                    if let Ok(default_bool) = default.parse::<bool>() {
                        self.var_type = ConfigVarValue::Bool(default_bool);
                    }
                },
            };
            self.value = default.to_string();
        }
    }
}

pub fn load_mods() -> Result<Vec<Mod>, Box<dyn Error>> {
    let mut mods = Vec::new();

    let mod_dirs = get_mod_dirs()?;

    for mod_dir in &mod_dirs {
        let mut preview = None;
        let preview_path = mod_dir.join("preview.png");
        if preview_path.exists() {
            let mut buffer = vec![];
            std::fs::File::open(preview_path)
                .unwrap()
                .read_to_end(&mut buffer)
                .unwrap();
            preview = Some(RetainedImage::from_image_bytes("preview", &buffer[..]).unwrap());
        }
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

            let prefix;
            if let Some(mod_id) = root.get_first_node("//modid/text()") {
                prefix = mod_id.value().parse::<u64>()?;
            } else {
                prefix = generate_mod_id(&name);
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
                preview,
                website: None,
                nexusmods: None,
                order,

                prefix,
                path: mod_dir.to_path_buf(),
                enabled: true,
                mapped_ids: HashMap::new(),
                config_variables,
                info_path,
            })
        }
    }

    if mods.is_empty() {
        log::warn!("No mods found");
        return Ok(mods)
    }

    mods.sort_by(|mod_a, mod_b| match (mod_a.order, mod_b.order) {
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => mod_a.order.cmp(&mod_b.order),
    });

    log::info!("Loaded {}/{} mods", mods.len(), mod_dirs.len());

    Ok(mods)
}

fn read_vars(config_node: &NodePtr) -> Result<IndexMap<String, ModConfigVar>, Box<dyn Error>> {
    let mut mod_config_var = IndexMap::new();
    let var_nodes = config_node.get_nodeset("//var")?;

    for var_node in var_nodes {
        let name = get_attribute_value_node(&var_node, "name")?;
        let text = var_node
            .get_first_node(".//text()")
            .ok_or(format!("No Text Value name for {}", &name))?
            .value();

        let value: String = get_attribute_value_node(&var_node, "value")?;
        let var_type = value.parse::<ConfigVarValue>()?;

        let mut size = None;
        let mut min = None;
        let mut max = None;
        let mut default = None;

        if let Ok(size_attr) = get_attribute_value_node(&var_node, "size") {
            size = Some(size_attr);
        }

        if let Ok(min_attr) = get_attribute_value_node(&var_node, "min") {
            min = Some(min_attr);
        }

        if let Ok(max_attr) = get_attribute_value_node(&var_node, "max") {
            max = Some(max_attr);
        }

        if let Ok(def_attr) = get_attribute_value_node(&var_node, "default") {
            default = Some(def_attr);
        }

        mod_config_var.insert(
            name,
            ModConfigVar {
                text,
                value,
                var_type,
                size,
                min,
                max,
                default,
            },
        );
    }
    Ok(mod_config_var)
}

pub fn generate_mod_id(s: &str) -> u64 {
    let mut result = 0;
    for (i, byte) in s.bytes().enumerate() {
        result += u64::from(byte) << (i % 8 * 8);
    }
    result % 1_000_000_000
}
