#![doc = include_str!("../README.md")]

use regex::Regex;

pub fn is_hawk_tuah(sentence: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(r"(?i)\bhawk\b[\s[:punct:]]+\b(too|two|tuah|to)\b")?;
    Ok(re.is_match(sentence))
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_hawk_tuah() {
        let valid_sentences = [
            "Hawk too!",
            "Hawk...two?",
            "hawk    two",
            "Hawk---too",
            "like a hawk. two days ago",
            "hawk, two",
            "hawk, too!",
            "HAWK. tOo",
            "hawk tuah",
            "HAWK, TUAH!",
            "hawk tuah.",
            "Hawk tuah?",
            "hawk to"
        ];

        let invalid_sentences = [
            "This doesn't match.",
            "hawk",
            "two",
            "hawk three",
            "tuah",
        ];

        for sentence in valid_sentences.iter() {
            assert!(super::is_hawk_tuah(sentence).unwrap());
        }

        for sentence in invalid_sentences.iter() {
            assert!(!super::is_hawk_tuah(sentence).unwrap());
        }
    }
}