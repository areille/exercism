use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .all(|c| set.insert(c))
}

// pub fn check(candidate: &str) -> bool {
//     let mut map: HashMap<char, u8> = HashMap::<char, u8>::new();
//     for c in candidate.to_lowercase().chars() {
//         if c.is_alphabetic() {
//             if map.contains_key(&c) {
//                 return false;
//             } else {
//                 map.insert(c, 1);
//             }
//         }
//     }
//     true
// }
