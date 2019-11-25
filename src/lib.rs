use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Generation {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

impl FromStr for Generation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! case_insensitive_contains {
            ($a:ident, $b:expr) => {
                $a.to_ascii_lowercase().contains($b)
            };
        }

        if case_insensitive_contains!(s, "red/blue/yellow") {
            Ok(Self::I)
        } else if case_insensitive_contains!(s, "gold/silver/crystal") {
            Ok(Self::II)
        } else if case_insensitive_contains!(s, "firered/leafgreen") {
            Ok(Self::III)
        } else if case_insensitive_contains!(s, "ruby/sapphire/emerald") {
            Ok(Self::III)
        } else if case_insensitive_contains!(s, "diamond/pearl") {
            Ok(Self::IV)
        } else if case_insensitive_contains!(s, "platinum") {
            Ok(Self::IV)
        } else if case_insensitive_contains!(s, "heartgold/soulsilver") {
            Ok(Self::IV)
        } else if case_insensitive_contains!(s, "black/white") {
            Ok(Self::V)
        } else if case_insensitive_contains!(s, "black 2/white 2") {
            Ok(Self::V)
        } else if case_insensitive_contains!(s, "omega ruby/alpha sapphire") {
            Ok(Self::IV)
        } else if case_insensitive_contains!(s, "x/y") {
            Ok(Self::VI)
        } else if case_insensitive_contains!(s, "sun/moon") {
            Ok(Self::VII)
        } else if case_insensitive_contains!(s, "u.sun/u.moon") {
            Ok(Self::VII)
        } else if case_insensitive_contains!(s, "let's go") {
            Ok(Self::VII)
        } else if case_insensitive_contains!(s, "sword/shield") {
            Ok(Self::VIII)
        } else {
            Err(s.to_owned())
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl FromStr for PType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(Self::Normal),
            "Fire" => Ok(Self::Fire),
            "Water" => Ok(Self::Water),
            "Electric" => Ok(Self::Electric),
            "Grass" => Ok(Self::Grass),
            "Ice" => Ok(Self::Ice),
            "Fighting" => Ok(Self::Fighting),
            "Poison" => Ok(Self::Poison),
            "Ground" => Ok(Self::Ground),
            "Flying" => Ok(Self::Flying),
            "Psychic" => Ok(Self::Psychic),
            "Bug" => Ok(Self::Bug),
            "Rock" => Ok(Self::Rock),
            "Ghost" => Ok(Self::Ghost),
            "Dragon" => Ok(Self::Dragon),
            "Dark" => Ok(Self::Dark),
            "Steel" => Ok(Self::Steel),
            "Fairy" => Ok(Self::Fairy),
            _ => Err(()),
        }
    }
}

#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct HpStat(pub u32);
#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct AttackStat(pub u32);
#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct DefenseStat(pub u32);
#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct SpecialAttackStat(pub u32);
#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct SpecialDefenseStat(pub u32);
#[derive(
    Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub struct SpeedStat(pub u32);

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BaseStats {
    pub hp: HpStat,
    pub attack: AttackStat,
    pub defense: DefenseStat,
    pub special_attack: SpecialAttackStat,
    pub special_defense: SpecialDefenseStat,
    pub speed: SpeedStat,
}

impl BaseStats {
    pub fn total(&self) -> u32 {
        self.hp.0
            + self.attack.0
            + self.defense.0
            + self.special_attack.0
            + self.special_defense.0
            + self.speed.0
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub national_number: u32,
    pub ptype: HashSet<PType>,
    pub local_number: HashMap<Generation, u32>,
    pub base_stats: BaseStats,
}
