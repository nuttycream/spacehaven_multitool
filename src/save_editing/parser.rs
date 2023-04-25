use super::gamesave::{Character, Faction, GameSave, Relationship, Ship, Stage, Tech};
use crate::utils::{get_attribute_value_node, get_attribute_value_xpath};

use amxml::dom::{new_document, NodePtr};

use std::{collections::HashMap, error::Error};

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
    let factions = parse_factions(&root)?;
    let research_tree = parse_research(&root)?;
    let game_settings = parse_game_settings(&root)?;

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
            item_storages: parse_storages(&ship_node)?,
            tool_storages: parse_tools(&ship_node)?,
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
            get_attribute_value_node(&attribute_node, "points")?,
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
            get_attribute_value_node(&skill_node, "level")?,
        );
    }

    Ok(skills)
}

fn parse_storages(ship_node: &NodePtr) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut items = HashMap::new();

    let storage_nodes = ship_node.get_nodeset("./e/l/feat/inv/s")?;

    for storage_node in storage_nodes {
        items.insert(
            get_attribute_value_node(&storage_node, "elementaryId")?,
            get_attribute_value_node(&storage_node, "inStorage")?,
        );
    }
    Ok(items)
}

fn parse_tools(ship_node: &NodePtr) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut tools = Vec::new();

    let tool_nodes = ship_node.get_nodeset("./e/lfeat/prod/inv/s[@elementaryId=162]")?;

    for tool_node in tool_nodes {
        tools.push(get_attribute_value_node(&tool_node, "inStorage")?);
    }
    Ok(tools)
}

fn parse_factions(root: &NodePtr) -> Result<Vec<Faction>, Box<dyn Error>> {
    let mut factions: Vec<Faction> = Vec::new();
    let faction_nodes = root.get_nodeset("/game/hostmap/map/l")?;

    for faction_node in faction_nodes {
        let mut faction = Faction {
            name: get_attribute_value_node(&faction_node, "s1")?,
            relationships: Default::default(),
        };

        let relationship = Relationship {
            name: get_attribute_value_node(&faction_node, "s2")?,
            amount: get_attribute_value_node(&faction_node, "relationship")?,
            patience: get_attribute_value_node(&faction_node, "patience")?,
            stance: get_attribute_value_node(&faction_node, "stance")?,
            trade: get_attribute_value_node(&faction_node, "accessTrade")?,
            ship: get_attribute_value_node(&faction_node, "accessShip")?,
            vision: get_attribute_value_node(&faction_node, "accessVision")?,
        };

        if let Some(existing_faction) = factions.iter_mut().find(|f| f.name == faction.name) {
            existing_faction.relationships.push(relationship);
        } else {
            faction.relationships.push(relationship);
            factions.push(faction);
        }
    }
    Ok(factions)
}

fn parse_research(root: &NodePtr) -> Result<Vec<Tech>, Box<dyn Error>> {
    let mut research_tree = Vec::new();
    let research_nodes = root.get_nodeset("/game/research/states/l")?;

    for research_node in research_nodes {
        let mut tech = Tech {
            id: get_attribute_value_node(&research_node, "techId")?,
            ..Default::default()
        };

        let stage_nodes = research_node.get_nodeset("./stageStates/l")?;

        for stage_node in stage_nodes {
            let mut stage = Stage::default();
            let stage_level = get_attribute_value_node(&stage_node, "stage")?;

            let blocks_nodes = stage_node.get_nodeset("./blocksDone")?;

            for blocks_node in blocks_nodes {
                stage.basic = get_attribute_value_node(&blocks_node, "level1")?;
                stage.intermediate = get_attribute_value_node(&blocks_node, "level2")?;
                stage.advanced = get_attribute_value_node(&blocks_node, "level3")?;
            }

            tech.stages.insert(stage_level, stage);
        }

        research_tree.push(tech);
    }
    Ok(research_tree)
}

fn parse_game_settings(root: &NodePtr) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut game_setttings = HashMap::new();

    let settings_nodes = root.get_nodeset("./game/settings/diff/modeSettings")?;

    for settings_node in settings_nodes {
        for attribute in settings_node.attributes() {
            game_setttings.insert(attribute.local_name(), attribute.value());
        }
    }

    Ok(game_setttings)
}
