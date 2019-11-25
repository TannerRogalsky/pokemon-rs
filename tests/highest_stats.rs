use pokemon::Pokemon;

#[test]
fn highest_stats() {
    let mut pks: Vec<Pokemon> = serde_json::from_reader(std::fs::File::open("data/pokemondb.json").unwrap()).unwrap();

    pks.sort_by(|a, b| b.base_stats.hp.partial_cmp(&a.base_stats.hp).unwrap());
    println!(
        "{:#?}",
        pks.iter()
            .map(|p| p.name.clone())
            .take(5)
            .collect::<Vec<_>>()
    );

    pks.sort_by(|a, b| b.base_stats.total().partial_cmp(&a.base_stats.total()).unwrap());
    println!(
        "{:#?}",
        pks.iter()
            .map(|p| (p.base_stats.total(), p))
            .take(5)
            .collect::<Vec<_>>()
    );
}
