use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn main() {
    // read dictionary from stdin
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let dict: HashSet<_> = input.split_whitespace().collect();

    // find words
    println!("Words that turn into new words when a Colemak typist types on a qwerty keyboard");
    for (c, q) in find_matching_words(&dict) {
        println!("{} -> {}", c, q);
    }
}

fn find_matching_words(dictionary: &HashSet<&str>) -> HashMap<String, String> {
    let qwerty = "ertyuiopsdfgjkl;n";
    let colemak = "fpgjluy;rstdneiok";
    let qwerty_from_colemak = Translator::new(colemak, qwerty);

    let mut matches = HashMap::new();
    for word in dictionary {
        let qwerty = qwerty_from_colemak.translate(word);
        if dictionary.contains::<str>(&qwerty) && qwerty != *word {
            matches.insert(word.to_string(), qwerty.clone());
        }
    }

    matches
}

struct Translator {
    translation: HashMap<char, char>,
}

impl Translator {
    fn new(from: &str, to: &str) -> Translator {
        Translator {
            translation: from.chars().zip(to.chars()).collect(),
        }
    }
    fn translate(&self, s: &str) -> String {
        s.chars()
            .map(|from| {
                if let Some(&to) = self.translation.get(&from) {
                    to
                } else {
                    from
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn colemak_as_qwerty(s: &str) -> String {
        let qwerty = "ertyuiopsdfgjkl;n";
        let colemak = "fpgjluy;rstdneiok";
        let qwerty_from_colemak = Translator::new(colemak, qwerty);
        qwerty_from_colemak.translate(s)
    }

    #[test]
    fn test_colemak_as_qwerty() {
        let qwerty = "qwertyuiopasdfghjkl;zxcvbnm";
        let colemak = "qwfpgjluy;arstdhneiozxcvbkm";
        assert_eq!("", colemak_as_qwerty(""));
        assert_eq!("a", colemak_as_qwerty("a"));
        assert_eq!("j", colemak_as_qwerty("n"));
        assert_eq!(qwerty, colemak_as_qwerty(colemak));
        assert_eq!("fork", colemak_as_qwerty("type"));
    }

    #[test]
    fn test_find_matching_words() {
        let mut dict: HashSet<&str> = HashSet::new();
        dict.insert("a");
        dict.insert("I");
        dict.insert("type");
        dict.insert("fork");
        let mut expected = HashMap::new();
        expected.insert("type".to_string(), "fork".to_string());
        assert_eq!(expected, find_matching_words(&dict));
    }
}
