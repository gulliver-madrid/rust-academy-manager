/// Transforms the string it receives into a representation of it with the indicated number of characters
/// Examples:
/// Rabbit, 4 => "R..."
/// Rabbit, 6 => "Rabbit"
/// Rabbit, 8 => "Rabbit  "
pub fn set_number_chars(s: &str, n: usize) -> String {
    let elipsis_size = 3;
    let copied = String::from(s);
    std::assert!(n >= elipsis_size);
    let chars_count = s.chars().count();
    if n == chars_count {
        copied
    } else if n > chars_count {
        add_repeated_char(copied, ' ', n - chars_count)
    } else {
        let substring = get_substring(copied, n - elipsis_size);
        add_repeated_char(substring, '.', elipsis_size)
    }
}

/// Anade el caracter c repetido n veces
pub fn add_repeated_char(s: String, c: char, n: usize) -> String {
    let mut repeated = String::new();
    for _i in 0..n {
        repeated.push(c)
    }
    s + &repeated
}

fn get_substring(s: String, n: usize) -> String {
    let chars = s.chars();
    let mut i = 0;
    let mut nueva = String::new();
    let v: Vec<char> = chars.collect();
    loop {
        if i >= n {
            break;
        }
        nueva.push(v[i]);
        i += 1;
    }
    nueva
}
pub fn get_last_element<T>(vector: &Vec<T>) -> Option<&T> {
    match vector.len() {
        0 => None,
        length => vector.get(length - 1),
    }
}

pub fn only_contains_digits(string: &str) -> bool {
    string.chars().all(|c| c.is_numeric())
}
pub fn contains_some_digits(string: &str) -> bool {
    string.chars().any(|c| c.is_numeric())
}
