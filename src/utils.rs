
pub fn is_letter(s: &String) -> bool {
    &"a".to_string() <= s && s <= &"z".to_string() ||
    &"A".to_string() <= s && s <= &"Z".to_string() || &"_".to_string() == s
}

pub fn is_digit(s: &String) -> bool {
    let c = s.chars().nth(0);
    match c {
        Some(n) => n.is_digit(10),
        None => false,
    }
}
