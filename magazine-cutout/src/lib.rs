use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();

    for word in magazine {
        *words.entry(word).or_insert(0) += 1;
    }

    for word in note {
        let entry = words.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    return true;
}
