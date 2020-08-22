

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

fn turn_on_bits(comp_bits: &mut Vec<usize>, on_bits: Vec<usize>) {
    on_bits.iter().for_each(|bit| comp_bits[*bit] = 1);
}

pub fn comp(mnemonic: Option<String>) -> String {

    // make vector of length 8, all zeros
    // make helper that accepts vector of indices to turn on
    // iterate through and turn them on

    let mut a = 0;
    let mut compBits = vec![0, 0, 0, 0, 0, 0, 0];

    let comp = mnemonic.unwrap();

    if comp.contains("M") {
        a = 1;
    }

    match comp.as_str() {
        "0" => turn_on_bits(&mut compBits, vec![1, 3, 5]),
        _ => println!("WORNG"),
    }

    format!("blah")
}
