/// Gets min cell with given number
pub fn get_min_cell(num: usize) -> String {
    let snum = num.to_string();
    let padding = 5_usize.saturating_sub(snum.len());

    let left = padding / 2;
    let right = padding - left;
    format!("{}{}{}", " ".repeat(left + 8), snum, " ".repeat(right + 8))
}

/// Gets cell with given number
pub fn get_cell(num: usize) -> String {
    match num {
        1 => format!(
            "{}{}{}{}{}",
            "           ",
            "    ▄█     ",
            "     █     ",
            "     █     ",
            "           ",
        ),
        2 => format!(
            "{}{}{}{}{}",
            "           ",
            "   ▄▀▀▀▄   ",
            "     ▄▀    ",
            "   ▄█▄▄▄   ",
            "           ",
        ),
        3 => format!(
            "{}{}{}{}{}",
            "           ",
            "   ▄▀▀▀▄   ",
            "      ▀▄   ",
            "   ▀▄▄▄▀   ",
            "           ",
        ),
        4 => format!(
            "{}{}{}{}{}",
            "           ",
            "    █      ",
            "    █▄█▄   ",
            "      █    ",
            "           ",
        ),
        5 => format!(
            "{}{}{}{}{}",
            "           ",
            "   █▀▀▀▀   ",
            "   ▀▀▀▀▄   ",
            "   ▀▄▄▄▀   ",
            "           ",
        ),
        6 => format!(
            "{}{}{}{}{}",
            "           ",
            "    ▄▀▀    ",
            "    █▀▀▄   ",
            "    ▀▄▄▀   ",
            "           ",
        ),
        7 => format!(
            "{}{}{}{}{}",
            "           ",
            "   ▀▀▀▀█   ",
            "      █    ",
            "     █     ",
            "           ",
        ),
        8 => format!(
            "{}{}{}{}{}",
            "           ",
            "   ▄▀▀▀▄   ",
            "   ▄▀▀▀▄   ",
            "   ▀▄▄▄▀   ",
            "           ",
        ),
        9 => format!(
            "{}{}{}{}{}",
            "           ",
            "    ▄▀▀▄   ",
            "    ▀▄▄█   ",
            "     ▄▄▀   ",
            "           ",
        ),
        _ => String::new(),
    }
}

/// Gets selected min cell with given number
pub fn get_min_sel_cell(num: usize) -> String {
    let snum = num.to_string();
    let padding = 5_usize.saturating_sub(snum.len());

    let left = padding / 2;
    let right = padding - left;
    format!(
        "█▀▀▀▀▀██{}{}{}██▄▄▄▄▄█",
        " ".repeat(left),
        snum,
        " ".repeat(right)
    )
}

/// Gets selected cell with given number
pub fn get_sel_cell(num: usize) -> String {
    match num {
        1 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█   ▄█    █",
            "█    █    █",
            "█    █    █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        2 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█  ▄▀▀▀▄  █",
            "█    ▄▀   █",
            "█  ▄█▄▄▄  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        3 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█  ▄▀▀▀▄  █",
            "█     ▀▄  █",
            "█  ▀▄▄▄▀  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        4 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█   █     █",
            "█   █▄█▄  █",
            "█     █   █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        5 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█  █▀▀▀▀  █",
            "█  ▀▀▀▀▄  █",
            "█  ▀▄▄▄▀  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        6 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█   ▄▀▀   █",
            "█   █▀▀▄  █",
            "█   ▀▄▄▀  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        7 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█  ▀▀▀▀█  █",
            "█     █   █",
            "█    █    █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        8 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█  ▄▀▀▀▄  █",
            "█  ▄▀▀▀▄  █",
            "█  ▀▄▄▄▀  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        9 => format!(
            "{}{}{}{}{}",
            "█▀▀▀▀▀▀▀▀▀█",
            "█   ▄▀▀▄  █",
            "█   ▀▄▄█  █",
            "█    ▄▄▀  █",
            "█▄▄▄▄▄▄▄▄▄█"
        ),
        _ => String::new(),
    }
}
