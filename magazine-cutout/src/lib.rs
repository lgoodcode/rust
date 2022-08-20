use std::cmp::max;
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut track: HashMap<String, u32> = HashMap::new();

    for word in note {
        track
            .entry(word.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for word in magazine {
        if track.contains_key(&word.to_string()) {
            track
                .entry(word.to_string())
                .and_modify(|counter| *counter = max(*counter, 1) - 1);
        }
    }

    return track.values().all(|count| *count == 0);
}
