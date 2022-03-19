use crate::structs::character::Character;

pub fn characterizing<'b, const T: usize>(text: &'b str, charmaps: [[&'b str; 2]; T]) -> Vec<Character<'b>> {
    let mut result = Vec::new();

    for character in text.chars() {
        let mut tmp1 = Vec::new();

        for charmap in charmaps {
            if !charmap[1].is_empty() {
                if charmap[1] == "***" {
                    tmp1.push(
                        Character {
                            character_value: character,
                            character_type: charmap[0]
                        }
                    )
                }
                else {
                    if let Ok(regex_engine) = regex::Regex::new(charmap[1]) {
                        if regex_engine.is_match(character) {
                            tmp1.push(
                                Character {
                                    character_value: character,
                                    character_type: charmap[0]
                                }
                            )
                        }
                    }
                    else {
                        tmp1.push(
                            Character {
                                character_value: character,
                                character_type: charmap[0]
                            }
                        )
                    }
                }
            }
        }

        result.push(tmp1);
    }

    return result;
}