use pokemon::Pokemon;

#[test]
fn highest_stats() {
    let mut pks = std::fs::read_dir("data")
        .unwrap()
        .filter(|f| f.as_ref().unwrap().file_type().unwrap().is_file())
        .map(|f| std::fs::File::open(f.unwrap().path()).unwrap())
        .map(|f| serde_json::from_reader(f).unwrap())
        .collect::<Vec<Pokemon>>();

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
