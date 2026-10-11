use tabular::reader::parse;

fn expected() -> Vec<Vec<String>> {
    include_str!("../data/orders_expected.txt")
        .lines()
        .map(|l| l.split('\t').map(String::from).collect())
        .collect()
}

#[test]
fn orders_file_parses_completely() {
    let got = parse(include_str!("../data/orders.csv"));
    for (i, r) in got.iter().enumerate() {
        println!("parsed {:>5}: {:?}", i, r);
    }
    assert_eq!(got, expected());
}

#[test]
fn orders_file_column_count() {
    let got = parse(include_str!("../data/orders.csv"));
    assert!(got.iter().all(|r| r.len() == 4), "rows: {:#?}", got);
}

#[test]
fn quoted_speech_is_preserved() {
    let text: String = include_str!("../data/orders.csv").lines().filter(|l| l.contains("say")).collect::<Vec<_>>().join("\n");
    let got: Vec<String> = parse(&text).into_iter().map(|r| r[3].clone()).collect();
    let want: Vec<String> = expected().into_iter().filter(|r| r[3].starts_with("say")).map(|r| r[3].clone()).collect();
    assert_eq!(got, want);
}
