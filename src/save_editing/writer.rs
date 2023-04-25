

use super::gamesave::GameSave;

use amxml::dom::*;

use std::error::Error;

pub fn write_save(gamesave: &GameSave) -> Result<(), Box<dyn Error>> {
    let save_path = gamesave.path.as_path();
    let content = std::fs::read_to_string(save_path)?;

    let doc = new_document(&content)?;

    let root = doc.root_element();

    //let player_bank_xpath = "/game/playerBank[@ca]";

    //let player_bank = root.eval_xpath(player_bank_xpath)?;

    let mut amount = String::new();

    root.each_node("/game/playerBank[@ca]", |n| {
        amount = n.attribute_value("ca").unwrap();
    })?;

    log::info!("{:?}", amount);

    root.each_node("/game/playerBank[@ca]", |mut n| {
        n.set_attribute("ca", "111111");
    })?;

    root.each_node("/game/playerBank[@ca]", |n| {
        amount = n.attribute_value("ca").unwrap();
    })?;

    log::info!("{:?}", amount);
    
    let result = root.to_string();

    std::fs::write(save_path, result)?;

    Ok(())
}


