// pkmn

pub mod mapper;

use serde_derive::Deserialize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Pokemon {
    pub dex_number: u64,
    pub name: String,
}

// This is what #[derive(Serialize)] would generate
impl Serialize for Pokemon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Pokemon", 2)?;
        s.serialize_field("dex_number", &self.dex_number)?;
        s.serialize_field("name", &self.name)?;
        s.end()
    }
}