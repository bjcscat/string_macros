use string_macros::{ascii_lowercase, ascii_uppercase};

const _: () = const {
    assert!(ascii_lowercase!("A").as_bytes()[0] == b'a');
};

#[test]
fn ascii_lower() {
    assert_eq!(ascii_lowercase!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz")
}

#[test]
fn ascii_upper() {
    assert_eq!(ascii_uppercase!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ")
}