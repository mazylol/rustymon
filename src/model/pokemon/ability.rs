use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    #[serde(rename = "effect_changes")]
    pub effect_changes: Vec<EffectChange>,
    #[serde(rename = "effect_entries")]
    pub effect_entries: Vec<EffectEntry2>,
    #[serde(rename = "flavor_text_entries")]
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub generation: Generation,
    pub id: i64,
    #[serde(rename = "is_main_series")]
    pub is_main_series: bool,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon: Vec<Pokemon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectChange {
    #[serde(rename = "effect_entries")]
    pub effect_entries: Vec<EffectEntry>,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEntry {
    pub effect: String,
    pub language: Language,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroup {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEntry2 {
    pub effect: String,
    pub language: Language2,
    #[serde(rename = "short_effect")]
    pub short_effect: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlavorTextEntry {
    #[serde(rename = "flavor_text")]
    pub flavor_text: String,
    pub language: Language3,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language3 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroup2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generation {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub language: Language4,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language4 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    pub pokemon: Pokemon2,
    pub slot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon2 {
    pub name: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let body = reqwest::blocking::get("https://pokeapi.co/api/v2/ability/stench")
            .unwrap()
            .text()
            .unwrap();

        let ability: Ability = serde_json::from_str(&body).unwrap();

        println!("{:?}", ability);
    }
}
