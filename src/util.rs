#[macro_export]
macro_rules! string_hash_map {
    ($($key : expr => $value : expr), *) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert(($key).to_string(), $value);)*
        map
    }}
}
