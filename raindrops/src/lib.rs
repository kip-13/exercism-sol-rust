pub fn raindrops(n: usize) -> String {
    let factor_mappers = [
        (3, "i"), (5, "a"), (7, "o")
    ];

    let string_number = factor_mappers
        .iter()
        .map(|&(x, y)| {
            if n % x == 0 {
                format!("Pl{}ng", y.to_string())
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("");

    if string_number.is_empty() {
        n.to_string()
    } else {
        string_number
    }
}
