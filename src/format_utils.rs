/// Format a number as an ordinal (1st, 2nd, 3rd, 4th, etc.)
pub fn format_ordinal(n: usize) -> String {
    let suffix = match n % 100 {
        11..=13 => "th", // Special case: 11th, 12th, 13th (not 11st, 12nd, 13rd)
        _ => match n % 10 {
            1 => "st",
            2 => "nd", 
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", n, suffix)
}
