const MIN: u32 = 367479;
const MAX: u32 = 893698;

fn is_valid_pw(pw: u32) -> bool {
    if pw < 100000 || pw > 999999 { return false }
    let pw_str = pw.to_string();
    let mut chars = pw_str.chars();
    let mut prev = chars.next().unwrap();
    let mut rep_count = 0u32;
    let mut double_digit = false;
    for ch in chars {
        if ch == prev {
            rep_count += 1
        } else {
            if rep_count == 1 { double_digit = true }
            rep_count = 0;
        }
        if ch < prev { return false }
        prev = ch
    }
    if rep_count == 1 { true }
    else { double_digit }
}

fn main() {
    let mut valid = 0u32;
    for pw in MIN..=MAX {
        valid += if is_valid_pw(pw) { 1 } else { 0 }
    }
    println!("{}", valid)
}
