include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

pub fn is_valid_alphabet_char(c: char) -> bool {
    [PADDING, PADDING_40, PADDING_41, PADDING_42, PADDING_43].contains(&c) ||
        EMOJIS_REV.contains_key(&c)
}

#[test]
fn test_mapping() {
    assert_eq!(EMOJIS.len(), 1024);
    assert_eq!(EMOJIS_REV.len(), 1024);
    for (i, c) in EMOJIS.iter().cloned().enumerate() {
        assert_eq!(i, EMOJIS_REV[&c]);
    }
}
