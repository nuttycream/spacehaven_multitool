use super::gamesave::{Character, Faction, GameSave, Relationship, Ship, Stage, Tech};

use crate::utils::{evaluate_nodeset, get_child_node, parse_attribute, Input};

use sxd_document::{dom::Document, parser};
use sxd_xpath::{nodeset::Node, Context, Factory};

use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

pub fn read_save(
    save_name: String,
    save_path: &Path,
) -> Result<GameSave, Box<dyn std::error::Error>> {
    let file = read_to_string(save_path.join("game"))?;
    let package = parser::parse(&file)?;
    let document = package.as_document();
    let context = Context::new();
    let factory = Factory::new();

    let bank = parse_bank(&document, &factory, &context)?;
    let ships = parse_ships(&document, &factory, &context)?;
    let factions = parse_factions(&document, &factory, &context)?;
    let research_tree = parse_research(&document, &factory, &context)?;
    let game_settings = parse_game_settings(&document, &factory, &context)?;

    Ok(GameSave {
        name: save_name,
        path: save_path.to_path_buf(),
        date: "Not Implemented".to_string(),
        bank,
        ships,
        factions,
        research_tree,
        game_settings,
    })
}

fn parse_bank(
    document: &Document,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<i32, Box<dyn Error>> {
    let player_bank = factory
        .build("./game/playerBank/@ca")?
        .unwrap()
        .evaluate(&context, document.root())?
        .number() as i32;

    Ok(player_bank)
}

fn parse_ships(
    document: &Document,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<Vec<Ship>, Box<dyn Error>> {
    let mut ships = Vec::new();

    let ship_nodes = evaluate_nodeset(
        "./game/ships/ship",
        &factory,
        &context,
        Input::Document(&document),
    )?;

    for ship_node in ship_nodes.document_order() {
        if let Some(ship_element) = ship_node.element() {
            ships.push(Ship {
                name: parse_attribute::<String>(&ship_element, "sname")?,
                owner: "Not Implemented".to_string(),
                size_x: parse_attribute::<i32>(&ship_element, "sx")?,
                size_y: parse_attribute::<i32>(&ship_element, "sy")?,
                characters: parse_characters(&ship_node, &factory, &context)?,
                item_storages: parse_storages(&ship_node, &factory, &context)?,
                tool_storages: parse_tools(&ship_node, &factory, &context)?,
            });
        }
    }

    Ok(ships)
}

fn parse_characters(
    ship_node: &Node,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<Vec<Character>, Box<dyn Error>> {
    let mut characters = Vec::new();

    let character_nodes = evaluate_nodeset(
        "./characters/c",
        &factory,
        &context,
        Input::Node(&ship_node),
    )?;

    for character_node in character_nodes.document_order() {
        if let Some(character_element) = character_node.element() {
            let props_node = get_child_node(&character_node, "props")?;
            let attr_node = get_child_node(&character_node, "attr")?;
            let traits_node = get_child_node(&character_node, "traits")?;
            let skills_node = get_child_node(&character_node, "skills")?;

            characters.push(Character {
                name: parse_attribute::<String>(&character_element, "name")?,
                side: parse_attribute::<String>(&character_element, "side")?,
                stats: parse_props(&props_node)?,
                attributes: parse_attr(&attr_node)?,
                traits: parse_traits(&traits_node)?,
                skills: parse_skills(&skills_node)?,
            });
        }
    }

    Ok(characters)
}

fn parse_props(props_node: &Node<'_>) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let mut props_map = HashMap::new();

    for child_node in props_node.children() {
        if let Some(node_element) = child_node.element() {
            if let Some(name) = child_node.prefixed_name() {
                props_map.insert(name, parse_attribute::<i32>(&node_element, "v")?);
            }
        }
    }
    Ok(props_map)
}

fn parse_attr(attr_node: &Node<'_>) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut attr_map = HashMap::new();

    for child_node in attr_node.children() {
        if let Some(node_element) = child_node.element() {
            attr_map.insert(
                parse_attribute::<i32>(&node_element, "id")?,
                parse_attribute::<i32>(&node_element, "points")?,
            );
        }
    }
    Ok(attr_map)
}

fn parse_traits(traits_node: &Node<'_>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut traits = Vec::new();

    for child_node in traits_node.children() {
        if let Some(node_element) = child_node.element() {
            traits.push(parse_attribute::<i32>(&node_element, "id")?);
        }
    }
    Ok(traits)
}

fn parse_skills(skills_node: &Node<'_>) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut skills = HashMap::new();

    for child_node in skills_node.children() {
        if let Some(node_element) = child_node.element() {
            skills.insert(
                parse_attribute::<i32>(&node_element, "sk")?,
                parse_attribute::<i32>(&node_element, "level")?,
            );
        }
    }
    Ok(skills)
}

fn parse_storages(
    ship_node: &Node,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<HashMap<i32, i32>, Box<dyn Error>> {
    let mut items = HashMap::new();

    let storage_nodes = evaluate_nodeset(
        "./e/l/feat/inv/s",
        &factory,
        &context,
        Input::Node(&ship_node),
    )?;

    for storage_node in storage_nodes.document_order() {
        if let Some(storage_element) = storage_node.element() {
            items.insert(
                parse_attribute(&storage_element, "elementaryId")?,
                parse_attribute(&storage_element, "inStorage")?,
            );
        }
    }

    Ok(items)
}

fn parse_tools(
    ship_node: &Node,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut tools = Vec::new();

    let tool_nodes = evaluate_nodeset(
        "./e/lfeat/prod/inv/s[@elementaryId=162]",
        &factory,
        &context,
        Input::Node(&ship_node),
    )?;

    for tool_node in tool_nodes.document_order() {
        if let Some(tool_element) = tool_node.element() {
            tools.push(parse_attribute(&tool_element, "inStorage")?);
        }
    }

    Ok(tools)
}

fn parse_factions(
    document: &Document,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<Vec<Faction>, Box<dyn Error>> {
    let mut factions: Vec<Faction> = Vec::new();

    let faction_nodes = evaluate_nodeset(
        "./game/hostmap/map/l",
        &factory,
        &context,
        Input::Document(&document),
    )?;

    for faction_node in faction_nodes.document_order() {
        if let Some(faction_element) = faction_node.element() {
            let mut faction = Faction {
                name: parse_attribute::<String>(&faction_element, "s1")?,
                relationships: Default::default(),
            };

            let relationship = Relationship {
                name: parse_attribute::<String>(&faction_element, "s2")?,
                amount: parse_attribute::<i32>(&faction_element, "relationship")?,
                patience: parse_attribute::<i32>(&faction_element, "s2")?,
                stance: parse_attribute::<String>(&faction_element, "stance")?,
                trade: parse_attribute::<bool>(&faction_element, "accessTrade")?,
                ship: parse_attribute::<bool>(&faction_element, "accessShip")?,
                vision: parse_attribute::<bool>(&faction_element, "accessVision")?,
            };

            if let Some(existing_faction) = factions.iter_mut().find(|f| f.name == faction.name) {
                existing_faction.relationships.push(relationship);
            } else {
                faction.relationships.push(relationship);
                factions.push(faction);
            }
        }
    }

    Ok(factions)
}

fn parse_research(
    document: &Document,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<Vec<Tech>, Box<dyn Error>> {
    let mut research_tree = Vec::new();

    let research_nodes = evaluate_nodeset(
        "./game/research/states/l",
        &factory,
        &context,
        Input::Document(&document),
    )?;

    for research_node in research_nodes.document_order() {
        if let Some(element) = research_node.element() {
            let mut tech = Tech::default();
            tech.id = parse_attribute(&element, "techId")?;
            let stage_nodes = evaluate_nodeset(
                "./stageStates/l",
                &factory,
                &context,
                Input::Node(&research_node),
            )?;

            for stage_node in stage_nodes.document_order() {
                if let Some(stage_element) = stage_node.element() {
                    let mut stage = Stage::default();
                    let stage_level = parse_attribute::<i32>(&stage_element, "stage")?;
                    stage.is_done = parse_attribute::<bool>(&stage_element, "done")?;
                    let blocks_done_nodes = evaluate_nodeset(
                        "./blocksDone",
                        &factory,
                        &context,
                        Input::Node(&stage_node),
                    )?;

                    for blocks_done_node in blocks_done_nodes.document_order() {
                        if let Some(blocks_done_element) = blocks_done_node.element() {
                            stage.basic = parse_attribute::<i32>(&blocks_done_element, "level1")?;
                            stage.intermediate =
                                parse_attribute::<i32>(&blocks_done_element, "level2")?;
                            stage.advanced =
                                parse_attribute::<i32>(&blocks_done_element, "level3")?;

                            //log::info!("{:?}", blocks_done_element.name())
                        }
                    }

                    tech.stages.insert(stage_level, stage);
                }
            }
            research_tree.push(tech);
        }
    }

    Ok(research_tree)
}

fn parse_game_settings(
    document: &Document,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut game_setttings = HashMap::new();

    let game_settings_nodes = evaluate_nodeset(
        "./game/settings/diff/modeSettings",
        &factory,
        &context,
        Input::Document(&document),
    )?;

    for game_settings_node in game_settings_nodes.document_order() {
        if let Some(settings_element) = game_settings_node.element() {
            for attribute in settings_element.attributes() {
                game_setttings.insert(
                    attribute.name().local_part().to_string(), 
                    attribute.value().to_string()
                );
            }
        }
    }

    Ok(game_setttings)
}
