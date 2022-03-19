use crate::structs::character::Character;

pub fn characterizing<'b, const T: usize>(text: &'b str, charmaps: [[&'b str; 2]; T]) -> Vec<Character<'b>> {
    let mut result = Vec::new();

    for character in text.chars() {
        let mut tmp1 = Vec::new();

        for charmap in charmaps.iter() {
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

pub fn character_categorizing<'b>(characters: Vec<Character<'b>>) -> Vec<Vec<Character<'b>>> {
    let mut result = Vec::new();

    let mut tmp1:Vec<Character> = Vec::new();
    for character in characters {
        if let Some(last_item) = tmp1.last() {
            if last_item.character_type == character.character_type {
                tmp1.push(character);
            }
            else {
                result.push(tmp1.clone());
                tmp1.clear();
                tmp1.push(character);
            }
        }
        else {
            tmp1.push(character);
        }
    }

    return result;
}

pub fn tokenizer<'b>(catcharacters: Vec<Vec<Character<'b>>>) -> Vec<String> {
    let mut result = Vec::new();

    let mut tmp1 = String::new();

    for cat in catcharacters {
        tmp1.clear();

        for character in cat {
            tmp1.push(character.character_value);
        }

        result.push(tmp1.clone());
    }

    return result;
}