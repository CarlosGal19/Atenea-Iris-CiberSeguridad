use super::index::{generate_index, Vector};
use instant_distance::{HnswMap, Search};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ic_cdk::println as ic_println;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub dimension: usize,
    pub metadata: Metadata,
    inner: HnswMap<Vector, String>,
    keys: Vec<Vector>,
    values: Vec<String>,
}

impl Collection {
    pub fn new(keys: Vec<Vector>, values: Vec<String>, dimension: usize) -> Self {
        Collection {
            keys: keys.clone(),
            values: values.clone(),
            inner: generate_index(keys, values),
            dimension,
            metadata: Metadata {
                file_names: HashSet::new(),
            },
        }
    }

    pub fn append(
        &mut self,
        keys: &mut Vec<Vector>,
        values: &mut Vec<String>,
        file_name: String,
    ) {
        ic_println!("Appending {} keys and values to the collection", keys.len());
        self.keys.append(keys);
        self.values.append(values);
        self.metadata.file_names.insert(file_name);
    }

    pub fn query(&self, key: &Vector, search: &mut Search, limit: i32) -> Vec<(f32, String)> {
        let mut res: Vec<(f32, String)> = vec![];
        let mut iter = self.inner.search(key, search);

        ic_println!("Starting search in collection with {} documents", self.keys.len());

        for _ in 0..limit {
            match iter.next() {
                Some(v) => {
                    let similarity = v.point.cos_sim(key);
                    ic_println!("Found document with similarity: {}", similarity);
                    res.push((similarity, (*v.value).clone()));
                }
                None => {
                    ic_println!("No more documents found in search iteration.");
                    break;
                }
            }
        }

        res.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        ic_println!("Final sorted search results: {:?}", res);
        res
    }

    pub fn build_index(&mut self) {
        self.inner = generate_index(self.keys.clone(), self.values.clone());
        ic_println!("Index built with {} documents", self.keys.len());
    }
}