use crate::structs::character::Character;

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