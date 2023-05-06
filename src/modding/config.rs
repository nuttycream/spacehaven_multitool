use std::collections::HashMap;
pub struct HavenIdLookupTable<'a>(HashMap<&'a str, &'a str>);

impl Default for HavenIdLookupTable<'_> {
    fn default() -> Self {
        Self(
            [
                ("/data/BackPack", "mid"),
                ("/data/BackStory", "id"),
                ("/data/CelestialObject", "id"),
                ("/data/Character", "cid"),
                ("/data/CharacterCondition", "id"),
                ("/data/CharacterSet", "cid"),
                ("/data/CharacterTrait", "id"),
                ("/data/CostGroup", "id"),
                ("/data/Craft", "cid"),
                ("/data/DataLog", "id"),
                ("/data/DataLogFragment", "id"),
                ("/data/DefaultStuff", "id"),
                ("/data/DialogChoice", "id"),
                ("/data/DifficultySettings", "id"),
                ("/data/Effect", "id"),
                ("/data/Element", "mid"),
                ("/data/Encounter", "id"),
                ("/data/Explosion", "id"),
                ("/data/Faction", "id"),
                ("/data/FloorExpPackage", "id"),
                ("/data/GameScenario", "id"),
                ("/data/GOAPAction", "id"),
                ("/data/IdleAnim", "id"),
                ("/data/IsoFX", "id"),
                ("/data/Item", "mid"),
                ("/data/MainCat", "id"),
                ("/data/Monster", "cid"),
                ("/data/Notes", "id"),
                ("/data/ObjectiveCollection", "nid"),
                ("/data/PersonalitySettings", "id"),
                ("/data/Plan", "id"),
                ("/data/Product", "eid"),
                ("/data/Randomizer", "id"),
                ("/data/RandomShip", "id"),
                ("/data/Robot", "cid"),
                ("/data/RoofExpPackage", "id"),
                ("/data/Room", "rid"),
                ("/data/Sector", "id"),
                ("/data/Ship", "rid"),
                ("/data/SubCat", "id"),
                ("/data/Tech", "id"),
                ("/data/TechTree", "id"),
                ("/data/TradingValues", "id"),
            ]
            .into(),
        )
    }
}
