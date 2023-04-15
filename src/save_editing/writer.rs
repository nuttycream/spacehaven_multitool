use super::gamesave::{GameSave, Ship};

use sxd_document::{dom::Document, parser, writer};

use std::error::Error;

pub fn write_save(gamesave: &GameSave) -> Result<(), Box<dyn Error>> {
    let save_path = gamesave.path.as_path().join("game");
    let content = std::fs::read_to_string(&save_path)?;

    let package = parser::parse(&content)?;
    let document = package.as_document();

    write_ships(&gamesave.ships, &document)?;

    let mut file = std::fs::File::create(save_path.as_path())?;
    writer::Writer::new().format_document(&document, &mut file)?;

    Ok(())
}

fn write_ships(ships: &Vec<Ship>, document: &Document<'_>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
