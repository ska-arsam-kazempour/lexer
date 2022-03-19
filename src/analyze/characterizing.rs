use crate::structs::character::Character;

pub fn characterizing<'b, const T: usize>(text: &'b str, charmaps: [[&'b str; 2]; T]) -> Vec<Character<'b>> {
    let mut result = Vec::new();

    for character in text.chars() {
        let mut tmp1 = Vec::new();

        for charmap in charmaps {
            if let Ok(regex_engine) = regex::Regex::new(charmap[1]) {
                if regex_engine.is_match(String::from(character).as_str()) {
                    tmp1.push(
                        Character {
                            character_value: character,
                            character_type: charmap[0]
                        }
                    );
                }
            }
        }

        if tmp1.len() > 0 {
            result.push(tmp1[0]);
        }
    }

    return result;
}