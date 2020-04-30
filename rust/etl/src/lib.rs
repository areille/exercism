use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::<char, i32>::new();
    for (points, letters) in h {
        for c in letters {
            res.entry(c.to_ascii_lowercase()).or_insert(*points);
        }
    }
    res
}
