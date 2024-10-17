use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::call::call;
use ic_cdk_macros::*;
use ic_cdk::println as ic_println;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

type DocumentId = u64;

#[derive(CandidType, Deserialize, Debug)]
enum CustomError {
    MemoryError,
    UniqueViolation,
    DimensionMismatch,
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
enum CustomResult {
    Ok,
    Err(CustomError),
}

#[derive(CandidType, Deserialize, Debug)]
enum CustomResult2 {
    Ok(Vec<String>),
    Err(CustomError),
}

#[derive(Default)]
struct NlpCanister {
    vocabulary: HashMap<String, usize>,
    doc_freqs: HashMap<usize, u64>,
    total_docs: u64,
}

thread_local! {
    static NLP_CANISTER: RefCell<NlpCanister> = RefCell::new(NlpCanister::default());
}

static VECTORDB_CANISTER_ID: Lazy<Principal> = Lazy::new(|| {
    // Reemplaza con el ID real de tu canister vectordb
    Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap()
});

#[update]
#[candid_method(update)]
async fn process_text(doc_id: DocumentId, text: String) -> Vec<f32> {
    ic_println!("Starting text processing for document ID: {}", doc_id);

    let (tf_idf_vector, vector_dimension) = NLP_CANISTER.with(|canister| {
        let mut canister = canister.borrow_mut();
        canister.total_docs += 1;
        let tokens = tokenize(&text);
        let mut term_counts = HashMap::new();

        for token in tokens {
            let token_lower = token.to_lowercase();
            let vocab_len = canister.vocabulary.len();
            let index = *canister
                .vocabulary
                .entry(token_lower.clone())
                .or_insert_with(|| vocab_len);
            *term_counts.entry(index).or_insert(0u64) += 1;
        }

        for &index in term_counts.keys() {
            *canister.doc_freqs.entry(index).or_insert(0u64) += 1;
        }

        ic_println!("Vocabulary size after processing: {}", canister.vocabulary.len());
        ic_println!("Term counts: {:?}", term_counts);
        ic_println!("Document frequencies: {:?}", canister.doc_freqs);

        let tf_idf_vector = compute_tf_idf(&term_counts, &canister.doc_freqs, canister.total_docs);
        let vector_dimension = tf_idf_vector.len();

        (tf_idf_vector, vector_dimension)
    });

    ic_println!("Computed TF-IDF vector: {:?}", tf_idf_vector);
    ic_println!("Vector dimension: {}", vector_dimension);

    let collection_name = "police_reports".to_string();

    ic_println!("Checking if collection '{}' exists in vectordb...", collection_name);
    let collections: Vec<String> = match call::<(), (CustomResult2,)>(
        *VECTORDB_CANISTER_ID,
        "get_collections",
        ()
    ).await {
        Ok((CustomResult2::Ok(collections),)) => {
            ic_println!("Retrieved collections: {:?}", collections);
            collections
        },
        Ok((CustomResult2::Err(error),)) => {
            ic_println!("Error fetching collections from vectordb: {:?}", error);
            return tf_idf_vector;
        },
        Err((rejection_code, msg)) => {
            ic_println!("Failed to call vectordb.get_collections: {:?}, {}", rejection_code, msg);
            return tf_idf_vector;
        }
    };

    if !collections.contains(&collection_name) {
        ic_println!("Collection '{}' not found, creating it...", collection_name);
        match call::<(String, u64), (CustomResult,)>(
            *VECTORDB_CANISTER_ID,
            "create_collection",
            (collection_name.clone(), vector_dimension as u64),
        )
        .await
        {
            Ok((CustomResult::Ok,)) => ic_println!("Collection created successfully."),
            Ok((CustomResult::Err(error),)) => {
                ic_println!("Error creating collection: {:?}", error);
                return tf_idf_vector;
            }
            Err((rejection_code, msg)) => {
                ic_println!("Failed to call vectordb.create_collection: {:?}, {}", rejection_code, msg);
                return tf_idf_vector;
            }
        }
    } else {
        ic_println!("Collection '{}' already exists.", collection_name);
    }

    ic_println!("Inserting document into vectordb collection '{}'", collection_name);
    let vectors = vec![tf_idf_vector.clone()];
    let documents = vec![text.clone()];
    let metadata = format!("doc_id:{}", doc_id);

    match call::<(String, Vec<Vec<f32>>, Vec<String>, String), (CustomResult,)>(
        *VECTORDB_CANISTER_ID,
        "insert",
        (collection_name.clone(), vectors, documents, metadata),
    )
    .await
    {
        Ok((CustomResult::Ok,)) => ic_println!("Successfully inserted data into vectordb."),
        Ok((CustomResult::Err(error),)) => ic_println!("Error inserting data into vectordb: {:?}", error),
        Err((rejection_code, msg)) => ic_println!("Failed to call vectordb.insert: {:?}, {}", rejection_code, msg),
    }

    tf_idf_vector
}

#[update]
#[candid_method(update)]
async fn find_similar(text: String, top_k: i32) -> Vec<String> {
    ic_println!("Starting similarity search for: '{}'", text);

    let (query_vector, vector_dimension) = NLP_CANISTER.with(|canister| {
        let canister = canister.borrow();
        ic_println!("Vocabulary size when finding similar: {}", canister.vocabulary.len());

        let tokens = tokenize(&text);
        let mut term_counts = HashMap::new();
        for token in tokens {
            let token_lower = token.to_lowercase();
            if let Some(&index) = canister.vocabulary.get(&token_lower) {
                *term_counts.entry(index).or_insert(0u64) += 1;
            }
        }

        ic_println!("Term counts for query: {:?}", term_counts);

        let query_vector = compute_tf_idf(&term_counts, &canister.doc_freqs, canister.total_docs);
        let vector_dimension = query_vector.len();

        (query_vector, vector_dimension)
    });

    ic_println!("Computed query TF-IDF vector: {:?}", query_vector);
    ic_println!("Query vector dimension: {}", vector_dimension);

    let collection_name = "police_reports".to_string();

    match call::<(String, Vec<f32>, i32), (CustomResult2,)>(
        *VECTORDB_CANISTER_ID,
        "find_query",
        (collection_name.clone(), query_vector, top_k),
    )
    .await
    {
        Ok((CustomResult2::Ok(results),)) => {
            ic_println!("Query successful, found results: {:?}", results);
            results
        },
        Ok((CustomResult2::Err(error),)) => {
            ic_println!("Error querying vectordb: {:?}", error);
            vec![]
        }
        Err((rejection_code, msg)) => {
            ic_println!("Failed to call vectordb.find_query: {:?}, {}", rejection_code, msg);
            vec![]
        }
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

fn compute_tf_idf(
    term_counts: &HashMap<usize, u64>,
    doc_freqs: &HashMap<usize, u64>,
    total_docs: u64,
) -> Vec<f32> {
    let vocab_size = doc_freqs.len();
    let mut tf_idf_vector = vec![0.0f32; vocab_size];
    let total_terms_in_doc: u64 = term_counts.values().sum();

    for (&index, &count) in term_counts {
        let tf = count as f32 / total_terms_in_doc as f32;
        let df = doc_freqs.get(&index).cloned().unwrap_or(1);
        let idf = ((total_docs as f32 + 1.0) / (df as f32 + 1.0)).ln() + 1.0;
        tf_idf_vector[index] = tf * idf;
    }
    ic_println!("Computed TF-IDF vector with length: {}", tf_idf_vector.len());
    tf_idf_vector
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn __export_candid() -> String {
    __export_service()
}