pub enum Biome {
    Plain,
    Forest,
    Mountain,
    Desert,
    Swamp,
    Tundra,
    Sea,
    Ocean,
}

pub struct Tile {
    altitude: u8,
    biome: Biome,
    // temperature: u8,
    civilization_id: Option<u8>,
    structure_id: Option<u8>,
    passability: u8,
    // has_road: bool,
    // additional_info: u16,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            altitude: 0,
            biome: Biome::Plain,
            civilization_id: None,
            structure_id: None,
            passability: 0,
        }
    }
}

impl serde::Serialize for Tile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("altitude", &self.altitude)?;
        map.end()
    }
}

//TODO test