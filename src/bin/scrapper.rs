#![feature(async_closure)]

use scraper::Node;
use std::collections::{HashMap, HashSet};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = "https://pokemondb.net".to_owned();
    let main_url = root.clone() + "/pokedex/game/sword-shield";
    let resp = reqwest::get(main_url.as_str()).await?.text().await?;
    let document = scraper::Html::parse_document(resp.as_str());
    let pk_name_selector = scraper::Selector::parse("a.ent-name").unwrap();

    let vitals_select = scraper::Selector::parse("table.vitals-table").unwrap();
    let data_select = scraper::Selector::parse("table.data-table").unwrap();
    let row_select = scraper::Selector::parse("tr").unwrap();
    let strong_select = scraper::Selector::parse("strong").unwrap();
    let type_select = scraper::Selector::parse("a.type-icon").unwrap();
    let small_select = scraper::Selector::parse("small").unwrap();
    let th_select = scraper::Selector::parse("th").unwrap();
    let tr_cell_num_select = scraper::Selector::parse("td.cell-num").unwrap();

    for e in document.select(&pk_name_selector).take(5) {
        let name = e.inner_html();

        let url = root.to_owned() + e.value().attr("href").unwrap();
        let resp = reqwest::get(url.as_str()).await?.text().await?;
        let pk_document = scraper::Html::parse_document(resp.as_str());

        let mut pk = pokemon::Pokemon::default();
        pk.name = name;

        let mut vitals = pk_document.select(&vitals_select);

        if let Some(e) = vitals.next() {
            let mut rows = e.select(&row_select);

            if let Some(national) = rows.next() {
                let v = national.select(&strong_select).next().unwrap();
                pk.national_number = v.inner_html().parse().unwrap();
            }

            if let Some(ptype) = rows.next() {
                pk.ptype.extend(
                    ptype
                        .select(&type_select)
                        .map(|ptype| ptype.inner_html().parse::<pokemon::PType>().unwrap()),
                )
            }

            if let Some(_species) = rows.next() {}
            if let Some(_height) = rows.next() {}
            if let Some(_width) = rows.next() {}
            if let Some(_abilities) = rows.next() {}

            if let Some(local) = rows.next() {
                pk.local_number.extend(local.select(&small_select).map(|e| {
                    let num = match e.prev_sibling().unwrap().value() {
                        Node::Text(t) => {
                            let t = t.text.trim_start_matches('0').trim();
                            t.parse::<u32>().unwrap()
                        }
                        _ => 0,
                    };
                    let gen: pokemon::Generation = e
                        .inner_html()
                        .parse()
                        .unwrap_or(pokemon::Generation::Unknown);
                    (gen, num as u32)
                }));
            }
        }

        if let Some(_training) = vitals.next() {}
        if let Some(_breeding) = vitals.next() {}

        if let Some(stats) = vitals.next() {
            let mut rows = stats.select(&row_select);
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.hp = pokemon::HpStat(stat_value);
            }
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.attack = pokemon::AttackStat(stat_value);
            }
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.defense = pokemon::DefenseStat(stat_value);
            }
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.special_attack = pokemon::SpecialAttackStat(stat_value);
            }
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.special_defense = pokemon::SpecialDefenseStat(stat_value);
            }
            if let Some(stat) = rows.next() {
                let stat_value = stat
                    .select(&tr_cell_num_select)
                    .next()
                    .unwrap()
                    .inner_html()
                    .parse::<u32>()
                    .unwrap();
                pk.base_stats.speed = pokemon::SpeedStat(stat_value);
            }
        }

        let path = std::path::PathBuf::new()
            .join("data")
            .join(format!("{:0>4}.json", pk.national_number));
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(serde_json::to_string_pretty(&pk).unwrap().as_bytes())
            .expect("failed to write file");
        println!("{:#?}", pk);
    }
    Ok(())
}
