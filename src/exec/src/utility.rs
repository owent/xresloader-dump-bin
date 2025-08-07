use std::collections::LinkedList;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufRead, BufReader};

pub fn load_file_by_lines<P>(file_path: &str, file_type: &str, has_error: &mut bool, func: P)
where
    P: FnMut(&str) -> Result<(), String>,
{
    let mut invoke = func;
    match std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(file_path)
    {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader
                .lines()
                .map_while(Result::ok)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && !s.starts_with('#'))
                .for_each(|line| {
                    if let Err(e) = invoke(&line) {
                        error!(
                            "Invalid {}: \"{}\" in file {}, {}, ignore this line",
                            file_type, line, file_path, e
                        );
                        *has_error = true;
                    }
                })
        }
        Err(e) => {
            error!(
                "Try to open {} file {} failed, {}, ignore this file",
                file_type, file_path, e
            );
            *has_error = true;
        }
    }
}

pub fn for_each_ordered_hash_map<K, V, F>(map: &HashMap<K, V>, mut func: F)
where
    K: std::cmp::Ord + Hash + Eq,
    F: FnMut(&K, &V),
{
    let mut keys = Vec::with_capacity(map.len());
    map.iter().for_each(|kv| {
        keys.push(kv.0);
    });
    keys.sort();

    for key in keys {
        if let Some(row) = map.get_key_value(key) {
            func(key, row.1);
        }
    }
}

#[allow(dead_code)]
pub fn for_each_ordered_hash_set_by<T, F, C>(list: &HashSet<T>, mut compare: C, mut func: F)
where
    C: FnMut(&T, &T) -> std::cmp::Ordering,
    F: FnMut(&T),
{
    let mut items = Vec::with_capacity(list.len());
    list.iter().for_each(|item| {
        items.push(item);
    });
    items.sort_by(|a, b| compare(a, b));

    for item in items {
        func(item);
    }
}

#[allow(dead_code)]
pub fn for_each_ordered_linked_list_by<T, F, C>(list: &LinkedList<T>, mut compare: C, mut func: F)
where
    C: FnMut(&T, &T) -> std::cmp::Ordering,
    F: FnMut(&T),
{
    let mut items = Vec::with_capacity(list.len());
    list.iter().for_each(|item| {
        items.push(item);
    });
    items.sort_by(|a, b| compare(a, b));

    for item in items {
        func(item);
    }
}

#[allow(dead_code)]
pub fn for_each_ordered_linked_list_key<T, F, C, K>(list: &LinkedList<T>, mut get_key: C, func: F)
where
    K: std::cmp::Ord,
    C: FnMut(&T) -> &K,
    F: FnMut(&T),
{
    for_each_ordered_linked_list_by(list, |a, b| get_key(a).cmp(get_key(b)), func);
}

pub fn for_each_ordered_vec_by<T, F, C>(list: &Vec<T>, mut compare: C, mut func: F)
where
    C: FnMut(&T, &T) -> std::cmp::Ordering,
    F: FnMut(&T),
{
    let mut items = Vec::with_capacity(list.len());
    list.iter().for_each(|item| {
        items.push(item);
    });
    items.sort_by(|a, b| compare(a, b));

    for item in items {
        func(item);
    }
}
