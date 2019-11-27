use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(deny_unknown_fields)]
struct Pokemon {
    num: i32,
    species: String,
    types: Vec<String>,
    base_stats: HashMap<String, i32>,
    abilities: HashMap<String, String>,
    heightm: f32,
    weightkg: f32,
    color: String,
    prevo: Option<String>,
    evo_level: Option<i32>,
    evos: Option<Vec<String>>,
    egg_groups: Vec<String>,

    gender_ratio: Option<HashMap<String, f32>>,
    other_formes: Option<Vec<String>>,
    other_forms: Option<Vec<String>>,
    base_species: Option<String>,
    forme: Option<String>,
    base_forme: Option<String>,
    evo_type: Option<String>,
    evo_item: Option<String>,
    evo_condition: Option<String>,
    evo_move: Option<String>,
    can_hatch: Option<bool>,
    gender: Option<String>,

    #[serde(rename(deserialize = "maxHP"))]
    max_hp: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Move {
    num: i32,
    base_power: i32,
    category: String,
    #[serde(rename(deserialize = "type"))]
    move_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct TypeData {
    damage_taken: HashMap<String, u8>
}

fn main() {
    let raw_pokedex = std::fs::read_to_string("raw/pokedex.json").unwrap();
    let pokedex: HashMap<String, Pokemon> = serde_json::from_str(raw_pokedex.as_str()).unwrap();

    let raw_moves = std::fs::read_to_string("raw/moves.json").unwrap();
    let moves: HashMap<String, Move> = serde_json::from_str(raw_moves.as_str()).unwrap();

    let raw_learnsets = std::fs::read_to_string("raw/learnsets.json").unwrap();
    let learnsets: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> = serde_json::from_str(raw_learnsets.as_str()).unwrap();
    for learnset in learnsets.values() {
        assert_eq!(learnset.len(), 1);
    }

    let raw_typechart = std::fs::read_to_string("raw/typechart.json").unwrap();
    let typechart: HashMap<String, TypeData> = serde_json::from_str(raw_typechart.as_str()).unwrap();
    println!("{:#?}", typechart["Bug"]);

    let zacian = &pokedex["zacian"];
    let zacian_learnset = &learnsets["zacian"]["learnset"];
    let zacian_moves = zacian_learnset.keys().map(|move_name| &moves[move_name]).collect::<Vec<_>>();
//    println!("{:#?}", zacian_moves);
}