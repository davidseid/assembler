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

fn turn_on_bits(target_bits: &mut Vec<usize>, on_bits: Vec<usize>) {
    on_bits.iter().for_each(|bit| target_bits[*bit] = 1);
}

pub fn jump(mnemonic: Option<String>) -> String {
    let mut jump_bits = vec![0, 0, 0];

    let jump = mnemonic.unwrap();

    match jump.as_str() {
        "JGT" => turn_on_bits(&mut jump_bits, vec![2]),
        "JEQ" => turn_on_bits(&mut jump_bits, vec![1]),
        "JGE" => turn_on_bits(&mut jump_bits, vec![1, 2]),
        "JLT" => turn_on_bits(&mut jump_bits, vec![0]),
        "JNE" => turn_on_bits(&mut jump_bits, vec![0, 2]),
        "JLE" => turn_on_bits(&mut jump_bits, vec![0, 1]),
        "JMP" => turn_on_bits(&mut jump_bits, vec![0, 1, 2]),
        _ => {},
    }

    format!("{}", jump_bits.into_iter().map(|i| i.to_string()).collect::<String>())
}

pub fn comp(mnemonic: Option<String>) -> String {
    let mut a = 0;
    let mut comp_bits = vec![0, 0, 0, 0, 0, 0];

    let comp = mnemonic.unwrap();

    if comp.contains("M") {
        a = 1;
    }

    match comp.as_str() {
        "0" => turn_on_bits(&mut comp_bits, vec![0, 2, 4]),
        "1" => turn_on_bits(&mut comp_bits, vec![0, 1, 2, 3, 4, 5]),
        "-1" => turn_on_bits(&mut comp_bits, vec![0, 1, 2, 4]),
        "D" => turn_on_bits(&mut comp_bits, vec![2, 3]),
        "A" | "M" => turn_on_bits(&mut comp_bits, vec![0, 1]),
        "!D" => turn_on_bits(&mut comp_bits, vec![2, 3, 5]),
        "!A" | "!M" => turn_on_bits(&mut comp_bits, vec![0, 1, 5]),
        "-D" => turn_on_bits(&mut comp_bits, vec![2, 3, 4, 5]),
        "-A" | "-M" => turn_on_bits(&mut comp_bits, vec![0, 1, 4, 5]),
        "D+1" => turn_on_bits(&mut comp_bits, vec![1, 2, 3, 4, 5]),
        "A+1" | "M+1" => turn_on_bits(&mut comp_bits, vec![0, 1, 3, 4, 5]),
        "D-1" => turn_on_bits(&mut comp_bits, vec![2, 3, 4]),
        "A-1" | "M-1" => turn_on_bits(&mut comp_bits, vec![0, 1, 4]),
        "D+A" | "D+M" => turn_on_bits(&mut comp_bits, vec![4]),
        "D-A" | "D-M" => turn_on_bits(&mut comp_bits, vec![1, 4, 5]),
        "A-D" | "M-D" => turn_on_bits(&mut comp_bits, vec![3, 4, 5]),
        "D&A" | "D&M" => {},
        "D|A" | "D|M" => turn_on_bits(&mut comp_bits, vec![1, 3, 5]),
        _ => {},
    }

    format!("{}{}", a, comp_bits.into_iter().map(|i| i.to_string()).collect::<String>())
}

