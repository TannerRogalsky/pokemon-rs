use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
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
struct MoveFlags {
    #[serde(default)]
    contact: i32,
    #[serde(default)]
    charge: i32,
    #[serde(default)]
    protect: i32,
    #[serde(default)]
    mirror: i32,
    #[serde(default)]
    gravity: i32,
    #[serde(default)]
    distance: i32,
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
    flags: MoveFlags,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct TypeData {
    damage_taken: HashMap<String, u8>,
}

fn main() {
    //    let raw_pokedex = std::fs::read_to_string("raw/pokedex.json").unwrap();
    //    let pokedex: HashMap<String, Pokemon> = serde_json::from_str(raw_pokedex.as_str()).unwrap();

    let raw_moves = std::fs::read_to_string("raw/moves.json").unwrap();
    let moves: HashMap<String, Move> = serde_json::from_str(raw_moves.as_str()).unwrap();

    let raw_learnsets = std::fs::read_to_string("raw/learnsets.json").unwrap();
    let learnsets: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> =
        serde_json::from_str(raw_learnsets.as_str()).unwrap();
    for learnset in learnsets.values() {
        assert_eq!(learnset.len(), 1);
    }

    let raw_typechart = std::fs::read_to_string("raw/typechart.json").unwrap();
    let typechart: HashMap<String, TypeData> =
        serde_json::from_str(raw_typechart.as_str()).unwrap();

    {
        let types = vec![
            "Bug", "Dark", "Dragon", "Electric", "Fairy", "Fighting", "Fire", "Flying", "Ghost",
            "Grass", "Ground", "Ice", "Normal", "Poison", "Psychic", "Rock", "Steel", "Water",
        ];
        let coverage_score = |moves: &[&Move]| -> u32 {
            let mut coverage: HashMap<String, u32> = HashMap::with_capacity(types.len());
            types.iter().for_each(|t| {
                if moves
                    .iter()
                    .any(|m| typechart[t.clone()].damage_taken[&m.move_type] == 1)
                {
                    coverage.insert(t.to_string(), 1u32);
                }
            });
            coverage.iter().fold(0, |acc, (_k, &v)| acc + v)
        };

        let zacian_learnset = &learnsets["zacian"]["learnset"];
        let zacian_moves = zacian_learnset
            .keys()
            .map(|move_name| &moves[move_name])
            .filter(|m| m.category == "Physical" && m.base_power >= 80 && m.flags.charge == 0)
            .filter(|m| m.move_type != "Steel" || m.name == "Iron Head")
            .collect::<Vec<_>>();
        // strongest base power per type
        let zacian_moves = zacian_moves
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, &Move>, m| {
                match acc.entry(m.move_type.clone()) {
                    Entry::Occupied(mut o) => {
                        if o.get().base_power < m.base_power {
                            o.insert(m);
                        }
                    }
                    Entry::Vacant(v) => {
                        v.insert(m);
                    }
                }
                acc
            })
            .values()
            .map(|m| *m)
            .collect_vec();
        //        println!("{:#?}", zacian_moves);
        let combinations = zacian_moves
            .into_iter()
            .combinations(4)
            .filter(|moves| moves.iter().any(|m| m.name == "Iron Head"));
        let coverages = combinations.map(|moves| (coverage_score(moves.as_slice()), moves));
        let most = coverages.max_by(|(cov_a, _), (cov_b, _)| cov_a.cmp(cov_b));
        println!("{:#?}", most);

        println!(
            "{:#?}",
            coverage_score(
                ["Close Combat", "Crunch", "Psychic Fangs", "Iron Head",]
                    .iter()
                    .map(|n| moves.values().find(|m| m.name.as_str() == *n).unwrap())
                    .collect_vec()
                    .as_slice()
            )
        );
    }
}
