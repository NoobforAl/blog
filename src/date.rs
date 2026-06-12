const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const MONTHS_LONG: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn parse(date: &str) -> Option<(i32, usize, u32)> {
    let mut parts = date.trim().splitn(3, '-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if (1..=12).contains(&month) {
        Some((year, month - 1, day))
    } else {
        None
    }
}

/// "2024-01-10" -> "Jan 10, 2024"
pub fn short(date: &str) -> String {
    parse(date)
        .map(|(y, m, d)| format!("{} {}, {}", MONTHS_SHORT[m], d, y))
        .unwrap_or_else(|| date.to_string())
}

/// "2024-01-10" -> "January 10, 2024"
pub fn long(date: &str) -> String {
    parse(date)
        .map(|(y, m, d)| format!("{} {}, {}", MONTHS_LONG[m], d, y))
        .unwrap_or_else(|| date.to_string())
}
