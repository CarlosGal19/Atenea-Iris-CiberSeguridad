use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::call::call;
use ic_cdk::println as ic_println;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Tipo para identificar documentos
type DocumentId = u64;

// Definición de errores personalizados
#[derive(CandidType, Deserialize, Debug)]
enum CustomError {
    MemoryError,
    UniqueViolation,
    DimensionMismatch,
    NotFound,
}

// Resultados personalizados
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

// Definición de la estructura del canister NLP
#[derive(Default)]
struct NlpCanister {
    vocabulary: HashMap<String, usize>, // Diccionario predefinido
    doc_freqs: HashMap<usize, u64>,     // Frecuencia de documentos por término
    total_docs: u64,                    // Total de documentos procesados
}

// Inicialización del estado del canister
thread_local! {
    static NLP_CANISTER: RefCell<NlpCanister> = RefCell::new(NlpCanister::default());
}

// Definición del diccionario predefinido con 50 palabras clave
static PREDEFINED_VOCABULARY: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let mut vocab = HashMap::new();
    vocab.insert("robo".to_string(), 0);
    vocab.insert("policía".to_string(), 1);
    vocab.insert("unidad".to_string(), 2);
    vocab.insert("vehículo".to_string(), 3);
    vocab.insert("patrulla".to_string(), 4);
    vocab.insert("reporte".to_string(), 5);
    vocab.insert("circulacion".to_string(), 6);
    vocab.insert("modelo".to_string(), 7);
    vocab.insert("color".to_string(), 8);
    vocab.insert("placa".to_string(), 9);
    vocab.insert("accidente".to_string(), 10);
    vocab.insert("camión".to_string(), 11);
    vocab.insert("ambulancia".to_string(), 12);
    vocab.insert("paramédico".to_string(), 13);
    vocab.insert("extorsión".to_string(), 14);
    vocab.insert("grua".to_string(), 15);
    vocab.insert("homicidio".to_string(), 16);
    vocab.insert("secuestro".to_string(), 17);
    vocab.insert("fuga".to_string(), 18);
    vocab.insert("detención".to_string(), 19);
    vocab.insert("investigador".to_string(), 20);
    vocab.insert("víctima".to_string(), 21);
    vocab.insert("delito".to_string(), 22);
    vocab.insert("arma".to_string(), 23);
    vocab.insert("evidencia".to_string(), 24);
    vocab.insert("testigo".to_string(), 25);
    vocab.insert("investigación".to_string(), 26);
    vocab.insert("ubicación".to_string(), 27);
    vocab.insert("fecha".to_string(), 28);
    vocab.insert("hora".to_string(), 29);
    vocab.insert("estado".to_string(), 30);
    vocab.insert("colonia".to_string(), 31);
    vocab.insert("calle".to_string(), 32);
    vocab.insert("fraccionamiento".to_string(), 33);
    vocab.insert("distrito".to_string(), 34);
    vocab.insert("sistema".to_string(), 35);
    vocab.insert("folio".to_string(), 36);
    vocab.insert("expediente".to_string(), 37);
    vocab.insert("marca".to_string(), 38);
    vocab.insert("nissan".to_string(), 39);
    vocab.insert("honda".to_string(), 40);
    vocab.insert("ford".to_string(), 41);
    vocab.insert("chevrolet".to_string(), 42);
    vocab.insert("volkswagen".to_string(), 43);
    vocab.insert("plataforma".to_string(), 44);
    vocab.insert("denunciante".to_string(), 45);
    vocab.insert("denunciado".to_string(), 46);
    vocab.insert("llamada".to_string(), 47);
    vocab.insert("violencia".to_string(), 48);
    vocab.insert("extensión".to_string(), 49);
    // Total: 50 palabras
    vocab
});

// ID del canister de la base de datos vectorial
static VECTORDB_CANISTER_ID: Lazy<Principal> = Lazy::new(|| {
    // Reemplaza con el ID real de tu canister vectordb
    Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap()
});

// Inicialización del canister para establecer el diccionario predefinido
#[init]
fn init() {
    NLP_CANISTER.with(|canister| {
        let mut canister = canister.borrow_mut();
        canister.vocabulary = PREDEFINED_VOCABULARY.clone();
    });
}

// Función para procesar texto y generar el vector TF-IDF
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
            if let Some(&index) = canister.vocabulary.get(&token_lower) {
                *term_counts.entry(index).or_insert(0u64) += 1;
            }
            // Ignorar palabras no presentes en el vocabulario predefinido
        }

        for &index in term_counts.keys() {
            *canister.doc_freqs.entry(index).or_insert(0u64) += 1;
        }

        ic_println!(
            "Vocabulary size after processing: {}",
            canister.vocabulary.len()
        );
        ic_println!("Term counts: {:?}", term_counts);
        ic_println!("Document frequencies: {:?}", canister.doc_freqs);

        let tf_idf_vector = compute_tf_idf(&term_counts, &canister.doc_freqs, canister.total_docs);
        let vector_dimension = tf_idf_vector.len();

        (tf_idf_vector, vector_dimension)
    });

    ic_println!("Computed TF-IDF vector: {:?}", tf_idf_vector);
    ic_println!("Vector dimension: {}", vector_dimension);

    let collection_name = "police_reports_v2".to_string(); // Nuevo nombre de colección

    // Verificar si la colección existe
    let collections: Vec<String> =
        match call::<(), (CustomResult2,)>(*VECTORDB_CANISTER_ID, "get_collections", ()).await {
            Ok((CustomResult2::Ok(collections),)) => {
                ic_println!("Retrieved collections: {:?}", collections);
                collections
            }
            Ok((CustomResult2::Err(error),)) => {
                ic_println!("Error fetching collections from vectordb: {:?}", error);
                return tf_idf_vector;
            }
            Err((rejection_code, msg)) => {
                ic_println!(
                    "Failed to call vectordb.get_collections: {:?}, {}",
                    rejection_code,
                    msg
                );
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
                ic_println!(
                    "Failed to call vectordb.create_collection: {:?}, {}",
                    rejection_code,
                    msg
                );
                return tf_idf_vector;
            }
        }
    } else {
        ic_println!("Collection '{}' already exists.", collection_name);
    }

    // Insertar el documento en vectordb
    ic_println!(
        "Inserting document into vectordb collection '{}'",
        collection_name
    );
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
        Ok((CustomResult::Err(error),)) => {
            ic_println!("Error inserting data into vectordb: {:?}", error);
        }
        Err((rejection_code, msg)) => {
            ic_println!(
                "Failed to call vectordb.insert: {:?}, {}",
                rejection_code,
                msg
            );
        }
    }

    tf_idf_vector
}

// Función para encontrar reportes similares
#[update]
#[candid_method(update)]
async fn find_similar(text: String, top_k: i32) -> Vec<String> {
    ic_println!("Starting similarity search for: '{}'", text);

    let (query_vector, vector_dimension) = NLP_CANISTER.with(|canister| {
        let canister = canister.borrow();
        ic_println!(
            "Vocabulary size when finding similar: {}",
            canister.vocabulary.len()
        );

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

    let collection_name = "police_reports_v2".to_string(); // Mismo nombre de colección nuevo

    // Verificar la dimensión de la colección
    let collections: Vec<String> =
        match call::<(), (CustomResult2,)>(*VECTORDB_CANISTER_ID, "get_collections", ()).await {
            Ok((CustomResult2::Ok(collections),)) => {
                ic_println!("Retrieved collections: {:?}", collections);
                collections
            }
            Ok((CustomResult2::Err(error),)) => {
                ic_println!("Error fetching collections from vectordb: {:?}", error);
                return vec![];
            }
            Err((rejection_code, msg)) => {
                ic_println!(
                    "Failed to call vectordb.get_collections: {:?}, {}",
                    rejection_code,
                    msg
                );
                return vec![];
            }
        };

    if !collections.contains(&collection_name) {
        ic_println!("Collection '{}' not found.", collection_name);
        return vec![];
    }

    // Realizar la consulta en vectordb
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
        }
        Ok((CustomResult2::Err(error),)) => {
            ic_println!("Error querying vectordb: {:?}", error);
            vec![]
        }
        Err((rejection_code, msg)) => {
            ic_println!(
                "Failed to call vectordb.find_query: {:?}, {}",
                rejection_code,
                msg
            );
            vec![]
        }
    }
}

// Función para tokenizar el texto
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

// Función para calcular el vector TF-IDF
fn compute_tf_idf(
    term_counts: &HashMap<usize, u64>,
    doc_freqs: &HashMap<usize, u64>,
    total_docs: u64,
) -> Vec<f32> {
    let vocab_size = PREDEFINED_VOCABULARY.len();
    let mut tf_idf_vector = vec![0.0f32; vocab_size];
    let total_terms_in_doc: u64 = term_counts.values().sum();

    for (&index, &count) in term_counts {
        let tf = count as f32 / total_terms_in_doc as f32;
        let df = doc_freqs.get(&index).cloned().unwrap_or(1);
        let idf = ((total_docs as f32 + 1.0) / (df as f32 + 1.0)).ln() + 1.0;
        if index < vocab_size {
            tf_idf_vector[index] = tf * idf;
        }
    }
    ic_println!(
        "Computed TF-IDF vector with length: {}",
        tf_idf_vector.len()
    );
    tf_idf_vector
}
