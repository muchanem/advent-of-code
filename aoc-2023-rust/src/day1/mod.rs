pub fn line_to_cal(line: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();

    for &item in line.as_bytes().iter() {
        if (item <= 57) && (item >= 49) {
            digits.push(u32::from(item-48));
        }
    }

    digits[0]*10+digits[digits.len()-1]
}

#[cfg(test)]
mod tests {
use crate::day1::*;
    #[test]
    fn test_line() {
        assert_eq!(line_to_cal("1abc2"),12);
        assert_eq!(line_to_cal("pqr3stu8vwx"),38);
        assert_eq!(line_to_cal("a1b2c3d4e5f"),15);
        assert_eq!(line_to_cal("treb7uchet"),77);
    }    
}