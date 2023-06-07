use std::collections::HashMap;


pub fn remove_value() -> () {


    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove_entry(&1), Some((1, "a")));
    assert_eq!(map.remove(&1), None);
}