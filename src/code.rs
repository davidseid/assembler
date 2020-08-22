

pub fn dest(mnemonic: Option<String>) -> String {
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d3 = 0;

    match mnemonic {
        Some(dest) => {
            if dest.contains("M") {
                d3 = 1;
            }

            if dest.contains("D") {
                d2 = 1;
            }

            if dest.contains("A") {
                d1 = 1;
            }
        },
        None => {},
    }

    format!("{}{}{}", d1, d2, d3)
}
