use super::gamesave::{Character, Faction, GameSave, Relationship, Ship, Stage, Tech};
use crate::utils::{get_attribute_value_node, get_attribute_value_xpath};

use amxml::dom::{new_document, NodePtr};

use std::{collections::HashMap, error::Error, hash::Hash};

pub fn read_save(
    save_name: String,
    save_dir: &std::path::Path,
) -> Result<GameSave, Box<dyn std::error::Error>> {
    let save_path = save_dir.join("game");
    let content = std::fs::read_to_string(save_path.as_path())?;
    log::info!("Parsing {}", save_path.display());
    let doc = new_document(&content)?;
    let root = doc.root_element();

    let bank = get_attribute_value_xpath(&root, "/game/playerBank", "ca")?;
    let ships = parse_ships(&root)?;
    let factions = parse_factions()?;
    let research_tree = parse_research()?;
    let game_settings = parse_game_settings()?;

    Ok(GameSave {
        name: save_name,
        path: save_path,
        date: "Not Implemented".to_string(), //info file, use UNIX epoch conversion
        bank,
        ships,
        factions,
        research_tree,
        game_settings,
    })
}

fn parse_ships(root: &NodePtr) -> Result<Vec<Ship>, Box<dyn Error>> {
    let mut ships = Vec::new();
    let ship_nodes = root.get_nodeset("/game/ships/ship")?;

    log::info!("Found {} ship/s", ship_nodes.len());

    for ship_node in ship_nodes {
        let ship = Ship {
            name: get_attribute_value_node(&ship_node, "sname").unwrap_or("None".to_string()),
            owner: "Not Implemented".to_string(),
            size_x: get_attribute_value_node(&ship_node, "sx")?,
            size_y: get_attribute_value_node(&ship_node, "sy")?,
            characters: parse_characters(&ship_node)?,
            item_storages: parse_storages()?,
            tool_storages: parse_tools()?,
        };
        log::info!("Successfully parsed {}", ship.name);
        ships.push(ship);
    }

    Ok(ships)
}

fn parse_characters(ship_node: &NodePtr) -> Result<Vec<Character>, Box<dyn Error>> {
    let mut characters = Vec::new();
    let character_nodes = ship_node.get_nodeset("./characters/c")?;

    log::info!("Found {} characters", character_nodes.len());

    for character_node in character_nodes {
        let character = Character {
            name: get_attribute_value_node(&character_node, "name")?,
            side: get_attribute_value_node(&character_node, "side")?,
            stats: parse_stats(&character_node)?,
            attributes: parse_attributes(&character_node)?,
            traits: parse_traits(&character_node)?,
            skills: parse_skills(&character_node)?,
        };
        characters.push(character);
    }

    Ok(characters)
}

fn parse_stats(char_node: &NodePtr) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let mut stats = HashMap::new();

    let props_nodes = char_node.get_nodeset("./props/*[@v]")?;

    for stats_node in props_nodes {
        stats.insert(
            stats_node.local_name(),
            get_attribute_value_node(&stats_node, "v")?,
        );
    }
    Ok(stats)
}

fn parse_attributes(char_node: &NodePtr) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut attributes = HashMap::new();
    let attribute_nodes = char_node.get_nodeset("./pers/attr/a")?;
    for attribute_node in attribute_nodes {
        attributes.insert(
            get_attribute_value_node(&attribute_node, "id")?, 
            get_attribute_value_node(&attribute_node, "points")?
        );
    }
    //log::info!("{:?}", attributes);
    Ok(attributes)
}

fn parse_traits(char_node: &NodePtr) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut traits = Vec::new();
    let traits_nodes = char_node.get_nodeset("./pers/traits/t")?;

    for traits_node in traits_nodes {
        traits.push(get_attribute_value_node(&traits_node, "id")?);
    }

    Ok(traits)
}

fn parse_skills(char_node: &NodePtr) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut skills = HashMap::new();
    let skills_nodes = char_node.get_nodeset("./pers/skills/s")?;

    for skill_node in skills_nodes {
        skills.insert(
            get_attribute_value_node(&skill_node, "sk")?, 
            get_attribute_value_node(&skill_node, "level")?
        );
    }

    Ok(skills)
}

fn parse_storages() -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let items = HashMap::new();

    Ok(items)
}

fn parse_tools() -> Result<Vec<i32>, Box<dyn Error>> {
    let tools = Vec::new();

    Ok(tools)
}

fn parse_factions() -> Result<Vec<Faction>, Box<dyn Error>> {
    let factions: Vec<Faction> = Vec::new();
    Ok(factions)
}

fn parse_research() -> Result<Vec<Tech>, Box<dyn Error>> {
    let research_tree = Vec::new();
    Ok(research_tree)
}

fn parse_game_settings() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let game_setttings = HashMap::new();

    Ok(game_setttings)
}
