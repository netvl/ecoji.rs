include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

#[test]
fn test_mapping() {
    assert_eq!(EMOJIS.len(), 1024);
    assert_eq!(EMOJIS_REV.len(), 1024);
    for (i, c) in EMOJIS.iter().cloned().enumerate() {
        assert_eq!(i, EMOJIS_REV[&c]);
    }
}
