use crate::utils::{Input, evaluate_nodeset, set_attribute};

use super::gamesave::{GameSave, Ship};

use sxd_document::{dom::Document, parser, writer};
use sxd_xpath::{Factory, Context};

use std::error::Error;

pub fn write_save(gamesave: &GameSave) -> Result<(), Box<dyn Error>> {
    let save_path = gamesave.path.as_path();
    let content = std::fs::read_to_string(save_path)?;

    let package = parser::parse(&content)?;
    let document = package.as_document();
    let context = Context::new();
    let factory = Factory::new();

    write_player_bank(11111, &document, &factory, &context)?;
    write_ships(&gamesave.ships, &document, &factory, &context)?;

    let mut file = std::fs::File::create(save_path)?;
    writer::Writer::new().format_document(&document, &mut file)?;

    Ok(())
}

fn write_player_bank(
    amount: i32,
    document: &Document<'_>,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<(), Box<dyn Error>> {

    let player_bank_element = evaluate_nodeset(
        "./game/playerBank", 
        factory, 
        context, 
        Input::Document(document))?
        .document_order_first()
        .unwrap()
        .element()
        .unwrap();

    set_attribute(&player_bank_element, "ca", amount)?;
        
    Ok(())
}

fn write_ships(
    _ships: &Vec<Ship>, 
    document: &Document<'_>,
    factory: &Factory,
    context: &Context<'_>,
) -> Result<(), Box<dyn Error>> {
    
    let ship_nodes = evaluate_nodeset(
        "./game/ships/ship",
        factory,
        context,
        Input::Document(document),
    )?;

    for _ship_node in ship_nodes.document_order() {

    }

    Ok(())
}
