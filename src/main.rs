fn main() {
    println!("Hello, world!");
}

fn colemak_as_qwerty(s: &str) -> String {
    let qwerty = "ertyuiopsdfgjkl;n";
    let colemak = "fpgjluy;rstdneiok";
    let qwerty_from_colemak = Translator::new(colemak, qwerty);
    qwerty_from_colemak.translate(s)
}

struct Translator {
    translation: std::collections::HashMap<char, char>,
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
