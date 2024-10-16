mod database;
use database::db::DB;
use database::error::Error;
use ic_cdk::println as ic_println;
use ic_cdk_macros::*;
use ic_stable_structures::writer::Writer;
use ic_stable_structures::Memory as _;
use crate::database::memory::get_upgrades_memory;
use candid::{candid_method, CandidType, Deserialize};

#[update]
#[candid_method(update)]
fn create_collection(name: String, dimension: usize) -> Result<(), Error> {
    ic_println!("Creating collection '{}' with dimension {}", name, dimension);
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(&name, dimension)
    })
}

#[update]
#[candid_method(update)]
fn create_index(
    name: String,
    dimension: usize,
    docs: Vec<String>,
    embeddings: Vec<Vec<f32>>,
    file_name: String,
) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(&name, dimension);
        db.insert_into_collection(&name, embeddings, docs, file_name)?;
        db.build_index(&name)
    })
}

#[update]
#[candid_method(update)]
fn insert(
    name: String,
    keys: Vec<Vec<f32>>,
    values: Vec<String>,
    file_name: String,
) -> Result<(), Error> {
    ic_println!("Inserting into collection '{}'", name);
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.insert_into_collection(&name, keys, values, file_name)
    })
}

#[update]
#[candid_method(update)]
fn build_index(name: String) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.build_index(&name)
    })
}

#[update]
#[candid_method(update)]
fn delete_collection(name: String) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.delete_collection(&name)
    })
}

#[update]
#[candid_method(update)]
fn find_query(name: String, q: Vec<f32>, limit: i32) -> Result<Vec<String>, Error> {
    ic_println!("Received find_query for collection '{}'", name);
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let result = db.query(&name, q.clone(), limit);
        match result {
            Ok(data) => {
                if data.is_empty() {
                    ic_println!("No matching documents found for the query.");
                } else {
                    ic_println!("Documents found: {:?}", data);
                }
                let (_, strings): (Vec<_>, Vec<_>) = data.into_iter().unzip();
                Ok(strings)
            }
            Err(error) => {
                ic_println!("Error during query execution: {:?}", error);
                Err(Error::NotFound)
            }
        }
    })
}

#[query]
#[candid_method(query)]
fn get_collections() -> Result<Vec<String>, Error> {
    ic_println!("Fetching all collections");
    DB.with(|db| {
        let db = db.borrow();
        Ok(db.get_all_collections())
    })
}

#[query]
#[candid_method(query)]
fn get_docs(index_name: String) -> Result<Vec<String>, Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.get_docs(&index_name)
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let mut state_bytes = vec![];
    DB.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    let len = state_bytes.len() as u32;
    let mut memory = get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let memory = get_upgrades_memory();
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    DB.with(|s| *s.borrow_mut() = state);
}

// Exportar la interfaz Candid
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}