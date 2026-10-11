use tabular::reader::{parse, parse_record};
use tabular::writer::write_record;
use tabular::stats::{column_sum, column_width};

#[test]
fn roundtrip_000() {
    let fields: Vec<String> = vec!["river152, alpha589", "north374", "cobalt900", "maple929", "south481", "maple45", "stone432", "stone357"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river152, alpha589",north374,cobalt900,maple929,south481,maple45,stone432,stone357"#);
    assert_eq!(parse_record(&line), fields, "record 0 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_001() {
    let fields: Vec<String> = vec!["beta751", "north293", "cedar119", "maple284", "beta752", "alpha795", "amber720"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta751,north293,cedar119,maple284,beta752,alpha795,amber720");
    assert_eq!(parse_record(&line), fields, "record 1 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_002() {
    let fields: Vec<String> = vec!["stone255", "cedar973", "cedar717", "delta761", "north409"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone255,cedar973,cedar717,delta761,north409");
    assert_eq!(parse_record(&line), fields, "record 2 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_003() {
    let fields: Vec<String> = vec!["delta835, beta720", "maple987", "gamma249"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta835, beta720",maple987,gamma249"#);
    assert_eq!(parse_record(&line), fields, "record 3 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_004() {
    let fields: Vec<String> = vec!["alpha353", "north888", "delta805", "delta544", "amber767", "maple898", "beta979"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha353,north888,delta805,delta544,amber767,maple898,beta979");
    assert_eq!(parse_record(&line), fields, "record 4 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_005() {
    let fields: Vec<String> = vec!["stone394", "gamma50", "stone681", "river961", "stone581", "river663"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone394,gamma50,stone681,river961,stone581,river663");
    assert_eq!(parse_record(&line), fields, "record 5 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_006() {
    let fields: Vec<String> = vec!["north331, north540", "amber91", "beta682"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north331, north540",amber91,beta682"#);
    assert_eq!(parse_record(&line), fields, "record 6 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_007() {
    let fields: Vec<String> = vec!["north623", "north227"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north623,north227");
    assert_eq!(parse_record(&line), fields, "record 7 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_008() {
    let fields: Vec<String> = vec!["maple511", "cedar264", "amber146", "gamma238", "delta312", "stone84"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple511,cedar264,amber146,gamma238,delta312,stone84");
    assert_eq!(parse_record(&line), fields, "record 8 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_009() {
    let fields: Vec<String> = vec!["alpha747, cobalt911", "stone225", "cedar233", "river639", "gamma642", "stone807"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha747, cobalt911",stone225,cedar233,river639,gamma642,stone807"#);
    assert_eq!(parse_record(&line), fields, "record 9 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_010() {
    let fields: Vec<String> = vec!["amber515", "amber709", "south982", "gamma197", "cobalt716", "beta621", "river122"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber515,amber709,south982,gamma197,cobalt716,beta621,river122");
    assert_eq!(parse_record(&line), fields, "record 10 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_011() {
    let fields: Vec<String> = vec!["gamma593", "delta700", "stone392", "cobalt635"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma593,delta700,stone392,cobalt635");
    assert_eq!(parse_record(&line), fields, "record 11 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_012() {
    let fields: Vec<String> = vec!["cedar970, river196", "amber893"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar970, river196",amber893"#);
    assert_eq!(parse_record(&line), fields, "record 12 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_013() {
    let fields: Vec<String> = vec!["river198", "stone818", "alpha637", "maple41", "alpha234", "north440", "amber786", "cedar884"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river198,stone818,alpha637,maple41,alpha234,north440,amber786,cedar884");
    assert_eq!(parse_record(&line), fields, "record 13 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_014() {
    let fields: Vec<String> = vec!["delta266", "south830", "maple909"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta266,south830,maple909");
    assert_eq!(parse_record(&line), fields, "record 14 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_015() {
    let fields: Vec<String> = vec!["stone593, maple337", "cedar25"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone593, maple337",cedar25"#);
    assert_eq!(parse_record(&line), fields, "record 15 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_016() {
    let fields: Vec<String> = vec!["stone149", "south617", "river859", "south141", "river551"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone149,south617,river859,south141,river551");
    assert_eq!(parse_record(&line), fields, "record 16 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_017() {
    let fields: Vec<String> = vec!["gamma869", "south402", "gamma413", "amber976", "cedar739", "stone826"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma869,south402,gamma413,amber976,cedar739,stone826");
    assert_eq!(parse_record(&line), fields, "record 17 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_018() {
    let fields: Vec<String> = vec!["beta338, north863", "cedar119", "delta523", "delta716", "north893", "stone424"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta338, north863",cedar119,delta523,delta716,north893,stone424"#);
    assert_eq!(parse_record(&line), fields, "record 18 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_019() {
    let fields: Vec<String> = vec!["delta402", "cobalt277", "cobalt984", "gamma100", "gamma196", "gamma807"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta402,cobalt277,cobalt984,gamma100,gamma196,gamma807");
    assert_eq!(parse_record(&line), fields, "record 19 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_020() {
    let fields: Vec<String> = vec!["cedar750", "river890", "cedar537", "cobalt860", "maple817", "delta402", "maple453", "amber936"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar750,river890,cedar537,cobalt860,maple817,delta402,maple453,amber936");
    assert_eq!(parse_record(&line), fields, "record 20 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_021() {
    let fields: Vec<String> = vec!["south912, beta253", "maple751"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south912, beta253",maple751"#);
    assert_eq!(parse_record(&line), fields, "record 21 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_022() {
    let fields: Vec<String> = vec!["gamma875", "river369", "river272"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma875,river369,river272");
    assert_eq!(parse_record(&line), fields, "record 22 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_023() {
    let fields: Vec<String> = vec!["beta685", "gamma849", "cedar992", "north121", "stone856"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta685,gamma849,cedar992,north121,stone856");
    assert_eq!(parse_record(&line), fields, "record 23 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_024() {
    let fields: Vec<String> = vec!["river369, amber75", "south892"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river369, amber75",south892"#);
    assert_eq!(parse_record(&line), fields, "record 24 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_025() {
    let fields: Vec<String> = vec!["cobalt859", "north486", "gamma146"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt859,north486,gamma146");
    assert_eq!(parse_record(&line), fields, "record 25 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_026() {
    let fields: Vec<String> = vec!["maple359", "cobalt274", "stone121", "amber462"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple359,cobalt274,stone121,amber462");
    assert_eq!(parse_record(&line), fields, "record 26 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_027() {
    let fields: Vec<String> = vec!["south690, amber588", "amber286", "amber671", "gamma455"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south690, amber588",amber286,amber671,gamma455"#);
    assert_eq!(parse_record(&line), fields, "record 27 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_028() {
    let fields: Vec<String> = vec!["cobalt916", "north643", "amber719"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt916,north643,amber719");
    assert_eq!(parse_record(&line), fields, "record 28 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_029() {
    let fields: Vec<String> = vec!["beta728", "alpha91", "alpha620", "cobalt474", "beta176", "amber67"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta728,alpha91,alpha620,cobalt474,beta176,amber67");
    assert_eq!(parse_record(&line), fields, "record 29 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_030() {
    let fields: Vec<String> = vec!["beta679, cobalt438", "cedar880"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta679, cobalt438",cedar880"#);
    assert_eq!(parse_record(&line), fields, "record 30 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_031() {
    let fields: Vec<String> = vec!["amber772", "amber636", "gamma243", "beta167", "cedar316", "gamma483", "amber943", "south411"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber772,amber636,gamma243,beta167,cedar316,gamma483,amber943,south411");
    assert_eq!(parse_record(&line), fields, "record 31 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_032() {
    let fields: Vec<String> = vec!["cedar458", "delta679", "amber544", "maple362", "amber500", "delta206", "beta39", "south513"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar458,delta679,amber544,maple362,amber500,delta206,beta39,south513");
    assert_eq!(parse_record(&line), fields, "record 32 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_033() {
    let fields: Vec<String> = vec!["south995, stone867", "cedar182", "alpha113", "river256", "south524"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south995, stone867",cedar182,alpha113,river256,south524"#);
    assert_eq!(parse_record(&line), fields, "record 33 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_034() {
    let fields: Vec<String> = vec!["maple580", "south514", "delta762", "north637", "river905", "maple673", "north65", "alpha214"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple580,south514,delta762,north637,river905,maple673,north65,alpha214");
    assert_eq!(parse_record(&line), fields, "record 34 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_035() {
    let fields: Vec<String> = vec!["south301", "cobalt537", "maple191", "gamma847"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south301,cobalt537,maple191,gamma847");
    assert_eq!(parse_record(&line), fields, "record 35 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_036() {
    let fields: Vec<String> = vec!["cedar993, river37", "cedar695", "gamma592", "north269", "gamma988", "river980"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar993, river37",cedar695,gamma592,north269,gamma988,river980"#);
    assert_eq!(parse_record(&line), fields, "record 36 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_037() {
    let fields: Vec<String> = vec!["beta829", "beta853", "delta38", "gamma591", "south310", "delta694", "cobalt746"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta829,beta853,delta38,gamma591,south310,delta694,cobalt746");
    assert_eq!(parse_record(&line), fields, "record 37 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_038() {
    let fields: Vec<String> = vec!["north280", "gamma876", "delta683"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north280,gamma876,delta683");
    assert_eq!(parse_record(&line), fields, "record 38 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_039() {
    let fields: Vec<String> = vec!["south11, south399", "cedar548", "north395", "beta927", "river295", "river11"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south11, south399",cedar548,north395,beta927,river295,river11"#);
    assert_eq!(parse_record(&line), fields, "record 39 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_040() {
    let fields: Vec<String> = vec!["cedar860", "stone29", "gamma182", "river535", "beta955", "north140", "beta434"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar860,stone29,gamma182,river535,beta955,north140,beta434");
    assert_eq!(parse_record(&line), fields, "record 40 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_041() {
    let fields: Vec<String> = vec!["delta190", "cedar666"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta190,cedar666");
    assert_eq!(parse_record(&line), fields, "record 41 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_042() {
    let fields: Vec<String> = vec!["stone441, stone11", "amber74", "stone154", "maple585", "delta594", "north215"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone441, stone11",amber74,stone154,maple585,delta594,north215"#);
    assert_eq!(parse_record(&line), fields, "record 42 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_043() {
    let fields: Vec<String> = vec!["maple523", "gamma422"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple523,gamma422");
    assert_eq!(parse_record(&line), fields, "record 43 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_044() {
    let fields: Vec<String> = vec!["north820", "north684", "maple106", "river85"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north820,north684,maple106,river85");
    assert_eq!(parse_record(&line), fields, "record 44 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_045() {
    let fields: Vec<String> = vec!["south176, river758", "delta555", "amber482", "beta834", "stone463", "stone101", "north600"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south176, river758",delta555,amber482,beta834,stone463,stone101,north600"#);
    assert_eq!(parse_record(&line), fields, "record 45 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_046() {
    let fields: Vec<String> = vec!["beta455", "stone878", "amber9", "cedar756", "stone591", "cobalt591", "maple429", "delta681"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta455,stone878,amber9,cedar756,stone591,cobalt591,maple429,delta681");
    assert_eq!(parse_record(&line), fields, "record 46 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_047() {
    let fields: Vec<String> = vec!["stone383", "maple628", "cobalt217", "delta918", "cedar476"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone383,maple628,cobalt217,delta918,cedar476");
    assert_eq!(parse_record(&line), fields, "record 47 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_048() {
    let fields: Vec<String> = vec!["beta998, delta959", "river972", "stone341", "alpha283", "south668", "alpha647", "maple783"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta998, delta959",river972,stone341,alpha283,south668,alpha647,maple783"#);
    assert_eq!(parse_record(&line), fields, "record 48 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_049() {
    let fields: Vec<String> = vec!["river828", "maple768"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river828,maple768");
    assert_eq!(parse_record(&line), fields, "record 49 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_050() {
    let fields: Vec<String> = vec!["south560", "south993", "north830", "delta221", "cobalt179", "river616"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south560,south993,north830,delta221,cobalt179,river616");
    assert_eq!(parse_record(&line), fields, "record 50 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_051() {
    let fields: Vec<String> = vec!["river847, amber888", "river556"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river847, amber888",river556"#);
    assert_eq!(parse_record(&line), fields, "record 51 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_052() {
    let fields: Vec<String> = vec!["beta501", "stone814", "delta755"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta501,stone814,delta755");
    assert_eq!(parse_record(&line), fields, "record 52 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_053() {
    let fields: Vec<String> = vec!["north230", "cedar869", "south436", "north644", "beta488", "river998", "cobalt273", "amber531"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north230,cedar869,south436,north644,beta488,river998,cobalt273,amber531");
    assert_eq!(parse_record(&line), fields, "record 53 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_054() {
    let fields: Vec<String> = vec!["river244, beta290", "cobalt772", "cobalt590", "stone461", "delta13", "north937", "alpha497", "delta201"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river244, beta290",cobalt772,cobalt590,stone461,delta13,north937,alpha497,delta201"#);
    assert_eq!(parse_record(&line), fields, "record 54 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_055() {
    let fields: Vec<String> = vec!["maple607", "beta903", "delta645"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple607,beta903,delta645");
    assert_eq!(parse_record(&line), fields, "record 55 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_056() {
    let fields: Vec<String> = vec!["gamma778", "maple527", "south537", "beta886", "beta209"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma778,maple527,south537,beta886,beta209");
    assert_eq!(parse_record(&line), fields, "record 56 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_057() {
    let fields: Vec<String> = vec!["delta765, cobalt717", "delta79", "river218", "south837", "gamma692", "maple698"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta765, cobalt717",delta79,river218,south837,gamma692,maple698"#);
    assert_eq!(parse_record(&line), fields, "record 57 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_058() {
    let fields: Vec<String> = vec!["river657", "cedar209", "river120", "alpha615", "delta288", "amber199", "cedar213", "maple480"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river657,cedar209,river120,alpha615,delta288,amber199,cedar213,maple480");
    assert_eq!(parse_record(&line), fields, "record 58 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_059() {
    let fields: Vec<String> = vec!["beta481", "beta208", "delta560", "cobalt287", "stone452", "south707", "river292", "gamma938"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta481,beta208,delta560,cobalt287,stone452,south707,river292,gamma938");
    assert_eq!(parse_record(&line), fields, "record 59 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_060() {
    let fields: Vec<String> = vec!["cobalt570, stone677", "alpha259", "delta855", "maple617", "stone607", "south174"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cobalt570, stone677",alpha259,delta855,maple617,stone607,south174"#);
    assert_eq!(parse_record(&line), fields, "record 60 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_061() {
    let fields: Vec<String> = vec!["cedar714", "delta867", "gamma595", "gamma569", "maple165", "beta143", "maple917"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar714,delta867,gamma595,gamma569,maple165,beta143,maple917");
    assert_eq!(parse_record(&line), fields, "record 61 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_062() {
    let fields: Vec<String> = vec!["beta686", "maple541", "cedar364", "cedar674", "delta830", "beta446", "cedar342"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta686,maple541,cedar364,cedar674,delta830,beta446,cedar342");
    assert_eq!(parse_record(&line), fields, "record 62 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_063() {
    let fields: Vec<String> = vec!["river541, beta392", "south254", "alpha172", "maple953"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river541, beta392",south254,alpha172,maple953"#);
    assert_eq!(parse_record(&line), fields, "record 63 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_064() {
    let fields: Vec<String> = vec!["delta158", "stone719", "cedar911", "cedar754", "cedar425", "amber605", "delta382", "cedar559"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta158,stone719,cedar911,cedar754,cedar425,amber605,delta382,cedar559");
    assert_eq!(parse_record(&line), fields, "record 64 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_065() {
    let fields: Vec<String> = vec!["cobalt940", "north491", "gamma787", "amber500", "south802", "amber129"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt940,north491,gamma787,amber500,south802,amber129");
    assert_eq!(parse_record(&line), fields, "record 65 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_066() {
    let fields: Vec<String> = vec!["stone525, delta641", "river557", "north175", "south658"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone525, delta641",river557,north175,south658"#);
    assert_eq!(parse_record(&line), fields, "record 66 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_067() {
    let fields: Vec<String> = vec!["stone64", "river430", "beta570", "river489", "cedar746", "south123", "amber541", "amber215"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone64,river430,beta570,river489,cedar746,south123,amber541,amber215");
    assert_eq!(parse_record(&line), fields, "record 67 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_068() {
    let fields: Vec<String> = vec!["south572", "south117", "cedar481", "gamma385", "gamma209", "north696", "stone750", "gamma24"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south572,south117,cedar481,gamma385,gamma209,north696,stone750,gamma24");
    assert_eq!(parse_record(&line), fields, "record 68 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_069() {
    let fields: Vec<String> = vec!["beta598, beta110", "delta127", "gamma574", "delta487", "stone499", "cedar180", "stone561"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta598, beta110",delta127,gamma574,delta487,stone499,cedar180,stone561"#);
    assert_eq!(parse_record(&line), fields, "record 69 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_070() {
    let fields: Vec<String> = vec!["beta420", "delta201", "beta124", "gamma61", "south655", "amber753", "cedar57"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta420,delta201,beta124,gamma61,south655,amber753,cedar57");
    assert_eq!(parse_record(&line), fields, "record 70 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_071() {
    let fields: Vec<String> = vec!["cobalt322", "south561"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt322,south561");
    assert_eq!(parse_record(&line), fields, "record 71 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_072() {
    let fields: Vec<String> = vec!["stone456, south621", "cedar250", "south955", "amber943", "cobalt629"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone456, south621",cedar250,south955,amber943,cobalt629"#);
    assert_eq!(parse_record(&line), fields, "record 72 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_073() {
    let fields: Vec<String> = vec!["cobalt65", "gamma683", "maple600"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt65,gamma683,maple600");
    assert_eq!(parse_record(&line), fields, "record 73 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_074() {
    let fields: Vec<String> = vec!["alpha527", "cobalt415", "cedar962"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha527,cobalt415,cedar962");
    assert_eq!(parse_record(&line), fields, "record 74 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_075() {
    let fields: Vec<String> = vec!["north18, gamma190", "amber283", "gamma353"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north18, gamma190",amber283,gamma353"#);
    assert_eq!(parse_record(&line), fields, "record 75 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_076() {
    let fields: Vec<String> = vec!["north217", "north283", "delta721"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north217,north283,delta721");
    assert_eq!(parse_record(&line), fields, "record 76 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_077() {
    let fields: Vec<String> = vec!["alpha131", "gamma683"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha131,gamma683");
    assert_eq!(parse_record(&line), fields, "record 77 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_078() {
    let fields: Vec<String> = vec!["cedar469, beta161", "south774", "delta0", "cedar109", "amber710", "cobalt79", "beta698"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar469, beta161",south774,delta0,cedar109,amber710,cobalt79,beta698"#);
    assert_eq!(parse_record(&line), fields, "record 78 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_079() {
    let fields: Vec<String> = vec!["gamma586", "maple848", "stone132", "maple649", "cedar359", "maple322", "gamma332", "south803"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma586,maple848,stone132,maple649,cedar359,maple322,gamma332,south803");
    assert_eq!(parse_record(&line), fields, "record 79 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_080() {
    let fields: Vec<String> = vec!["beta541", "delta625"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta541,delta625");
    assert_eq!(parse_record(&line), fields, "record 80 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_081() {
    let fields: Vec<String> = vec!["stone158, beta464", "alpha740", "cobalt109", "maple998", "stone589", "alpha804", "north463", "alpha472"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone158, beta464",alpha740,cobalt109,maple998,stone589,alpha804,north463,alpha472"#);
    assert_eq!(parse_record(&line), fields, "record 81 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_082() {
    let fields: Vec<String> = vec!["amber930", "beta442", "river519", "beta690", "maple469"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber930,beta442,river519,beta690,maple469");
    assert_eq!(parse_record(&line), fields, "record 82 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_083() {
    let fields: Vec<String> = vec!["cedar958", "gamma337", "south141", "maple976", "north391", "alpha343", "cedar967", "north927"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar958,gamma337,south141,maple976,north391,alpha343,cedar967,north927");
    assert_eq!(parse_record(&line), fields, "record 83 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_084() {
    let fields: Vec<String> = vec!["alpha280, north417", "south161", "river705", "alpha856", "south453", "amber891", "north596", "south536"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha280, north417",south161,river705,alpha856,south453,amber891,north596,south536"#);
    assert_eq!(parse_record(&line), fields, "record 84 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_085() {
    let fields: Vec<String> = vec!["gamma979", "delta611", "cedar687", "delta554"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma979,delta611,cedar687,delta554");
    assert_eq!(parse_record(&line), fields, "record 85 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_086() {
    let fields: Vec<String> = vec!["alpha510", "cedar155", "alpha721", "amber720", "amber293", "cedar96"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha510,cedar155,alpha721,amber720,amber293,cedar96");
    assert_eq!(parse_record(&line), fields, "record 86 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_087() {
    let fields: Vec<String> = vec!["beta684, cobalt363", "alpha374"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta684, cobalt363",alpha374"#);
    assert_eq!(parse_record(&line), fields, "record 87 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_088() {
    let fields: Vec<String> = vec!["gamma707", "stone536", "stone337", "maple468", "cedar461", "cedar510"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma707,stone536,stone337,maple468,cedar461,cedar510");
    assert_eq!(parse_record(&line), fields, "record 88 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_089() {
    let fields: Vec<String> = vec!["south461", "north354", "beta599", "alpha994", "south623"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south461,north354,beta599,alpha994,south623");
    assert_eq!(parse_record(&line), fields, "record 89 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_090() {
    let fields: Vec<String> = vec!["delta382, gamma191", "stone271", "south468", "beta815", "river230", "south360", "amber142"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta382, gamma191",stone271,south468,beta815,river230,south360,amber142"#);
    assert_eq!(parse_record(&line), fields, "record 90 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_091() {
    let fields: Vec<String> = vec!["south605", "maple672", "cedar318", "gamma244", "cobalt645", "cobalt573"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south605,maple672,cedar318,gamma244,cobalt645,cobalt573");
    assert_eq!(parse_record(&line), fields, "record 91 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_092() {
    let fields: Vec<String> = vec!["north236", "amber615", "delta541", "beta147", "cedar840", "amber224", "river655", "cedar701"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north236,amber615,delta541,beta147,cedar840,amber224,river655,cedar701");
    assert_eq!(parse_record(&line), fields, "record 92 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_093() {
    let fields: Vec<String> = vec!["gamma832, river812", "south178", "north995", "river791"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma832, river812",south178,north995,river791"#);
    assert_eq!(parse_record(&line), fields, "record 93 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_094() {
    let fields: Vec<String> = vec!["alpha378", "gamma354", "alpha318", "gamma816"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha378,gamma354,alpha318,gamma816");
    assert_eq!(parse_record(&line), fields, "record 94 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_095() {
    let fields: Vec<String> = vec!["north101", "stone915", "cobalt503", "amber651", "river138", "beta784", "cedar319"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north101,stone915,cobalt503,amber651,river138,beta784,cedar319");
    assert_eq!(parse_record(&line), fields, "record 95 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_096() {
    let fields: Vec<String> = vec!["amber640, north331", "beta348"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber640, north331",beta348"#);
    assert_eq!(parse_record(&line), fields, "record 96 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_097() {
    let fields: Vec<String> = vec!["amber838", "stone710", "stone590", "maple282", "south718", "beta626", "north954", "maple977"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber838,stone710,stone590,maple282,south718,beta626,north954,maple977");
    assert_eq!(parse_record(&line), fields, "record 97 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_098() {
    let fields: Vec<String> = vec!["maple560", "alpha585", "gamma158", "stone784", "alpha341", "gamma437"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple560,alpha585,gamma158,stone784,alpha341,gamma437");
    assert_eq!(parse_record(&line), fields, "record 98 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_099() {
    let fields: Vec<String> = vec!["delta454, beta917", "stone442", "river682", "south390", "delta989", "south24", "river472"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta454, beta917",stone442,river682,south390,delta989,south24,river472"#);
    assert_eq!(parse_record(&line), fields, "record 99 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_100() {
    let fields: Vec<String> = vec!["beta33", "stone315", "cedar734"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta33,stone315,cedar734");
    assert_eq!(parse_record(&line), fields, "record 100 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_101() {
    let fields: Vec<String> = vec!["stone977", "south621", "maple381", "maple926", "gamma6"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone977,south621,maple381,maple926,gamma6");
    assert_eq!(parse_record(&line), fields, "record 101 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_102() {
    let fields: Vec<String> = vec!["river605, river352", "alpha380"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river605, river352",alpha380"#);
    assert_eq!(parse_record(&line), fields, "record 102 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_103() {
    let fields: Vec<String> = vec!["stone592", "south158", "north479", "river824", "north575", "south449"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone592,south158,north479,river824,north575,south449");
    assert_eq!(parse_record(&line), fields, "record 103 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_104() {
    let fields: Vec<String> = vec!["river679", "gamma897", "beta656", "stone534"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river679,gamma897,beta656,stone534");
    assert_eq!(parse_record(&line), fields, "record 104 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_105() {
    let fields: Vec<String> = vec!["cedar399, stone362", "beta960"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar399, stone362",beta960"#);
    assert_eq!(parse_record(&line), fields, "record 105 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_106() {
    let fields: Vec<String> = vec!["south306", "cedar724", "cedar477"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south306,cedar724,cedar477");
    assert_eq!(parse_record(&line), fields, "record 106 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_107() {
    let fields: Vec<String> = vec!["river158", "stone935", "stone135", "north406", "delta600", "delta348", "north289", "delta381"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river158,stone935,stone135,north406,delta600,delta348,north289,delta381");
    assert_eq!(parse_record(&line), fields, "record 107 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_108() {
    let fields: Vec<String> = vec!["cedar281, cedar966", "south78", "cedar713", "north688", "beta955"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar281, cedar966",south78,cedar713,north688,beta955"#);
    assert_eq!(parse_record(&line), fields, "record 108 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_109() {
    let fields: Vec<String> = vec!["cedar934", "stone45"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar934,stone45");
    assert_eq!(parse_record(&line), fields, "record 109 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_110() {
    let fields: Vec<String> = vec!["cedar18", "gamma938", "gamma219", "stone441"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar18,gamma938,gamma219,stone441");
    assert_eq!(parse_record(&line), fields, "record 110 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_111() {
    let fields: Vec<String> = vec!["beta124, gamma812", "stone337"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta124, gamma812",stone337"#);
    assert_eq!(parse_record(&line), fields, "record 111 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_112() {
    let fields: Vec<String> = vec!["cedar817", "amber603", "delta655"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar817,amber603,delta655");
    assert_eq!(parse_record(&line), fields, "record 112 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_113() {
    let fields: Vec<String> = vec!["cedar883", "stone687", "amber187", "north809"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar883,stone687,amber187,north809");
    assert_eq!(parse_record(&line), fields, "record 113 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_114() {
    let fields: Vec<String> = vec!["beta268, amber109", "north104"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta268, amber109",north104"#);
    assert_eq!(parse_record(&line), fields, "record 114 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_115() {
    let fields: Vec<String> = vec!["delta145", "cobalt53", "north306", "south0", "alpha165"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta145,cobalt53,north306,south0,alpha165");
    assert_eq!(parse_record(&line), fields, "record 115 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_116() {
    let fields: Vec<String> = vec!["gamma186", "gamma759", "river581", "maple666", "river453", "north129", "delta895", "river14"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma186,gamma759,river581,maple666,river453,north129,delta895,river14");
    assert_eq!(parse_record(&line), fields, "record 116 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_117() {
    let fields: Vec<String> = vec!["cedar737, delta253", "amber718", "maple16", "cobalt989", "gamma818", "delta664", "maple913", "beta176"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar737, delta253",amber718,maple16,cobalt989,gamma818,delta664,maple913,beta176"#);
    assert_eq!(parse_record(&line), fields, "record 117 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_118() {
    let fields: Vec<String> = vec!["maple975", "alpha434", "amber217", "cedar613", "gamma927", "river874", "cobalt866", "cedar228"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple975,alpha434,amber217,cedar613,gamma927,river874,cobalt866,cedar228");
    assert_eq!(parse_record(&line), fields, "record 118 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_119() {
    let fields: Vec<String> = vec!["alpha721", "stone82", "beta744", "stone80"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha721,stone82,beta744,stone80");
    assert_eq!(parse_record(&line), fields, "record 119 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_120() {
    let fields: Vec<String> = vec!["cedar787, river197", "north365", "delta668", "river279", "cedar974", "north856", "river213"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar787, river197",north365,delta668,river279,cedar974,north856,river213"#);
    assert_eq!(parse_record(&line), fields, "record 120 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_121() {
    let fields: Vec<String> = vec!["amber976", "delta412", "alpha672"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber976,delta412,alpha672");
    assert_eq!(parse_record(&line), fields, "record 121 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_122() {
    let fields: Vec<String> = vec!["amber345", "gamma771", "beta543", "river813", "delta901", "north598"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber345,gamma771,beta543,river813,delta901,north598");
    assert_eq!(parse_record(&line), fields, "record 122 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_123() {
    let fields: Vec<String> = vec!["river249, cobalt569", "river428"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river249, cobalt569",river428"#);
    assert_eq!(parse_record(&line), fields, "record 123 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_124() {
    let fields: Vec<String> = vec!["maple303", "beta740", "delta996", "maple929", "river509", "cedar400", "maple961"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple303,beta740,delta996,maple929,river509,cedar400,maple961");
    assert_eq!(parse_record(&line), fields, "record 124 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_125() {
    let fields: Vec<String> = vec!["river588", "north308"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river588,north308");
    assert_eq!(parse_record(&line), fields, "record 125 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_126() {
    let fields: Vec<String> = vec!["beta493, delta509", "amber296", "cobalt708", "cobalt416"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta493, delta509",amber296,cobalt708,cobalt416"#);
    assert_eq!(parse_record(&line), fields, "record 126 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_127() {
    let fields: Vec<String> = vec!["amber715", "delta354", "south565"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber715,delta354,south565");
    assert_eq!(parse_record(&line), fields, "record 127 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_128() {
    let fields: Vec<String> = vec!["alpha0", "river496", "north468", "cobalt435", "maple535"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha0,river496,north468,cobalt435,maple535");
    assert_eq!(parse_record(&line), fields, "record 128 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_129() {
    let fields: Vec<String> = vec!["delta676, amber647", "maple602"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta676, amber647",maple602"#);
    assert_eq!(parse_record(&line), fields, "record 129 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_130() {
    let fields: Vec<String> = vec!["delta393", "amber81", "cedar39", "gamma62", "cedar972", "gamma770", "cedar38"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta393,amber81,cedar39,gamma62,cedar972,gamma770,cedar38");
    assert_eq!(parse_record(&line), fields, "record 130 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_131() {
    let fields: Vec<String> = vec!["river278", "river236", "maple858", "gamma335", "beta343", "stone948", "delta277", "maple673"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river278,river236,maple858,gamma335,beta343,stone948,delta277,maple673");
    assert_eq!(parse_record(&line), fields, "record 131 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_132() {
    let fields: Vec<String> = vec!["river968, south613", "cobalt643", "gamma627", "cobalt914"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river968, south613",cobalt643,gamma627,cobalt914"#);
    assert_eq!(parse_record(&line), fields, "record 132 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_133() {
    let fields: Vec<String> = vec!["beta282", "delta982", "alpha848", "delta368", "alpha243", "amber387", "stone824", "amber662"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta282,delta982,alpha848,delta368,alpha243,amber387,stone824,amber662");
    assert_eq!(parse_record(&line), fields, "record 133 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_134() {
    let fields: Vec<String> = vec!["alpha306", "alpha777", "cedar121", "delta482"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha306,alpha777,cedar121,delta482");
    assert_eq!(parse_record(&line), fields, "record 134 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_135() {
    let fields: Vec<String> = vec!["gamma187, cedar222", "cobalt136", "river910", "alpha940", "beta552", "cobalt470", "cedar19"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma187, cedar222",cobalt136,river910,alpha940,beta552,cobalt470,cedar19"#);
    assert_eq!(parse_record(&line), fields, "record 135 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_136() {
    let fields: Vec<String> = vec!["delta740", "river639", "south863", "cedar69", "maple702", "cobalt282", "maple94", "amber708"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta740,river639,south863,cedar69,maple702,cobalt282,maple94,amber708");
    assert_eq!(parse_record(&line), fields, "record 136 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_137() {
    let fields: Vec<String> = vec!["river20", "stone8", "alpha612", "stone794"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river20,stone8,alpha612,stone794");
    assert_eq!(parse_record(&line), fields, "record 137 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_138() {
    let fields: Vec<String> = vec!["gamma932, cobalt243", "maple689", "beta178", "south995", "south990", "stone889", "south306", "south813"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma932, cobalt243",maple689,beta178,south995,south990,stone889,south306,south813"#);
    assert_eq!(parse_record(&line), fields, "record 138 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_139() {
    let fields: Vec<String> = vec!["cobalt985", "south473", "amber379", "cedar700", "maple967"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt985,south473,amber379,cedar700,maple967");
    assert_eq!(parse_record(&line), fields, "record 139 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_140() {
    let fields: Vec<String> = vec!["beta107", "delta146", "south581", "north750", "south695", "north73", "alpha887", "delta231"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta107,delta146,south581,north750,south695,north73,alpha887,delta231");
    assert_eq!(parse_record(&line), fields, "record 140 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_141() {
    let fields: Vec<String> = vec!["stone116, cedar212", "north634", "maple401", "river339", "maple267"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone116, cedar212",north634,maple401,river339,maple267"#);
    assert_eq!(parse_record(&line), fields, "record 141 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_142() {
    let fields: Vec<String> = vec!["river768", "beta760", "beta924", "cedar727", "stone477", "south715", "maple227", "cedar858"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river768,beta760,beta924,cedar727,stone477,south715,maple227,cedar858");
    assert_eq!(parse_record(&line), fields, "record 142 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_143() {
    let fields: Vec<String> = vec!["south797", "stone229", "cobalt586"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south797,stone229,cobalt586");
    assert_eq!(parse_record(&line), fields, "record 143 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_144() {
    let fields: Vec<String> = vec!["amber531, amber728", "gamma467", "river460", "gamma555", "beta659"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber531, amber728",gamma467,river460,gamma555,beta659"#);
    assert_eq!(parse_record(&line), fields, "record 144 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_145() {
    let fields: Vec<String> = vec!["beta603", "beta508", "river70"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta603,beta508,river70");
    assert_eq!(parse_record(&line), fields, "record 145 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_146() {
    let fields: Vec<String> = vec!["amber156", "beta392", "cedar705", "maple998", "north30"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber156,beta392,cedar705,maple998,north30");
    assert_eq!(parse_record(&line), fields, "record 146 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_147() {
    let fields: Vec<String> = vec!["amber935, beta203", "cobalt873", "south101", "alpha644", "beta639"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber935, beta203",cobalt873,south101,alpha644,beta639"#);
    assert_eq!(parse_record(&line), fields, "record 147 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_148() {
    let fields: Vec<String> = vec!["alpha121", "gamma607", "alpha560", "delta406", "delta875", "beta830", "river760"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha121,gamma607,alpha560,delta406,delta875,beta830,river760");
    assert_eq!(parse_record(&line), fields, "record 148 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_149() {
    let fields: Vec<String> = vec!["cobalt305", "delta673", "maple921"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt305,delta673,maple921");
    assert_eq!(parse_record(&line), fields, "record 149 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_150() {
    let fields: Vec<String> = vec!["amber868, stone126", "north385", "amber834", "delta776"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber868, stone126",north385,amber834,delta776"#);
    assert_eq!(parse_record(&line), fields, "record 150 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_151() {
    let fields: Vec<String> = vec!["river778", "maple790", "alpha550", "river403", "delta142", "cedar40"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river778,maple790,alpha550,river403,delta142,cedar40");
    assert_eq!(parse_record(&line), fields, "record 151 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_152() {
    let fields: Vec<String> = vec!["north949", "river399", "south403", "river516"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north949,river399,south403,river516");
    assert_eq!(parse_record(&line), fields, "record 152 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_153() {
    let fields: Vec<String> = vec!["river71, north266", "north937", "stone477", "cobalt54", "amber960", "maple993", "stone771"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river71, north266",north937,stone477,cobalt54,amber960,maple993,stone771"#);
    assert_eq!(parse_record(&line), fields, "record 153 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_154() {
    let fields: Vec<String> = vec!["north647", "amber202", "cedar349", "south509", "stone766", "cedar53", "delta951", "cobalt877"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north647,amber202,cedar349,south509,stone766,cedar53,delta951,cobalt877");
    assert_eq!(parse_record(&line), fields, "record 154 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_155() {
    let fields: Vec<String> = vec!["stone1", "river235", "beta38", "delta279"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone1,river235,beta38,delta279");
    assert_eq!(parse_record(&line), fields, "record 155 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_156() {
    let fields: Vec<String> = vec!["north162, gamma368", "cobalt853", "cobalt930", "beta833"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north162, gamma368",cobalt853,cobalt930,beta833"#);
    assert_eq!(parse_record(&line), fields, "record 156 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_157() {
    let fields: Vec<String> = vec!["cedar953", "cobalt645", "river874", "gamma249"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar953,cobalt645,river874,gamma249");
    assert_eq!(parse_record(&line), fields, "record 157 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_158() {
    let fields: Vec<String> = vec!["gamma451", "amber145", "cedar754", "cedar981", "amber146", "stone780"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma451,amber145,cedar754,cedar981,amber146,stone780");
    assert_eq!(parse_record(&line), fields, "record 158 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_159() {
    let fields: Vec<String> = vec!["north309, delta217", "beta175", "river187", "cobalt464", "alpha694", "delta958", "stone371"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north309, delta217",beta175,river187,cobalt464,alpha694,delta958,stone371"#);
    assert_eq!(parse_record(&line), fields, "record 159 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_160() {
    let fields: Vec<String> = vec!["alpha470", "cobalt150", "cedar84", "maple589"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha470,cobalt150,cedar84,maple589");
    assert_eq!(parse_record(&line), fields, "record 160 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_161() {
    let fields: Vec<String> = vec!["beta422", "alpha950", "cobalt179"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta422,alpha950,cobalt179");
    assert_eq!(parse_record(&line), fields, "record 161 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_162() {
    let fields: Vec<String> = vec!["amber317, gamma297", "cedar626", "amber119"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber317, gamma297",cedar626,amber119"#);
    assert_eq!(parse_record(&line), fields, "record 162 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_163() {
    let fields: Vec<String> = vec!["alpha305", "river483", "cedar210", "alpha648", "delta917", "south702", "maple948"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha305,river483,cedar210,alpha648,delta917,south702,maple948");
    assert_eq!(parse_record(&line), fields, "record 163 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_164() {
    let fields: Vec<String> = vec!["river901", "cedar606", "gamma190", "river42", "north81", "maple953", "cobalt321", "maple460"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river901,cedar606,gamma190,river42,north81,maple953,cobalt321,maple460");
    assert_eq!(parse_record(&line), fields, "record 164 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_165() {
    let fields: Vec<String> = vec!["cedar468, cobalt691", "amber574", "cobalt726", "cedar617", "amber341", "delta996", "delta914"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar468, cobalt691",amber574,cobalt726,cedar617,amber341,delta996,delta914"#);
    assert_eq!(parse_record(&line), fields, "record 165 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_166() {
    let fields: Vec<String> = vec!["beta495", "alpha108", "delta831"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta495,alpha108,delta831");
    assert_eq!(parse_record(&line), fields, "record 166 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_167() {
    let fields: Vec<String> = vec!["cedar434", "alpha214"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar434,alpha214");
    assert_eq!(parse_record(&line), fields, "record 167 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_168() {
    let fields: Vec<String> = vec!["north793, beta974", "north847", "cedar45", "stone562"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north793, beta974",north847,cedar45,stone562"#);
    assert_eq!(parse_record(&line), fields, "record 168 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_169() {
    let fields: Vec<String> = vec!["alpha16", "delta313", "delta768", "gamma754", "alpha485", "stone419"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha16,delta313,delta768,gamma754,alpha485,stone419");
    assert_eq!(parse_record(&line), fields, "record 169 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_170() {
    let fields: Vec<String> = vec!["cedar769", "amber16", "beta182", "south969", "gamma930", "maple172"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar769,amber16,beta182,south969,gamma930,maple172");
    assert_eq!(parse_record(&line), fields, "record 170 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_171() {
    let fields: Vec<String> = vec!["cedar276, amber883", "cobalt787"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar276, amber883",cobalt787"#);
    assert_eq!(parse_record(&line), fields, "record 171 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_172() {
    let fields: Vec<String> = vec!["cobalt685", "alpha80", "delta416", "cedar959", "gamma973", "river162"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt685,alpha80,delta416,cedar959,gamma973,river162");
    assert_eq!(parse_record(&line), fields, "record 172 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_173() {
    let fields: Vec<String> = vec!["cedar469", "cobalt752", "cobalt480", "amber454", "gamma381", "stone831"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar469,cobalt752,cobalt480,amber454,gamma381,stone831");
    assert_eq!(parse_record(&line), fields, "record 173 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_174() {
    let fields: Vec<String> = vec!["maple294, maple881", "gamma789", "alpha186", "delta766", "cedar589", "maple373"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple294, maple881",gamma789,alpha186,delta766,cedar589,maple373"#);
    assert_eq!(parse_record(&line), fields, "record 174 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_175() {
    let fields: Vec<String> = vec!["gamma828", "cedar66", "maple840", "cobalt171", "maple715", "river960", "alpha197", "amber66"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma828,cedar66,maple840,cobalt171,maple715,river960,alpha197,amber66");
    assert_eq!(parse_record(&line), fields, "record 175 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_176() {
    let fields: Vec<String> = vec!["river187", "beta877", "delta168", "alpha713", "amber425", "stone610", "gamma357"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river187,beta877,delta168,alpha713,amber425,stone610,gamma357");
    assert_eq!(parse_record(&line), fields, "record 176 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_177() {
    let fields: Vec<String> = vec!["gamma186, gamma756", "cedar409"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma186, gamma756",cedar409"#);
    assert_eq!(parse_record(&line), fields, "record 177 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_178() {
    let fields: Vec<String> = vec!["north470", "south563", "alpha620", "south548"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north470,south563,alpha620,south548");
    assert_eq!(parse_record(&line), fields, "record 178 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_179() {
    let fields: Vec<String> = vec!["alpha82", "stone181", "maple928"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha82,stone181,maple928");
    assert_eq!(parse_record(&line), fields, "record 179 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_180() {
    let fields: Vec<String> = vec!["stone447, cobalt139", "south769", "cedar918", "stone164", "stone946", "gamma460", "beta783"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone447, cobalt139",south769,cedar918,stone164,stone946,gamma460,beta783"#);
    assert_eq!(parse_record(&line), fields, "record 180 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_181() {
    let fields: Vec<String> = vec!["north69", "amber146", "amber352", "gamma300", "south825", "south598", "cobalt747"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north69,amber146,amber352,gamma300,south825,south598,cobalt747");
    assert_eq!(parse_record(&line), fields, "record 181 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_182() {
    let fields: Vec<String> = vec!["alpha69", "river38", "cobalt993", "alpha995", "gamma74"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha69,river38,cobalt993,alpha995,gamma74");
    assert_eq!(parse_record(&line), fields, "record 182 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_183() {
    let fields: Vec<String> = vec!["north666, south892", "river662", "gamma834", "cedar734", "river50", "north415", "north598", "beta6"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north666, south892",river662,gamma834,cedar734,river50,north415,north598,beta6"#);
    assert_eq!(parse_record(&line), fields, "record 183 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_184() {
    let fields: Vec<String> = vec!["alpha289", "stone610", "maple989", "alpha306", "cobalt933", "stone375", "beta31", "stone105"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha289,stone610,maple989,alpha306,cobalt933,stone375,beta31,stone105");
    assert_eq!(parse_record(&line), fields, "record 184 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_185() {
    let fields: Vec<String> = vec!["river639", "cobalt3", "gamma378", "north659", "amber669", "stone305"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river639,cobalt3,gamma378,north659,amber669,stone305");
    assert_eq!(parse_record(&line), fields, "record 185 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_186() {
    let fields: Vec<String> = vec!["cedar989, south104", "beta534", "amber716", "beta786", "alpha40", "south357", "maple95", "delta114"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar989, south104",beta534,amber716,beta786,alpha40,south357,maple95,delta114"#);
    assert_eq!(parse_record(&line), fields, "record 186 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_187() {
    let fields: Vec<String> = vec!["north908", "alpha484", "delta219", "river441", "alpha642", "south278", "maple769", "cedar470"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north908,alpha484,delta219,river441,alpha642,south278,maple769,cedar470");
    assert_eq!(parse_record(&line), fields, "record 187 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_188() {
    let fields: Vec<String> = vec!["river733", "stone476", "maple821", "cedar82", "alpha959", "south189", "alpha983"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river733,stone476,maple821,cedar82,alpha959,south189,alpha983");
    assert_eq!(parse_record(&line), fields, "record 188 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_189() {
    let fields: Vec<String> = vec!["beta482, north552", "river502", "maple608"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta482, north552",river502,maple608"#);
    assert_eq!(parse_record(&line), fields, "record 189 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_190() {
    let fields: Vec<String> = vec!["north811", "delta370", "amber707", "cedar147", "beta458", "stone717", "delta396"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north811,delta370,amber707,cedar147,beta458,stone717,delta396");
    assert_eq!(parse_record(&line), fields, "record 190 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_191() {
    let fields: Vec<String> = vec!["river484", "delta315", "cobalt651"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river484,delta315,cobalt651");
    assert_eq!(parse_record(&line), fields, "record 191 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_192() {
    let fields: Vec<String> = vec!["maple304, maple840", "cobalt564", "amber656", "river410", "stone825", "gamma671"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple304, maple840",cobalt564,amber656,river410,stone825,gamma671"#);
    assert_eq!(parse_record(&line), fields, "record 192 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_193() {
    let fields: Vec<String> = vec!["north726", "gamma701", "maple140"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north726,gamma701,maple140");
    assert_eq!(parse_record(&line), fields, "record 193 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_194() {
    let fields: Vec<String> = vec!["south85", "alpha519", "cobalt211"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south85,alpha519,cobalt211");
    assert_eq!(parse_record(&line), fields, "record 194 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_195() {
    let fields: Vec<String> = vec!["gamma29, cobalt512", "maple621"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma29, cobalt512",maple621"#);
    assert_eq!(parse_record(&line), fields, "record 195 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_196() {
    let fields: Vec<String> = vec!["north483", "cedar168"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north483,cedar168");
    assert_eq!(parse_record(&line), fields, "record 196 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_197() {
    let fields: Vec<String> = vec!["cobalt666", "north660", "cedar249", "cobalt901", "beta513", "amber841", "river378"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt666,north660,cedar249,cobalt901,beta513,amber841,river378");
    assert_eq!(parse_record(&line), fields, "record 197 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_198() {
    let fields: Vec<String> = vec!["cedar817, stone68", "stone474"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar817, stone68",stone474"#);
    assert_eq!(parse_record(&line), fields, "record 198 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_199() {
    let fields: Vec<String> = vec!["cedar937", "gamma582", "cobalt881", "amber691"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar937,gamma582,cobalt881,amber691");
    assert_eq!(parse_record(&line), fields, "record 199 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_200() {
    let fields: Vec<String> = vec!["beta307", "amber770", "alpha516", "maple630"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta307,amber770,alpha516,maple630");
    assert_eq!(parse_record(&line), fields, "record 200 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_201() {
    let fields: Vec<String> = vec!["delta811, beta124", "south583"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta811, beta124",south583"#);
    assert_eq!(parse_record(&line), fields, "record 201 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_202() {
    let fields: Vec<String> = vec!["maple687", "delta776", "amber529", "river380", "beta507", "river50", "beta881", "delta435"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple687,delta776,amber529,river380,beta507,river50,beta881,delta435");
    assert_eq!(parse_record(&line), fields, "record 202 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_203() {
    let fields: Vec<String> = vec!["maple286", "cedar429", "amber298", "stone741", "gamma584", "north911", "alpha458", "delta451"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple286,cedar429,amber298,stone741,gamma584,north911,alpha458,delta451");
    assert_eq!(parse_record(&line), fields, "record 203 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_204() {
    let fields: Vec<String> = vec!["cedar548, cobalt856", "cobalt836", "cobalt25", "cobalt36", "cedar48", "north977", "river186"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar548, cobalt856",cobalt836,cobalt25,cobalt36,cedar48,north977,river186"#);
    assert_eq!(parse_record(&line), fields, "record 204 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_205() {
    let fields: Vec<String> = vec!["amber429", "south12", "river99", "amber870"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber429,south12,river99,amber870");
    assert_eq!(parse_record(&line), fields, "record 205 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_206() {
    let fields: Vec<String> = vec!["gamma117", "cedar230", "river714", "river992", "north346"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma117,cedar230,river714,river992,north346");
    assert_eq!(parse_record(&line), fields, "record 206 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_207() {
    let fields: Vec<String> = vec!["beta905, stone763", "beta717", "amber542", "south997"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta905, stone763",beta717,amber542,south997"#);
    assert_eq!(parse_record(&line), fields, "record 207 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_208() {
    let fields: Vec<String> = vec!["amber329", "cedar573", "amber958", "delta501", "cedar33", "south882", "gamma464", "south437"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber329,cedar573,amber958,delta501,cedar33,south882,gamma464,south437");
    assert_eq!(parse_record(&line), fields, "record 208 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_209() {
    let fields: Vec<String> = vec!["cedar174", "beta370", "cedar899"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar174,beta370,cedar899");
    assert_eq!(parse_record(&line), fields, "record 209 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_210() {
    let fields: Vec<String> = vec!["delta651, south612", "delta203"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta651, south612",delta203"#);
    assert_eq!(parse_record(&line), fields, "record 210 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_211() {
    let fields: Vec<String> = vec!["south29", "cobalt979", "cedar832", "cedar208", "cobalt402", "cobalt189", "alpha381"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south29,cobalt979,cedar832,cedar208,cobalt402,cobalt189,alpha381");
    assert_eq!(parse_record(&line), fields, "record 211 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_212() {
    let fields: Vec<String> = vec!["south709", "south515", "south646", "cobalt100", "maple333", "north610", "cedar937", "cobalt541"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south709,south515,south646,cobalt100,maple333,north610,cedar937,cobalt541");
    assert_eq!(parse_record(&line), fields, "record 212 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_213() {
    let fields: Vec<String> = vec!["stone3, maple98", "amber635", "gamma87"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone3, maple98",amber635,gamma87"#);
    assert_eq!(parse_record(&line), fields, "record 213 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_214() {
    let fields: Vec<String> = vec!["south610", "river736", "beta924"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south610,river736,beta924");
    assert_eq!(parse_record(&line), fields, "record 214 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_215() {
    let fields: Vec<String> = vec!["river426", "beta978", "stone105", "cedar558", "delta428"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river426,beta978,stone105,cedar558,delta428");
    assert_eq!(parse_record(&line), fields, "record 215 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_216() {
    let fields: Vec<String> = vec!["beta633, cedar499", "cobalt7", "beta223", "delta713"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta633, cedar499",cobalt7,beta223,delta713"#);
    assert_eq!(parse_record(&line), fields, "record 216 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_217() {
    let fields: Vec<String> = vec!["river756", "beta123", "cobalt191", "north345", "maple392", "maple964", "stone956"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river756,beta123,cobalt191,north345,maple392,maple964,stone956");
    assert_eq!(parse_record(&line), fields, "record 217 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_218() {
    let fields: Vec<String> = vec!["cobalt125", "amber637"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt125,amber637");
    assert_eq!(parse_record(&line), fields, "record 218 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_219() {
    let fields: Vec<String> = vec!["south103, maple271", "delta287"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south103, maple271",delta287"#);
    assert_eq!(parse_record(&line), fields, "record 219 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_220() {
    let fields: Vec<String> = vec!["river150", "cobalt129"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river150,cobalt129");
    assert_eq!(parse_record(&line), fields, "record 220 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_221() {
    let fields: Vec<String> = vec!["gamma100", "beta884", "north206"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma100,beta884,north206");
    assert_eq!(parse_record(&line), fields, "record 221 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_222() {
    let fields: Vec<String> = vec!["maple259, south575", "delta544", "cedar523", "beta133"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple259, south575",delta544,cedar523,beta133"#);
    assert_eq!(parse_record(&line), fields, "record 222 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_223() {
    let fields: Vec<String> = vec!["beta30", "stone801", "cedar633", "maple244", "maple231", "delta847", "river707", "river647"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta30,stone801,cedar633,maple244,maple231,delta847,river707,river647");
    assert_eq!(parse_record(&line), fields, "record 223 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_224() {
    let fields: Vec<String> = vec!["beta454", "cobalt414", "cobalt32", "river52", "river867", "cedar656", "gamma212", "beta554"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta454,cobalt414,cobalt32,river52,river867,cedar656,gamma212,beta554");
    assert_eq!(parse_record(&line), fields, "record 224 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_225() {
    let fields: Vec<String> = vec!["delta34, stone189", "maple487", "cedar200", "amber408", "south712"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta34, stone189",maple487,cedar200,amber408,south712"#);
    assert_eq!(parse_record(&line), fields, "record 225 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_226() {
    let fields: Vec<String> = vec!["maple682", "maple762", "north734", "south951"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple682,maple762,north734,south951");
    assert_eq!(parse_record(&line), fields, "record 226 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_227() {
    let fields: Vec<String> = vec!["stone516", "alpha326", "cobalt695", "maple415", "beta764", "gamma168", "maple594", "alpha886"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone516,alpha326,cobalt695,maple415,beta764,gamma168,maple594,alpha886");
    assert_eq!(parse_record(&line), fields, "record 227 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_228() {
    let fields: Vec<String> = vec!["maple811, cedar411", "cobalt229", "north29", "delta267"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple811, cedar411",cobalt229,north29,delta267"#);
    assert_eq!(parse_record(&line), fields, "record 228 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_229() {
    let fields: Vec<String> = vec!["cobalt598", "beta984"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt598,beta984");
    assert_eq!(parse_record(&line), fields, "record 229 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_230() {
    let fields: Vec<String> = vec!["beta373", "amber672"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta373,amber672");
    assert_eq!(parse_record(&line), fields, "record 230 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_231() {
    let fields: Vec<String> = vec!["river623, north320", "delta918", "south375", "beta342"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river623, north320",delta918,south375,beta342"#);
    assert_eq!(parse_record(&line), fields, "record 231 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_232() {
    let fields: Vec<String> = vec!["cobalt657", "south1"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt657,south1");
    assert_eq!(parse_record(&line), fields, "record 232 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_233() {
    let fields: Vec<String> = vec!["beta421", "amber119", "south540", "delta516", "north660"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta421,amber119,south540,delta516,north660");
    assert_eq!(parse_record(&line), fields, "record 233 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_234() {
    let fields: Vec<String> = vec!["cobalt956, cobalt681", "delta32", "delta539", "amber510", "amber473", "maple70"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cobalt956, cobalt681",delta32,delta539,amber510,amber473,maple70"#);
    assert_eq!(parse_record(&line), fields, "record 234 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_235() {
    let fields: Vec<String> = vec!["north487", "beta581", "stone65", "cobalt533", "river471", "maple5"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north487,beta581,stone65,cobalt533,river471,maple5");
    assert_eq!(parse_record(&line), fields, "record 235 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_236() {
    let fields: Vec<String> = vec!["south19", "cobalt45", "cobalt331", "maple10", "stone468", "gamma650", "delta783"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south19,cobalt45,cobalt331,maple10,stone468,gamma650,delta783");
    assert_eq!(parse_record(&line), fields, "record 236 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_237() {
    let fields: Vec<String> = vec!["south892, stone906", "alpha870"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south892, stone906",alpha870"#);
    assert_eq!(parse_record(&line), fields, "record 237 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_238() {
    let fields: Vec<String> = vec!["alpha629", "north753", "stone730", "south346", "river7", "maple545", "gamma551"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha629,north753,stone730,south346,river7,maple545,gamma551");
    assert_eq!(parse_record(&line), fields, "record 238 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_239() {
    let fields: Vec<String> = vec!["stone297", "gamma170", "stone582", "gamma769", "north562", "cobalt800"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone297,gamma170,stone582,gamma769,north562,cobalt800");
    assert_eq!(parse_record(&line), fields, "record 239 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_240() {
    let fields: Vec<String> = vec!["north654, maple621", "maple288", "gamma567", "alpha350", "delta108", "gamma947"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north654, maple621",maple288,gamma567,alpha350,delta108,gamma947"#);
    assert_eq!(parse_record(&line), fields, "record 240 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_241() {
    let fields: Vec<String> = vec!["north738", "south152"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north738,south152");
    assert_eq!(parse_record(&line), fields, "record 241 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_242() {
    let fields: Vec<String> = vec!["south953", "south70", "beta774", "alpha899"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south953,south70,beta774,alpha899");
    assert_eq!(parse_record(&line), fields, "record 242 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_243() {
    let fields: Vec<String> = vec!["beta708, amber97", "north417", "maple23"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta708, amber97",north417,maple23"#);
    assert_eq!(parse_record(&line), fields, "record 243 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_244() {
    let fields: Vec<String> = vec!["south187", "cedar733", "alpha683"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south187,cedar733,alpha683");
    assert_eq!(parse_record(&line), fields, "record 244 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_245() {
    let fields: Vec<String> = vec!["cedar205", "maple180", "delta58", "alpha684", "amber295", "river38", "delta292", "cedar609"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar205,maple180,delta58,alpha684,amber295,river38,delta292,cedar609");
    assert_eq!(parse_record(&line), fields, "record 245 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_246() {
    let fields: Vec<String> = vec!["delta648, stone806", "alpha827", "stone509", "delta87", "beta952", "alpha443", "delta708"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""delta648, stone806",alpha827,stone509,delta87,beta952,alpha443,delta708"#);
    assert_eq!(parse_record(&line), fields, "record 246 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_247() {
    let fields: Vec<String> = vec!["maple813", "river311", "south678"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple813,river311,south678");
    assert_eq!(parse_record(&line), fields, "record 247 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_248() {
    let fields: Vec<String> = vec!["river706", "cobalt591", "alpha514", "cobalt746", "north498", "river7", "amber491", "south779"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river706,cobalt591,alpha514,cobalt746,north498,river7,amber491,south779");
    assert_eq!(parse_record(&line), fields, "record 248 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_249() {
    let fields: Vec<String> = vec!["beta409, maple863", "cobalt637", "north29", "north973", "maple401", "south315", "north96"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta409, maple863",cobalt637,north29,north973,maple401,south315,north96"#);
    assert_eq!(parse_record(&line), fields, "record 249 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_250() {
    let fields: Vec<String> = vec!["cobalt40", "amber233"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt40,amber233");
    assert_eq!(parse_record(&line), fields, "record 250 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_251() {
    let fields: Vec<String> = vec!["gamma981", "cedar439", "south139", "alpha143"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma981,cedar439,south139,alpha143");
    assert_eq!(parse_record(&line), fields, "record 251 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_252() {
    let fields: Vec<String> = vec!["alpha276, delta419", "cedar795", "gamma193", "delta463"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha276, delta419",cedar795,gamma193,delta463"#);
    assert_eq!(parse_record(&line), fields, "record 252 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_253() {
    let fields: Vec<String> = vec!["south294", "north673", "beta188"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south294,north673,beta188");
    assert_eq!(parse_record(&line), fields, "record 253 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_254() {
    let fields: Vec<String> = vec!["amber713", "gamma756", "south360", "maple631"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber713,gamma756,south360,maple631");
    assert_eq!(parse_record(&line), fields, "record 254 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_255() {
    let fields: Vec<String> = vec!["stone535, amber876", "north35", "cedar268"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone535, amber876",north35,cedar268"#);
    assert_eq!(parse_record(&line), fields, "record 255 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_256() {
    let fields: Vec<String> = vec!["alpha190", "cedar679", "alpha665", "beta238", "maple782", "north455"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha190,cedar679,alpha665,beta238,maple782,north455");
    assert_eq!(parse_record(&line), fields, "record 256 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_257() {
    let fields: Vec<String> = vec!["cobalt761", "south583", "alpha998"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt761,south583,alpha998");
    assert_eq!(parse_record(&line), fields, "record 257 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_258() {
    let fields: Vec<String> = vec!["amber685, amber436", "maple753", "alpha210", "south623", "beta823", "stone755"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber685, amber436",maple753,alpha210,south623,beta823,stone755"#);
    assert_eq!(parse_record(&line), fields, "record 258 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_259() {
    let fields: Vec<String> = vec!["delta613", "beta5", "gamma348", "maple299", "cedar142", "maple749", "cobalt544", "delta492"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta613,beta5,gamma348,maple299,cedar142,maple749,cobalt544,delta492");
    assert_eq!(parse_record(&line), fields, "record 259 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_260() {
    let fields: Vec<String> = vec!["south716", "beta591"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south716,beta591");
    assert_eq!(parse_record(&line), fields, "record 260 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_261() {
    let fields: Vec<String> = vec!["alpha183, north473", "beta723", "alpha739"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha183, north473",beta723,alpha739"#);
    assert_eq!(parse_record(&line), fields, "record 261 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_262() {
    let fields: Vec<String> = vec!["maple218", "river180", "beta668", "delta287", "amber742", "maple231"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple218,river180,beta668,delta287,amber742,maple231");
    assert_eq!(parse_record(&line), fields, "record 262 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_263() {
    let fields: Vec<String> = vec!["stone68", "north58", "maple165", "stone212", "river46", "maple909", "amber872", "delta762"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone68,north58,maple165,stone212,river46,maple909,amber872,delta762");
    assert_eq!(parse_record(&line), fields, "record 263 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_264() {
    let fields: Vec<String> = vec!["stone518, north816", "maple771", "gamma100", "cedar279", "amber846", "river1", "alpha70", "river702"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone518, north816",maple771,gamma100,cedar279,amber846,river1,alpha70,river702"#);
    assert_eq!(parse_record(&line), fields, "record 264 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_265() {
    let fields: Vec<String> = vec!["cedar920", "alpha792", "beta808", "beta844"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar920,alpha792,beta808,beta844");
    assert_eq!(parse_record(&line), fields, "record 265 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_266() {
    let fields: Vec<String> = vec!["beta289", "stone753"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta289,stone753");
    assert_eq!(parse_record(&line), fields, "record 266 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_267() {
    let fields: Vec<String> = vec!["south407, north833", "gamma468", "cedar46", "north30"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south407, north833",gamma468,cedar46,north30"#);
    assert_eq!(parse_record(&line), fields, "record 267 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_268() {
    let fields: Vec<String> = vec!["south554", "cedar746", "delta221", "maple255", "delta269", "gamma348", "amber856", "delta548"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south554,cedar746,delta221,maple255,delta269,gamma348,amber856,delta548");
    assert_eq!(parse_record(&line), fields, "record 268 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_269() {
    let fields: Vec<String> = vec!["cobalt49", "stone76", "maple759"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt49,stone76,maple759");
    assert_eq!(parse_record(&line), fields, "record 269 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_270() {
    let fields: Vec<String> = vec!["alpha369, alpha534", "beta322", "delta665", "cobalt932"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha369, alpha534",beta322,delta665,cobalt932"#);
    assert_eq!(parse_record(&line), fields, "record 270 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_271() {
    let fields: Vec<String> = vec!["gamma577", "stone732", "stone586", "stone27", "cobalt166", "river172", "delta184"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma577,stone732,stone586,stone27,cobalt166,river172,delta184");
    assert_eq!(parse_record(&line), fields, "record 271 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_272() {
    let fields: Vec<String> = vec!["gamma183", "river825", "cedar206", "delta482", "cobalt53", "cobalt856"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma183,river825,cedar206,delta482,cobalt53,cobalt856");
    assert_eq!(parse_record(&line), fields, "record 272 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_273() {
    let fields: Vec<String> = vec!["stone811, amber904", "alpha781", "maple999"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone811, amber904",alpha781,maple999"#);
    assert_eq!(parse_record(&line), fields, "record 273 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_274() {
    let fields: Vec<String> = vec!["beta118", "south623", "north814", "alpha861", "stone965", "beta357", "alpha422"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta118,south623,north814,alpha861,stone965,beta357,alpha422");
    assert_eq!(parse_record(&line), fields, "record 274 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_275() {
    let fields: Vec<String> = vec!["north617", "gamma621", "cobalt225", "amber45", "stone523", "maple413", "alpha14"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north617,gamma621,cobalt225,amber45,stone523,maple413,alpha14");
    assert_eq!(parse_record(&line), fields, "record 275 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_276() {
    let fields: Vec<String> = vec!["cedar826, amber995", "stone691", "maple89", "north540", "beta494", "delta222"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar826, amber995",stone691,maple89,north540,beta494,delta222"#);
    assert_eq!(parse_record(&line), fields, "record 276 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_277() {
    let fields: Vec<String> = vec!["maple27", "river184", "river354", "north795", "cedar876", "beta930", "gamma226", "cedar542"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple27,river184,river354,north795,cedar876,beta930,gamma226,cedar542");
    assert_eq!(parse_record(&line), fields, "record 277 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_278() {
    let fields: Vec<String> = vec!["beta112", "amber404", "gamma902", "alpha459", "river613", "gamma7", "river98"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta112,amber404,gamma902,alpha459,river613,gamma7,river98");
    assert_eq!(parse_record(&line), fields, "record 278 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_279() {
    let fields: Vec<String> = vec!["north300, alpha572", "stone255", "river750", "north499", "delta478", "cobalt484", "beta800", "cobalt75"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north300, alpha572",stone255,river750,north499,delta478,cobalt484,beta800,cobalt75"#);
    assert_eq!(parse_record(&line), fields, "record 279 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_280() {
    let fields: Vec<String> = vec!["river213", "gamma343"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river213,gamma343");
    assert_eq!(parse_record(&line), fields, "record 280 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_281() {
    let fields: Vec<String> = vec!["gamma596", "beta355", "delta981", "north436", "alpha434", "amber299", "alpha772", "alpha188"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma596,beta355,delta981,north436,alpha434,amber299,alpha772,alpha188");
    assert_eq!(parse_record(&line), fields, "record 281 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_282() {
    let fields: Vec<String> = vec!["south429, cobalt251", "river681", "delta327"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south429, cobalt251",river681,delta327"#);
    assert_eq!(parse_record(&line), fields, "record 282 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_283() {
    let fields: Vec<String> = vec!["alpha674", "amber534"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha674,amber534");
    assert_eq!(parse_record(&line), fields, "record 283 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_284() {
    let fields: Vec<String> = vec!["alpha836", "cobalt327", "south728", "beta314", "river290", "north656", "south321"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha836,cobalt327,south728,beta314,river290,north656,south321");
    assert_eq!(parse_record(&line), fields, "record 284 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_285() {
    let fields: Vec<String> = vec!["river788, cobalt690", "cedar170", "south794", "river974"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river788, cobalt690",cedar170,south794,river974"#);
    assert_eq!(parse_record(&line), fields, "record 285 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_286() {
    let fields: Vec<String> = vec!["delta703", "stone833", "north484", "south835", "beta307", "amber833", "amber564"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta703,stone833,north484,south835,beta307,amber833,amber564");
    assert_eq!(parse_record(&line), fields, "record 286 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_287() {
    let fields: Vec<String> = vec!["delta190", "maple506", "gamma704", "delta23"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta190,maple506,gamma704,delta23");
    assert_eq!(parse_record(&line), fields, "record 287 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_288() {
    let fields: Vec<String> = vec!["maple739, river954", "stone834", "river930", "gamma900", "maple401", "cedar460", "alpha39", "beta91"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple739, river954",stone834,river930,gamma900,maple401,cedar460,alpha39,beta91"#);
    assert_eq!(parse_record(&line), fields, "record 288 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_289() {
    let fields: Vec<String> = vec!["maple97", "gamma685", "amber265", "gamma128", "gamma337", "beta840"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple97,gamma685,amber265,gamma128,gamma337,beta840");
    assert_eq!(parse_record(&line), fields, "record 289 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_290() {
    let fields: Vec<String> = vec!["cedar297", "south898"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar297,south898");
    assert_eq!(parse_record(&line), fields, "record 290 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_291() {
    let fields: Vec<String> = vec!["south821, south79", "river559", "delta840", "stone237", "gamma344", "south423", "cobalt774"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south821, south79",river559,delta840,stone237,gamma344,south423,cobalt774"#);
    assert_eq!(parse_record(&line), fields, "record 291 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_292() {
    let fields: Vec<String> = vec!["alpha770", "maple231", "south914"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha770,maple231,south914");
    assert_eq!(parse_record(&line), fields, "record 292 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_293() {
    let fields: Vec<String> = vec!["cobalt68", "beta91"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt68,beta91");
    assert_eq!(parse_record(&line), fields, "record 293 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_294() {
    let fields: Vec<String> = vec!["beta10, cobalt616", "cedar959"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta10, cobalt616",cedar959"#);
    assert_eq!(parse_record(&line), fields, "record 294 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_295() {
    let fields: Vec<String> = vec!["gamma809", "north22", "alpha892", "gamma28", "maple517"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma809,north22,alpha892,gamma28,maple517");
    assert_eq!(parse_record(&line), fields, "record 295 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_296() {
    let fields: Vec<String> = vec!["delta587", "cobalt572", "cedar936", "river224"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta587,cobalt572,cedar936,river224");
    assert_eq!(parse_record(&line), fields, "record 296 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_297() {
    let fields: Vec<String> = vec!["alpha503, amber962", "gamma379", "stone876", "gamma781", "south45", "delta863"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha503, amber962",gamma379,stone876,gamma781,south45,delta863"#);
    assert_eq!(parse_record(&line), fields, "record 297 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_298() {
    let fields: Vec<String> = vec!["gamma338", "alpha360", "amber754", "beta855", "maple24"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma338,alpha360,amber754,beta855,maple24");
    assert_eq!(parse_record(&line), fields, "record 298 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_299() {
    let fields: Vec<String> = vec!["north239", "gamma313"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north239,gamma313");
    assert_eq!(parse_record(&line), fields, "record 299 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_300() {
    let fields: Vec<String> = vec!["north608, delta354", "delta70"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north608, delta354",delta70"#);
    assert_eq!(parse_record(&line), fields, "record 300 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_301() {
    let fields: Vec<String> = vec!["river23", "amber710", "amber770"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river23,amber710,amber770");
    assert_eq!(parse_record(&line), fields, "record 301 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_302() {
    let fields: Vec<String> = vec!["maple950", "maple215", "north389", "gamma412", "delta429", "maple498", "south796", "delta645"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple950,maple215,north389,gamma412,delta429,maple498,south796,delta645");
    assert_eq!(parse_record(&line), fields, "record 302 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_303() {
    let fields: Vec<String> = vec!["alpha546, river592", "amber112", "alpha370", "maple903", "stone351", "north881", "gamma163"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha546, river592",amber112,alpha370,maple903,stone351,north881,gamma163"#);
    assert_eq!(parse_record(&line), fields, "record 303 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_304() {
    let fields: Vec<String> = vec!["amber74", "river226", "gamma946", "beta74", "amber579", "beta855", "amber60"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber74,river226,gamma946,beta74,amber579,beta855,amber60");
    assert_eq!(parse_record(&line), fields, "record 304 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_305() {
    let fields: Vec<String> = vec!["amber73", "amber639", "north767", "river361", "amber646"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber73,amber639,north767,river361,amber646");
    assert_eq!(parse_record(&line), fields, "record 305 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_306() {
    let fields: Vec<String> = vec!["beta665, delta70", "amber777", "cedar787", "alpha652", "cobalt334"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta665, delta70",amber777,cedar787,alpha652,cobalt334"#);
    assert_eq!(parse_record(&line), fields, "record 306 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_307() {
    let fields: Vec<String> = vec!["cobalt760", "river629"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt760,river629");
    assert_eq!(parse_record(&line), fields, "record 307 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_308() {
    let fields: Vec<String> = vec!["beta718", "maple482"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta718,maple482");
    assert_eq!(parse_record(&line), fields, "record 308 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_309() {
    let fields: Vec<String> = vec!["amber825, maple288", "cobalt652", "south871", "cobalt8", "cedar878", "river919", "cedar908", "north206"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber825, maple288",cobalt652,south871,cobalt8,cedar878,river919,cedar908,north206"#);
    assert_eq!(parse_record(&line), fields, "record 309 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_310() {
    let fields: Vec<String> = vec!["north643", "river397", "alpha586", "river494", "alpha716"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north643,river397,alpha586,river494,alpha716");
    assert_eq!(parse_record(&line), fields, "record 310 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_311() {
    let fields: Vec<String> = vec!["delta416", "south775", "stone669", "beta618"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta416,south775,stone669,beta618");
    assert_eq!(parse_record(&line), fields, "record 311 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_312() {
    let fields: Vec<String> = vec!["cedar809, south91", "beta11", "maple961", "beta560", "north339"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar809, south91",beta11,maple961,beta560,north339"#);
    assert_eq!(parse_record(&line), fields, "record 312 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_313() {
    let fields: Vec<String> = vec!["river879", "river721", "delta496", "alpha707"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river879,river721,delta496,alpha707");
    assert_eq!(parse_record(&line), fields, "record 313 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_314() {
    let fields: Vec<String> = vec!["gamma958", "cedar743"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma958,cedar743");
    assert_eq!(parse_record(&line), fields, "record 314 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_315() {
    let fields: Vec<String> = vec!["stone12, gamma249", "alpha732"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone12, gamma249",alpha732"#);
    assert_eq!(parse_record(&line), fields, "record 315 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_316() {
    let fields: Vec<String> = vec!["beta181", "gamma34"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta181,gamma34");
    assert_eq!(parse_record(&line), fields, "record 316 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_317() {
    let fields: Vec<String> = vec!["gamma440", "maple370", "cedar631", "north35", "maple415"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma440,maple370,cedar631,north35,maple415");
    assert_eq!(parse_record(&line), fields, "record 317 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_318() {
    let fields: Vec<String> = vec!["south187, maple243", "maple719", "maple522", "gamma464", "amber212", "amber842", "river488"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south187, maple243",maple719,maple522,gamma464,amber212,amber842,river488"#);
    assert_eq!(parse_record(&line), fields, "record 318 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_319() {
    let fields: Vec<String> = vec!["amber847", "stone540", "north987", "alpha570"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber847,stone540,north987,alpha570");
    assert_eq!(parse_record(&line), fields, "record 319 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_320() {
    let fields: Vec<String> = vec!["maple55", "cedar306", "stone572", "maple48", "delta143", "cedar690"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple55,cedar306,stone572,maple48,delta143,cedar690");
    assert_eq!(parse_record(&line), fields, "record 320 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_321() {
    let fields: Vec<String> = vec!["alpha94, north808", "alpha754", "beta595"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha94, north808",alpha754,beta595"#);
    assert_eq!(parse_record(&line), fields, "record 321 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_322() {
    let fields: Vec<String> = vec!["beta819", "delta475", "maple780", "stone514", "south637", "amber133", "delta942"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta819,delta475,maple780,stone514,south637,amber133,delta942");
    assert_eq!(parse_record(&line), fields, "record 322 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_323() {
    let fields: Vec<String> = vec!["amber174", "gamma301", "north658", "amber344", "cobalt528", "amber987", "cedar396", "south722"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber174,gamma301,north658,amber344,cobalt528,amber987,cedar396,south722");
    assert_eq!(parse_record(&line), fields, "record 323 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_324() {
    let fields: Vec<String> = vec!["beta394, north88", "stone757"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta394, north88",stone757"#);
    assert_eq!(parse_record(&line), fields, "record 324 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_325() {
    let fields: Vec<String> = vec!["cobalt678", "alpha856", "south79"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt678,alpha856,south79");
    assert_eq!(parse_record(&line), fields, "record 325 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_326() {
    let fields: Vec<String> = vec!["south646", "river466", "south655", "south186"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south646,river466,south655,south186");
    assert_eq!(parse_record(&line), fields, "record 326 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_327() {
    let fields: Vec<String> = vec!["amber646, amber128", "cedar901", "delta685", "south713"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber646, amber128",cedar901,delta685,south713"#);
    assert_eq!(parse_record(&line), fields, "record 327 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_328() {
    let fields: Vec<String> = vec!["north322", "amber10", "alpha762", "alpha783"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north322,amber10,alpha762,alpha783");
    assert_eq!(parse_record(&line), fields, "record 328 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_329() {
    let fields: Vec<String> = vec!["cedar263", "cedar925", "amber78", "south988", "cedar647", "delta279", "beta596"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar263,cedar925,amber78,south988,cedar647,delta279,beta596");
    assert_eq!(parse_record(&line), fields, "record 329 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_330() {
    let fields: Vec<String> = vec!["beta147, south936", "amber337", "maple127", "delta161", "cedar998", "cobalt353", "stone968"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta147, south936",amber337,maple127,delta161,cedar998,cobalt353,stone968"#);
    assert_eq!(parse_record(&line), fields, "record 330 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_331() {
    let fields: Vec<String> = vec!["gamma949", "north972", "south657", "river928"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma949,north972,south657,river928");
    assert_eq!(parse_record(&line), fields, "record 331 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_332() {
    let fields: Vec<String> = vec!["river196", "river286", "delta776", "cedar178"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river196,river286,delta776,cedar178");
    assert_eq!(parse_record(&line), fields, "record 332 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_333() {
    let fields: Vec<String> = vec!["alpha730, beta113", "amber797", "maple967"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha730, beta113",amber797,maple967"#);
    assert_eq!(parse_record(&line), fields, "record 333 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_334() {
    let fields: Vec<String> = vec!["north763", "river171", "alpha360", "alpha212"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north763,river171,alpha360,alpha212");
    assert_eq!(parse_record(&line), fields, "record 334 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_335() {
    let fields: Vec<String> = vec!["north127", "alpha271", "delta182", "gamma378", "north40", "delta511", "cobalt301"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north127,alpha271,delta182,gamma378,north40,delta511,cobalt301");
    assert_eq!(parse_record(&line), fields, "record 335 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_336() {
    let fields: Vec<String> = vec!["north596, alpha163", "delta352", "south544"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north596, alpha163",delta352,south544"#);
    assert_eq!(parse_record(&line), fields, "record 336 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_337() {
    let fields: Vec<String> = vec!["gamma486", "amber271"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma486,amber271");
    assert_eq!(parse_record(&line), fields, "record 337 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_338() {
    let fields: Vec<String> = vec!["south147", "south995", "alpha719", "north710", "cedar392", "south704", "amber33"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south147,south995,alpha719,north710,cedar392,south704,amber33");
    assert_eq!(parse_record(&line), fields, "record 338 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_339() {
    let fields: Vec<String> = vec!["alpha388, gamma399", "amber460", "river726", "alpha780", "stone733", "river20"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha388, gamma399",amber460,river726,alpha780,stone733,river20"#);
    assert_eq!(parse_record(&line), fields, "record 339 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_340() {
    let fields: Vec<String> = vec!["alpha452", "north901", "cobalt590", "maple612", "alpha187", "amber872"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha452,north901,cobalt590,maple612,alpha187,amber872");
    assert_eq!(parse_record(&line), fields, "record 340 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_341() {
    let fields: Vec<String> = vec!["stone407", "alpha322", "gamma465"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone407,alpha322,gamma465");
    assert_eq!(parse_record(&line), fields, "record 341 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_342() {
    let fields: Vec<String> = vec!["amber322, north103", "cedar501", "river115", "alpha431", "gamma95", "river650"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber322, north103",cedar501,river115,alpha431,gamma95,river650"#);
    assert_eq!(parse_record(&line), fields, "record 342 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_343() {
    let fields: Vec<String> = vec!["amber580", "amber882", "maple713", "river303"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber580,amber882,maple713,river303");
    assert_eq!(parse_record(&line), fields, "record 343 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_344() {
    let fields: Vec<String> = vec!["maple268", "south372", "cedar249", "gamma745", "beta511", "delta487", "maple312"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple268,south372,cedar249,gamma745,beta511,delta487,maple312");
    assert_eq!(parse_record(&line), fields, "record 344 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_345() {
    let fields: Vec<String> = vec!["cedar720, beta137", "gamma819", "cobalt5", "alpha393", "beta608", "cobalt767", "stone898", "north249"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar720, beta137",gamma819,cobalt5,alpha393,beta608,cobalt767,stone898,north249"#);
    assert_eq!(parse_record(&line), fields, "record 345 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_346() {
    let fields: Vec<String> = vec!["south720", "stone103", "amber933", "gamma840"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south720,stone103,amber933,gamma840");
    assert_eq!(parse_record(&line), fields, "record 346 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_347() {
    let fields: Vec<String> = vec!["stone988", "north553", "alpha646", "cobalt664"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone988,north553,alpha646,cobalt664");
    assert_eq!(parse_record(&line), fields, "record 347 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_348() {
    let fields: Vec<String> = vec!["river946, cedar771", "river67", "cedar787", "gamma306"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river946, cedar771",river67,cedar787,gamma306"#);
    assert_eq!(parse_record(&line), fields, "record 348 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_349() {
    let fields: Vec<String> = vec!["cobalt998", "stone68", "amber985", "maple100", "gamma541", "south773"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt998,stone68,amber985,maple100,gamma541,south773");
    assert_eq!(parse_record(&line), fields, "record 349 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_350() {
    let fields: Vec<String> = vec!["stone4", "maple539", "cedar279", "stone335", "river699", "amber943"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone4,maple539,cedar279,stone335,river699,amber943");
    assert_eq!(parse_record(&line), fields, "record 350 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_351() {
    let fields: Vec<String> = vec!["north59, south437", "stone524", "maple175"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north59, south437",stone524,maple175"#);
    assert_eq!(parse_record(&line), fields, "record 351 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_352() {
    let fields: Vec<String> = vec!["gamma317", "alpha281", "delta802", "alpha834"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma317,alpha281,delta802,alpha834");
    assert_eq!(parse_record(&line), fields, "record 352 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_353() {
    let fields: Vec<String> = vec!["beta376", "north715", "alpha447"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta376,north715,alpha447");
    assert_eq!(parse_record(&line), fields, "record 353 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_354() {
    let fields: Vec<String> = vec!["gamma351, river4", "beta781", "beta136"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma351, river4",beta781,beta136"#);
    assert_eq!(parse_record(&line), fields, "record 354 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_355() {
    let fields: Vec<String> = vec!["cobalt278", "river1"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt278,river1");
    assert_eq!(parse_record(&line), fields, "record 355 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_356() {
    let fields: Vec<String> = vec!["cobalt674", "maple391", "north945", "south565", "delta600", "cobalt27", "delta544", "river779"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt674,maple391,north945,south565,delta600,cobalt27,delta544,river779");
    assert_eq!(parse_record(&line), fields, "record 356 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_357() {
    let fields: Vec<String> = vec!["amber649, alpha103", "alpha599", "cedar104"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber649, alpha103",alpha599,cedar104"#);
    assert_eq!(parse_record(&line), fields, "record 357 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_358() {
    let fields: Vec<String> = vec!["stone600", "delta433", "gamma338"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone600,delta433,gamma338");
    assert_eq!(parse_record(&line), fields, "record 358 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_359() {
    let fields: Vec<String> = vec!["south528", "beta318", "amber145", "gamma785", "stone621", "stone807"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south528,beta318,amber145,gamma785,stone621,stone807");
    assert_eq!(parse_record(&line), fields, "record 359 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_360() {
    let fields: Vec<String> = vec!["alpha174, south633", "cobalt7", "delta500", "amber921", "amber984"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha174, south633",cobalt7,delta500,amber921,amber984"#);
    assert_eq!(parse_record(&line), fields, "record 360 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_361() {
    let fields: Vec<String> = vec!["beta664", "cobalt95", "north951", "north758", "cedar494", "north331", "stone848", "delta694"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta664,cobalt95,north951,north758,cedar494,north331,stone848,delta694");
    assert_eq!(parse_record(&line), fields, "record 361 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_362() {
    let fields: Vec<String> = vec!["amber242", "amber549", "south457", "north910", "cobalt885", "cobalt617"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber242,amber549,south457,north910,cobalt885,cobalt617");
    assert_eq!(parse_record(&line), fields, "record 362 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_363() {
    let fields: Vec<String> = vec!["river125, stone349", "beta918", "gamma768", "beta555"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river125, stone349",beta918,gamma768,beta555"#);
    assert_eq!(parse_record(&line), fields, "record 363 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_364() {
    let fields: Vec<String> = vec!["beta831", "cobalt142", "river346", "gamma111", "south798", "river698", "beta809", "alpha38"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta831,cobalt142,river346,gamma111,south798,river698,beta809,alpha38");
    assert_eq!(parse_record(&line), fields, "record 364 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_365() {
    let fields: Vec<String> = vec!["alpha263", "amber968", "river579", "river469", "beta412", "beta856"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha263,amber968,river579,river469,beta412,beta856");
    assert_eq!(parse_record(&line), fields, "record 365 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_366() {
    let fields: Vec<String> = vec!["maple716, cedar716", "delta785", "amber144", "stone773", "north491", "cedar478"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple716, cedar716",delta785,amber144,stone773,north491,cedar478"#);
    assert_eq!(parse_record(&line), fields, "record 366 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_367() {
    let fields: Vec<String> = vec!["beta818", "beta274", "north360", "cedar341", "north955", "alpha818"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta818,beta274,north360,cedar341,north955,alpha818");
    assert_eq!(parse_record(&line), fields, "record 367 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_368() {
    let fields: Vec<String> = vec!["gamma153", "maple951", "north247", "stone111", "maple245", "gamma640"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma153,maple951,north247,stone111,maple245,gamma640");
    assert_eq!(parse_record(&line), fields, "record 368 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_369() {
    let fields: Vec<String> = vec!["cedar212, beta512", "amber141", "stone14", "amber172", "alpha660", "alpha453", "stone142"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cedar212, beta512",amber141,stone14,amber172,alpha660,alpha453,stone142"#);
    assert_eq!(parse_record(&line), fields, "record 369 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_370() {
    let fields: Vec<String> = vec!["gamma876", "delta846"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma876,delta846");
    assert_eq!(parse_record(&line), fields, "record 370 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_371() {
    let fields: Vec<String> = vec!["cobalt22", "beta409", "alpha859", "amber770", "cedar247", "alpha751", "amber60"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt22,beta409,alpha859,amber770,cedar247,alpha751,amber60");
    assert_eq!(parse_record(&line), fields, "record 371 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_372() {
    let fields: Vec<String> = vec!["river608, beta354", "gamma466", "delta48", "cobalt152", "delta467"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river608, beta354",gamma466,delta48,cobalt152,delta467"#);
    assert_eq!(parse_record(&line), fields, "record 372 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_373() {
    let fields: Vec<String> = vec!["cedar256", "south210", "gamma874", "gamma27", "maple452", "beta475", "alpha829", "beta848"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar256,south210,gamma874,gamma27,maple452,beta475,alpha829,beta848");
    assert_eq!(parse_record(&line), fields, "record 373 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_374() {
    let fields: Vec<String> = vec!["amber375", "delta643", "cobalt579"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber375,delta643,cobalt579");
    assert_eq!(parse_record(&line), fields, "record 374 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_375() {
    let fields: Vec<String> = vec!["amber72, gamma285", "amber575", "gamma860"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber72, gamma285",amber575,gamma860"#);
    assert_eq!(parse_record(&line), fields, "record 375 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_376() {
    let fields: Vec<String> = vec!["maple952", "delta675", "river978", "beta315", "delta469", "beta529", "delta147"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple952,delta675,river978,beta315,delta469,beta529,delta147");
    assert_eq!(parse_record(&line), fields, "record 376 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_377() {
    let fields: Vec<String> = vec!["delta483", "delta211", "south771", "river392"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta483,delta211,south771,river392");
    assert_eq!(parse_record(&line), fields, "record 377 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_378() {
    let fields: Vec<String> = vec!["amber858, north358", "amber389", "cobalt637", "alpha685", "amber878"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber858, north358",amber389,cobalt637,alpha685,amber878"#);
    assert_eq!(parse_record(&line), fields, "record 378 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_379() {
    let fields: Vec<String> = vec!["amber835", "cobalt107"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber835,cobalt107");
    assert_eq!(parse_record(&line), fields, "record 379 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_380() {
    let fields: Vec<String> = vec!["cobalt797", "cedar730", "cobalt519", "stone551", "alpha709", "delta306", "stone109", "delta523"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt797,cedar730,cobalt519,stone551,alpha709,delta306,stone109,delta523");
    assert_eq!(parse_record(&line), fields, "record 380 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_381() {
    let fields: Vec<String> = vec!["south883, stone876", "maple508"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south883, stone876",maple508"#);
    assert_eq!(parse_record(&line), fields, "record 381 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_382() {
    let fields: Vec<String> = vec!["river479", "maple34"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "river479,maple34");
    assert_eq!(parse_record(&line), fields, "record 382 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_383() {
    let fields: Vec<String> = vec!["alpha426", "cedar327"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha426,cedar327");
    assert_eq!(parse_record(&line), fields, "record 383 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_384() {
    let fields: Vec<String> = vec!["maple571, stone926", "stone401"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple571, stone926",stone401"#);
    assert_eq!(parse_record(&line), fields, "record 384 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_385() {
    let fields: Vec<String> = vec!["amber18", "delta831", "north814", "south163"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber18,delta831,north814,south163");
    assert_eq!(parse_record(&line), fields, "record 385 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_386() {
    let fields: Vec<String> = vec!["cobalt962", "alpha973", "north316", "river374"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt962,alpha973,north316,river374");
    assert_eq!(parse_record(&line), fields, "record 386 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_387() {
    let fields: Vec<String> = vec!["maple577, south467", "maple235", "cedar17", "gamma364", "amber669", "north788", "river172"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""maple577, south467",maple235,cedar17,gamma364,amber669,north788,river172"#);
    assert_eq!(parse_record(&line), fields, "record 387 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_388() {
    let fields: Vec<String> = vec!["gamma113", "delta777", "cedar751", "stone261", "amber403", "alpha295", "north314"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma113,delta777,cedar751,stone261,amber403,alpha295,north314");
    assert_eq!(parse_record(&line), fields, "record 388 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_389() {
    let fields: Vec<String> = vec!["north630", "alpha942"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north630,alpha942");
    assert_eq!(parse_record(&line), fields, "record 389 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_390() {
    let fields: Vec<String> = vec!["south932, south940", "beta967", "stone744", "cedar9", "river881", "north954", "stone395"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""south932, south940",beta967,stone744,cedar9,river881,north954,stone395"#);
    assert_eq!(parse_record(&line), fields, "record 390 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_391() {
    let fields: Vec<String> = vec!["stone310", "stone608"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone310,stone608");
    assert_eq!(parse_record(&line), fields, "record 391 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_392() {
    let fields: Vec<String> = vec!["cedar4", "maple588", "river535", "delta612", "maple444", "south902", "alpha227", "cedar129"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar4,maple588,river535,delta612,maple444,south902,alpha227,cedar129");
    assert_eq!(parse_record(&line), fields, "record 392 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_393() {
    let fields: Vec<String> = vec!["beta233, beta483", "north944", "beta868", "south895", "amber588"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta233, beta483",north944,beta868,south895,amber588"#);
    assert_eq!(parse_record(&line), fields, "record 393 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_394() {
    let fields: Vec<String> = vec!["gamma886", "south633", "cobalt441", "beta873", "south538", "north629", "alpha541", "north608"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma886,south633,cobalt441,beta873,south538,north629,alpha541,north608");
    assert_eq!(parse_record(&line), fields, "record 394 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_395() {
    let fields: Vec<String> = vec!["gamma848", "river334", "stone363", "beta405", "cobalt417", "beta391", "maple194"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma848,river334,stone363,beta405,cobalt417,beta391,maple194");
    assert_eq!(parse_record(&line), fields, "record 395 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_396() {
    let fields: Vec<String> = vec!["stone104, cedar524", "maple295"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""stone104, cedar524",maple295"#);
    assert_eq!(parse_record(&line), fields, "record 396 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_397() {
    let fields: Vec<String> = vec!["cedar924", "cedar98", "south177", "south780", "maple727", "cobalt605", "south632", "delta87"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar924,cedar98,south177,south780,maple727,cobalt605,south632,delta87");
    assert_eq!(parse_record(&line), fields, "record 397 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_398() {
    let fields: Vec<String> = vec!["gamma594", "river676", "stone178", "maple564", "stone4"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma594,river676,stone178,maple564,stone4");
    assert_eq!(parse_record(&line), fields, "record 398 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_399() {
    let fields: Vec<String> = vec!["alpha278, alpha487", "cobalt881", "stone6", "maple199"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""alpha278, alpha487",cobalt881,stone6,maple199"#);
    assert_eq!(parse_record(&line), fields, "record 399 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_400() {
    let fields: Vec<String> = vec!["alpha6", "gamma600", "south866", "delta23", "beta805", "gamma822", "beta212"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "alpha6,gamma600,south866,delta23,beta805,gamma822,beta212");
    assert_eq!(parse_record(&line), fields, "record 400 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_401() {
    let fields: Vec<String> = vec!["north595", "beta675", "beta794", "alpha286"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north595,beta675,beta794,alpha286");
    assert_eq!(parse_record(&line), fields, "record 401 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_402() {
    let fields: Vec<String> = vec!["north644, beta844", "cedar319", "stone432", "river295", "beta381", "river354", "river960", "delta8"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""north644, beta844",cedar319,stone432,river295,beta381,river354,river960,delta8"#);
    assert_eq!(parse_record(&line), fields, "record 402 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_403() {
    let fields: Vec<String> = vec!["gamma539", "stone148", "gamma545", "south817", "stone936", "amber661", "beta519", "stone695"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "gamma539,stone148,gamma545,south817,stone936,amber661,beta519,stone695");
    assert_eq!(parse_record(&line), fields, "record 403 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_404() {
    let fields: Vec<String> = vec!["south205", "north999", "cobalt91", "delta756", "amber27", "cobalt455", "gamma562", "gamma176"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "south205,north999,cobalt91,delta756,amber27,cobalt455,gamma562,gamma176");
    assert_eq!(parse_record(&line), fields, "record 404 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_405() {
    let fields: Vec<String> = vec!["beta902, cobalt792", "cobalt312", "delta560"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""beta902, cobalt792",cobalt312,delta560"#);
    assert_eq!(parse_record(&line), fields, "record 405 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_406() {
    let fields: Vec<String> = vec!["maple391", "beta442", "delta246", "beta787"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple391,beta442,delta246,beta787");
    assert_eq!(parse_record(&line), fields, "record 406 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_407() {
    let fields: Vec<String> = vec!["cobalt312", "delta623", "alpha151", "cedar625", "stone205", "amber525", "delta121", "stone97"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt312,delta623,alpha151,cedar625,stone205,amber525,delta121,stone97");
    assert_eq!(parse_record(&line), fields, "record 407 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_408() {
    let fields: Vec<String> = vec!["cobalt872, cobalt80", "alpha231", "stone660", "alpha876", "gamma478"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""cobalt872, cobalt80",alpha231,stone660,alpha876,gamma478"#);
    assert_eq!(parse_record(&line), fields, "record 408 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_409() {
    let fields: Vec<String> = vec!["beta331", "river755", "delta41", "cobalt848"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "beta331,river755,delta41,cobalt848");
    assert_eq!(parse_record(&line), fields, "record 409 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_410() {
    let fields: Vec<String> = vec!["maple567", "beta794", "cedar637", "river646"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "maple567,beta794,cedar637,river646");
    assert_eq!(parse_record(&line), fields, "record 410 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_411() {
    let fields: Vec<String> = vec!["gamma560, gamma200", "beta989", "south736", "stone742", "delta58"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""gamma560, gamma200",beta989,south736,stone742,delta58"#);
    assert_eq!(parse_record(&line), fields, "record 411 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_412() {
    let fields: Vec<String> = vec!["delta316", "cedar382", "cedar965", "cedar763", "river285"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "delta316,cedar382,cedar965,cedar763,river285");
    assert_eq!(parse_record(&line), fields, "record 412 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_413() {
    let fields: Vec<String> = vec!["north369", "amber643", "cobalt916", "south318", "south964", "gamma765", "river604", "south431"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "north369,amber643,cobalt916,south318,south964,gamma765,river604,south431");
    assert_eq!(parse_record(&line), fields, "record 413 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_414() {
    let fields: Vec<String> = vec!["river371, beta488", "cedar738", "river949", "alpha400", "maple661", "beta19"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""river371, beta488",cedar738,river949,alpha400,maple661,beta19"#);
    assert_eq!(parse_record(&line), fields, "record 414 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_415() {
    let fields: Vec<String> = vec!["cobalt924", "cobalt92", "delta696", "cobalt684", "cobalt120", "north466", "alpha214", "south879"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cobalt924,cobalt92,delta696,cobalt684,cobalt120,north466,alpha214,south879");
    assert_eq!(parse_record(&line), fields, "record 415 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_416() {
    let fields: Vec<String> = vec!["stone274", "stone12", "amber905", "amber19", "stone725", "river110", "maple263", "amber817"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "stone274,stone12,amber905,amber19,stone725,river110,maple263,amber817");
    assert_eq!(parse_record(&line), fields, "record 416 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_417() {
    let fields: Vec<String> = vec!["amber522, maple15", "gamma4", "alpha18", "amber11", "river205"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, r#""amber522, maple15",gamma4,alpha18,amber11,river205"#);
    assert_eq!(parse_record(&line), fields, "record 417 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_418() {
    let fields: Vec<String> = vec!["cedar843", "stone77", "river133", "stone491", "alpha116", "river219", "delta692"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "cedar843,stone77,river133,stone491,alpha116,river219,delta692");
    assert_eq!(parse_record(&line), fields, "record 418 mismatch: {:#?}", fields);
}

#[test]
fn roundtrip_419() {
    let fields: Vec<String> = vec!["amber544", "north379", "gamma254", "maple947", "alpha352"].into_iter().map(String::from).collect();
    let line = write_record(&fields);
    assert_eq!(line, "amber544,north379,gamma254,maple947,alpha352");
    assert_eq!(parse_record(&line), fields, "record 419 mismatch: {:#?}", fields);
}

#[test]
fn column_sum_000() {
    let rows: Vec<Vec<String>> = (0..53).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..53).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_001() {
    let rows: Vec<Vec<String>> = (0..92).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..92).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_002() {
    let rows: Vec<Vec<String>> = (0..189).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..189).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_003() {
    let rows: Vec<Vec<String>> = (0..63).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..63).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_004() {
    let rows: Vec<Vec<String>> = (0..80).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..80).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_005() {
    let rows: Vec<Vec<String>> = (0..69).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..69).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_006() {
    let rows: Vec<Vec<String>> = (0..200).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..200).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_007() {
    let rows: Vec<Vec<String>> = (0..124).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..124).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_008() {
    let rows: Vec<Vec<String>> = (0..75).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..75).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_009() {
    let rows: Vec<Vec<String>> = (0..77).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..77).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_010() {
    let rows: Vec<Vec<String>> = (0..130).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..130).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_011() {
    let rows: Vec<Vec<String>> = (0..101).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..101).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_012() {
    let rows: Vec<Vec<String>> = (0..186).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..186).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_013() {
    let rows: Vec<Vec<String>> = (0..96).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..96).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_014() {
    let rows: Vec<Vec<String>> = (0..151).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..151).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_015() {
    let rows: Vec<Vec<String>> = (0..83).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..83).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_016() {
    let rows: Vec<Vec<String>> = (0..93).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..93).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_017() {
    let rows: Vec<Vec<String>> = (0..106).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..106).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_018() {
    let rows: Vec<Vec<String>> = (0..128).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..128).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_019() {
    let rows: Vec<Vec<String>> = (0..169).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..169).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_020() {
    let rows: Vec<Vec<String>> = (0..59).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..59).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_021() {
    let rows: Vec<Vec<String>> = (0..122).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..122).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_022() {
    let rows: Vec<Vec<String>> = (0..87).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..87).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_023() {
    let rows: Vec<Vec<String>> = (0..79).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..79).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_024() {
    let rows: Vec<Vec<String>> = (0..53).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..53).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_025() {
    let rows: Vec<Vec<String>> = (0..106).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..106).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_026() {
    let rows: Vec<Vec<String>> = (0..84).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..84).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_027() {
    let rows: Vec<Vec<String>> = (0..169).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..169).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_028() {
    let rows: Vec<Vec<String>> = (0..173).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..173).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_029() {
    let rows: Vec<Vec<String>> = (0..70).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..70).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_030() {
    let rows: Vec<Vec<String>> = (0..105).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..105).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_031() {
    let rows: Vec<Vec<String>> = (0..160).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..160).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_032() {
    let rows: Vec<Vec<String>> = (0..70).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..70).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_033() {
    let rows: Vec<Vec<String>> = (0..71).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..71).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_034() {
    let rows: Vec<Vec<String>> = (0..123).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..123).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_035() {
    let rows: Vec<Vec<String>> = (0..96).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..96).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_036() {
    let rows: Vec<Vec<String>> = (0..177).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..177).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_037() {
    let rows: Vec<Vec<String>> = (0..62).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..62).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_038() {
    let rows: Vec<Vec<String>> = (0..122).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..122).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_039() {
    let rows: Vec<Vec<String>> = (0..179).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..179).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_040() {
    let rows: Vec<Vec<String>> = (0..175).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..175).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_041() {
    let rows: Vec<Vec<String>> = (0..68).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..68).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_042() {
    let rows: Vec<Vec<String>> = (0..155).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..155).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_043() {
    let rows: Vec<Vec<String>> = (0..86).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..86).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_044() {
    let rows: Vec<Vec<String>> = (0..149).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..149).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_045() {
    let rows: Vec<Vec<String>> = (0..91).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..91).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_046() {
    let rows: Vec<Vec<String>> = (0..127).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..127).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_047() {
    let rows: Vec<Vec<String>> = (0..78).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..78).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_048() {
    let rows: Vec<Vec<String>> = (0..63).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..63).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_049() {
    let rows: Vec<Vec<String>> = (0..133).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..133).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_050() {
    let rows: Vec<Vec<String>> = (0..76).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..76).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_051() {
    let rows: Vec<Vec<String>> = (0..120).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..120).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_052() {
    let rows: Vec<Vec<String>> = (0..88).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..88).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_053() {
    let rows: Vec<Vec<String>> = (0..109).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..109).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_054() {
    let rows: Vec<Vec<String>> = (0..140).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..140).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_055() {
    let rows: Vec<Vec<String>> = (0..78).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..78).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_056() {
    let rows: Vec<Vec<String>> = (0..149).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..149).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_057() {
    let rows: Vec<Vec<String>> = (0..84).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..84).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_058() {
    let rows: Vec<Vec<String>> = (0..94).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..94).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_059() {
    let rows: Vec<Vec<String>> = (0..127).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..127).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_060() {
    let rows: Vec<Vec<String>> = (0..186).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..186).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_061() {
    let rows: Vec<Vec<String>> = (0..198).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..198).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_062() {
    let rows: Vec<Vec<String>> = (0..161).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..161).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_063() {
    let rows: Vec<Vec<String>> = (0..165).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..165).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_064() {
    let rows: Vec<Vec<String>> = (0..159).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..159).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_065() {
    let rows: Vec<Vec<String>> = (0..168).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..168).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_066() {
    let rows: Vec<Vec<String>> = (0..102).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..102).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_067() {
    let rows: Vec<Vec<String>> = (0..89).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..89).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_068() {
    let rows: Vec<Vec<String>> = (0..69).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..69).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_069() {
    let rows: Vec<Vec<String>> = (0..127).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..127).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_070() {
    let rows: Vec<Vec<String>> = (0..128).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..128).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_071() {
    let rows: Vec<Vec<String>> = (0..194).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..194).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_072() {
    let rows: Vec<Vec<String>> = (0..97).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..97).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_073() {
    let rows: Vec<Vec<String>> = (0..93).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..93).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_074() {
    let rows: Vec<Vec<String>> = (0..147).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..147).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_075() {
    let rows: Vec<Vec<String>> = (0..86).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..86).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_076() {
    let rows: Vec<Vec<String>> = (0..165).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..165).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_077() {
    let rows: Vec<Vec<String>> = (0..127).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..127).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_078() {
    let rows: Vec<Vec<String>> = (0..108).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..108).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_079() {
    let rows: Vec<Vec<String>> = (0..113).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..113).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_080() {
    let rows: Vec<Vec<String>> = (0..81).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..81).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_081() {
    let rows: Vec<Vec<String>> = (0..167).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..167).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_082() {
    let rows: Vec<Vec<String>> = (0..153).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..153).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_083() {
    let rows: Vec<Vec<String>> = (0..181).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..181).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_084() {
    let rows: Vec<Vec<String>> = (0..152).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..152).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_085() {
    let rows: Vec<Vec<String>> = (0..111).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..111).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_086() {
    let rows: Vec<Vec<String>> = (0..128).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..128).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_087() {
    let rows: Vec<Vec<String>> = (0..152).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..152).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_088() {
    let rows: Vec<Vec<String>> = (0..177).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..177).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_089() {
    let rows: Vec<Vec<String>> = (0..101).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..101).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_090() {
    let rows: Vec<Vec<String>> = (0..150).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..150).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_091() {
    let rows: Vec<Vec<String>> = (0..97).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..97).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_092() {
    let rows: Vec<Vec<String>> = (0..94).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..94).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_093() {
    let rows: Vec<Vec<String>> = (0..145).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..145).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_094() {
    let rows: Vec<Vec<String>> = (0..53).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..53).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_095() {
    let rows: Vec<Vec<String>> = (0..195).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..195).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_096() {
    let rows: Vec<Vec<String>> = (0..105).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..105).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_097() {
    let rows: Vec<Vec<String>> = (0..142).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..142).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_098() {
    let rows: Vec<Vec<String>> = (0..59).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..59).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_099() {
    let rows: Vec<Vec<String>> = (0..94).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..94).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_100() {
    let rows: Vec<Vec<String>> = (0..87).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..87).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_101() {
    let rows: Vec<Vec<String>> = (0..89).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..89).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_102() {
    let rows: Vec<Vec<String>> = (0..71).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..71).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_103() {
    let rows: Vec<Vec<String>> = (0..56).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..56).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_104() {
    let rows: Vec<Vec<String>> = (0..73).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..73).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_105() {
    let rows: Vec<Vec<String>> = (0..148).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..148).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_106() {
    let rows: Vec<Vec<String>> = (0..88).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..88).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_107() {
    let rows: Vec<Vec<String>> = (0..123).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..123).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_108() {
    let rows: Vec<Vec<String>> = (0..164).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..164).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_109() {
    let rows: Vec<Vec<String>> = (0..111).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..111).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_110() {
    let rows: Vec<Vec<String>> = (0..92).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..92).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_111() {
    let rows: Vec<Vec<String>> = (0..130).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..130).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_112() {
    let rows: Vec<Vec<String>> = (0..177).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..177).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_113() {
    let rows: Vec<Vec<String>> = (0..200).map(|j: i64| vec![format!("r{}", j), (j * 2).to_string()]).collect();
    let expected: i64 = (0..200).map(|j: i64| j * 2).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_114() {
    let rows: Vec<Vec<String>> = (0..116).map(|j: i64| vec![format!("r{}", j), (j * 3).to_string()]).collect();
    let expected: i64 = (0..116).map(|j: i64| j * 3).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_115() {
    let rows: Vec<Vec<String>> = (0..107).map(|j: i64| vec![format!("r{}", j), (j * 4).to_string()]).collect();
    let expected: i64 = (0..107).map(|j: i64| j * 4).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_116() {
    let rows: Vec<Vec<String>> = (0..183).map(|j: i64| vec![format!("r{}", j), (j * 5).to_string()]).collect();
    let expected: i64 = (0..183).map(|j: i64| j * 5).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_117() {
    let rows: Vec<Vec<String>> = (0..58).map(|j: i64| vec![format!("r{}", j), (j * 6).to_string()]).collect();
    let expected: i64 = (0..58).map(|j: i64| j * 6).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_118() {
    let rows: Vec<Vec<String>> = (0..82).map(|j: i64| vec![format!("r{}", j), (j * 7).to_string()]).collect();
    let expected: i64 = (0..82).map(|j: i64| j * 7).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

#[test]
fn column_sum_119() {
    let rows: Vec<Vec<String>> = (0..183).map(|j: i64| vec![format!("r{}", j), (j * 1).to_string()]).collect();
    let expected: i64 = (0..183).map(|j: i64| j * 1).sum();
    for r in &rows { println!("row {:?}", r); }
    assert_eq!(column_sum(&rows, 1), expected, "rows: {:#?}", rows);
    assert!(column_width(&rows, 0) >= 2);
}

