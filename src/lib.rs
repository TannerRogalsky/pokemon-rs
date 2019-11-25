use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Generation {
    Unknown,
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

        if case_insensitive_contains!(s, "ruby") {
            Ok(Self::III)
        } else if case_insensitive_contains!(s, "sapphire") {
            Ok(Self::III)
        } else if case_insensitive_contains!(s, "emerald") {
            Ok(Self::III)
        } else if case_insensitive_contains!(s, "sword") {
            Ok(Self::VIII)
        } else if case_insensitive_contains!(s, "shield") {
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

macro_rules! stat_string {
    ($stat:ident, $name:tt) => {
        impl FromStr for $stat {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == $name {
                    Ok(Self::default())
                } else {
                    Err(())
                }
            }
        }
    };
}

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HpStat(pub u32);
stat_string!(HpStat, "HP");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AttackStat(pub u32);
stat_string!(AttackStat, "Attack");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DefenseStat(pub u32);
stat_string!(DefenseStat, "Defense");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SpecialAttackStat(pub u32);
stat_string!(SpecialAttackStat, "Sp. Atk");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SpecialDefenseStat(pub u32);
stat_string!(SpecialDefenseStat, "Sp. Def");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SpeedStat(pub u32);
stat_string!(SpeedStat, "Speed");

#[derive(Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BaseStats {
    pub hp: HpStat,
    pub attack: AttackStat,
    pub defense: DefenseStat,
    pub special_attack: SpecialAttackStat,
    pub special_defense: SpecialDefenseStat,
    pub speed: SpeedStat,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub national_number: u32,
    pub ptype: HashSet<PType>,
    pub local_number: HashMap<Generation, u32>,
    pub base_stats: BaseStats,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
