use super::gamesave::{Character, Faction, GameSave, Ship, Tech};
use crate::utils::{get_attribute_value_node, set_attribute_value_node};

use amxml::dom::{new_document, NodePtr};

use std::{collections::HashMap, error::Error};

pub fn write_save(gamesave: &GameSave) -> Result<(), Box<dyn Error>> {
    let save_path = gamesave.path.as_path();
    let content = std::fs::read_to_string(save_path)?;
    let doc = new_document(&content)?;
    log::info!("Saving {}", save_path.display());
    let mut root = doc.root_element();

    let mut amount = String::new();

    root.each_node("/game/playerBank[@ca]", |n| {
        amount = n.attribute_value("ca").unwrap();
    })?;

    let mut player_bank = root.get_first_node("/game/playerBank").unwrap();
    set_attribute_value_node(&mut player_bank, "ca", gamesave.bank)?;
    write_ships(&mut root, &gamesave.ships)?;
    write_factions(&mut root, &gamesave.factions)?;
    write_research(&mut root, &gamesave.research_tree)?;
    write_game_settings(&mut root, &gamesave.game_settings)?;

    let result = root.to_string();
    std::fs::write(save_path, result)?;

    Ok(())
}

fn write_ships(root: &mut NodePtr, ships: &Vec<Ship>) -> Result<(), Box<dyn Error>> {
    let ship_nodes = root.get_nodeset("/game/ships/ship")?;
    log::info!("Found {} ship nodes", ship_nodes.len());

    let mut ships_by_name = HashMap::new();
    for ship in ships {
        ships_by_name.insert(ship.name.to_string(), ship);
    }

    let ship_node_count = ship_nodes.len();
    let mut ship_count = 0;
    for mut ship_node in ship_nodes {
        let ship_node_name = get_attribute_value_node::<String>(&ship_node, "sname")
            .unwrap_or("NO NAME".to_string());
        if let Some(ship) = ships_by_name.get(&ship_node_name) {
            log::info!("Writing {} to ship nodes", ship.name);
            write_characters(&mut ship_node, &ship.characters)?;
            write_storages(&mut ship_node, &ship.item_storages)?;
            write_tools(&ship_node, &ship.tool_storages)?;
            ship_count += 1;
            log::info!("Completed {} of {} ship nodes", ship_count, ship_node_count);
        }
    }

    Ok(())
}

fn write_characters(
    ship_node: &mut NodePtr,
    characters: &Vec<Character>,
) -> Result<(), Box<dyn Error>> {
    let character_nodes = ship_node.get_nodeset("./characters/c")?;
    let mut character_by_name = HashMap::new();
    for character in characters {
        character_by_name.insert(character.name.to_string(), character);
    }

    let character_node_count = character_nodes.len();
    let mut character_count = 0;
    for mut character_node in character_nodes {
        let character_node_name = get_attribute_value_node::<String>(&character_node, "name")?;
        if let Some(character) = character_by_name.get(&character_node_name) {
            set_attribute_value_node(&mut character_node, "side", &character.side)?;
            write_stats(&character_node, &character.stats)?;
            write_attributes(&character_node, &character.attributes)?;
            write_traits(&character_node, &character.traits)?;
            write_skills(&character_node, &character.skills)?;

            character_count += 1;
            if character_count == character_node_count {
                log::info!("Completed {} character nodes", character_node_count);
            }
        }
    }

    Ok(())
}

fn write_stats(char_node: &NodePtr, stats: &HashMap<String, i32>) -> Result<(), Box<dyn Error>> {
    let props_nodes = char_node.get_nodeset("./props/*[@v]")?;
    for mut stats_node in props_nodes {
        if let Some(stat) = stats.get(&stats_node.local_name()) {
            set_attribute_value_node(&mut stats_node, "v", stat)?;
        }
    }
    Ok(())
}

fn write_attributes(
    char_node: &NodePtr,
    attributes: &HashMap<i32, i32>,
) -> Result<(), Box<dyn Error>> {
    let attribute_nodes = char_node.get_nodeset("./pers/attr/a")?;
    for mut attribute_node in attribute_nodes {
        if let Some(attribute) =
            attributes.get(&get_attribute_value_node::<i32>(&attribute_node, "id")?)
        {
            set_attribute_value_node(&mut attribute_node, "points", attribute)?;
        }
    }
    Ok(())
}

fn write_traits(char_node: &NodePtr, traits: &Vec<i32>) -> Result<(), Box<dyn Error>> {
    let traits_root_node = char_node
        .get_first_node("./pers/traits")
        .ok_or("No Trait Root Found")?;

    for child in traits_root_node.children() {
        traits_root_node.delete_child(&child);
    }

    for new_trait in traits {
        let trait_text = format!(r#"<t id="{}"/>"#, new_trait);
        let new_trait_doc = new_document(&trait_text)?;
        traits_root_node.append_child(&new_trait_doc);
    }

    Ok(())
}

fn write_skills(char_node: &NodePtr, skills: &HashMap<i32, i32>) -> Result<(), Box<dyn Error>> {
    let skills_nodes = char_node.get_nodeset("./pers/skills/s")?;
    for mut skill_node in skills_nodes {
        if let Some(skill) = skills.get(&get_attribute_value_node::<i32>(&skill_node, "sk")?) {
            set_attribute_value_node(&mut skill_node, "level", skill)?;
        }
    }

    Ok(())
}

fn write_storages(
    ship_node: &mut NodePtr,
    storages: &HashMap<i32, i32>,
) -> Result<(), Box<dyn Error>> {
    let storage_nodes = ship_node.get_nodeset("./e/l/feat/inv/s")?;
    for mut storage_node in storage_nodes {
        if let Some(amount) = storages.get(&get_attribute_value_node::<i32>(
            &storage_node,
            "elementaryId",
        )?) {
            set_attribute_value_node(&mut storage_node, "inStorage", amount)?;
        }
    }
    Ok(())
}

fn write_tools(ship_node: &NodePtr, tools: &[i32]) -> Result<(), Box<dyn Error>> {
    let tool_nodes = ship_node.get_nodeset("./e/lfeat/prod/inv/s[@elementaryId=162]")?;
    for (count, mut tool_node) in tool_nodes.into_iter().enumerate() {
        set_attribute_value_node(&mut tool_node, "inStorage", tools[count])?;
    }
    Ok(())
}

fn write_factions(root: &mut NodePtr, factions: &[Faction]) -> Result<(), Box<dyn Error>> {
    let faction_nodes = root.get_nodeset("/game/hostmap/map/l")?;

    let mut faction_count = 0;
    let faction_node_count = faction_nodes.len();
    for mut faction_node in faction_nodes {
        let faction_name = get_attribute_value_node::<String>(&faction_node, "s1")?;
        let relationship_name = get_attribute_value_node::<String>(&faction_node, "s2")?;
        let faction = factions.iter().find(|f| f.name == faction_name);

        if faction.is_none() {
            continue;
        }

        if let Some(relationship) =
            faction.and_then(|f| f.relationships.iter().find(|r| r.name == relationship_name))
        {
            set_attribute_value_node(&mut faction_node, "relationship", relationship.amount)?;
            set_attribute_value_node(&mut faction_node, "patience", relationship.patience)?;
            set_attribute_value_node(&mut faction_node, "stance", &relationship.stance)?;
            set_attribute_value_node(&mut faction_node, "accessTrade", relationship.trade)?;
            set_attribute_value_node(&mut faction_node, "accessShip", relationship.ship)?;
            set_attribute_value_node(&mut faction_node, "accessVision", relationship.vision)?;

            faction_count += 1;
        }
    }

    log::info!(
        "Completed {} of {} faction nodes",
        faction_count,
        faction_node_count
    );

    Ok(())
}

fn write_research(root: &mut NodePtr, research_tree: &Vec<Tech>) -> Result<(), Box<dyn Error>> {
    let research_nodes = root.get_nodeset("/game/research/states/l")?;

    let mut count = 0;
    let research_node_count = research_tree.len();

    for research_node in research_nodes {
        let research_node_id = get_attribute_value_node::<i32>(&research_node, "techId")?;
        let research = research_tree.iter().find(|r| r.id == research_node_id);

        if research.is_none() {
            continue;
        }

        let stage_nodes = research_node.get_nodeset("./stageStates/l")?;

        for stage_node in stage_nodes {
            let stage_level = get_attribute_value_node::<i32>(&stage_node, "stage")?;

            if let Some(stage) =
                research.and_then(|t| t.stages.iter().find(|s| *s.0 == stage_level))
            {
                if let Some(mut blocks_node) = stage_node.get_first_node("./blocksDone") {
                    set_attribute_value_node(&mut blocks_node, "level1", stage.1.basic)?;
                    set_attribute_value_node(&mut blocks_node, "level2", stage.1.intermediate)?;
                    set_attribute_value_node(&mut blocks_node, "level3", stage.1.advanced)?;
                }
            }
        }
        count += 1;
    }

    log::info!(
        "Completed {} of {} research nodes.",
        count,
        research_node_count
    );

    Ok(())
}

fn write_game_settings(
    root: &mut NodePtr,
    game_settings: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    if let Some(settings_node) = root.get_first_node("/game/settings/diff/modeSettings") {
        for mut settings_attribute in settings_node.attributes() {
            if let Some(settings) = game_settings
                .iter()
                .find(|s| *s.0 == settings_attribute.name())
            {
                settings_attribute.set_attribute(settings.0, settings.1);
                count += 1;
            }
        }
    }
    log::info!("Completed {} settings attributes.", count);

    Ok(())
}
