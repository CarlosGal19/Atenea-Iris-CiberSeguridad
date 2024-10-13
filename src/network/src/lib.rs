use serde::Deserialize;
use std::collections::HashMap;

mod data;
use crate::data::DATA;

#[derive(Debug, Deserialize, Clone)]
struct CrimeRecord {
    ano: String,
    clave_ent: String,
    entidad: String,
    cve_municipio: String,
    municipio: String,
    bien_juridico_afectado: String,
    tipo_de_delito: String,
    subtipo_de_delito: String,
    modalidad: String,
    enero: Option<u32>,
    febrero: Option<u32>,
    marzo: Option<u32>,
    abril: Option<u32>,
    mayo: Option<u32>,
    junio: Option<u32>,
    julio: Option<u32>,
    agosto: Option<u32>,
    septiembre: Option<u32>,
    octubre: Option<u32>,
    noviembre: Option<u32>,
    diciembre: Option<u32>,
}

fn get_crime_data() -> Vec<CrimeRecord> {
    let csv_data = DATA.trim();
    let lines: Vec<&str> = csv_data.lines().collect();

    if lines.is_empty() {
        return Vec::new();
    }

    let headers = parse_csv_line(lines[0]);
    let mut records = Vec::new();

    for &line in lines.iter().skip(1) {
        let fields = parse_csv_line(line);

        if fields.len() != headers.len() {
            continue;
        }

        let record_map: HashMap<String, String> = headers.iter().zip(fields.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        if let Ok(record) = create_crime_record(&record_map) {
            records.push(record);
        }
    }

    records
}

fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn create_crime_record(
    record_map: &std::collections::HashMap<String, String>,
) -> Result<CrimeRecord, ()> {
    let normalized_map: HashMap<String, String> = record_map
        .iter()
        .map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_string()))
        .collect();

    let tipo_de_delito = normalized_map
        .get("tipo de delito")
        .unwrap_or(&"".to_string())
        .to_string();

    if tipo_de_delito.is_empty() {
        return Err(());
    }

    Ok(CrimeRecord {
        ano: normalized_map.get("ano").unwrap_or(&"".to_string()).to_string(),
        clave_ent: normalized_map.get("clave_ent").unwrap_or(&"".to_string()).to_string(),
        entidad: normalized_map.get("entidad").unwrap_or(&"".to_string()).to_string(),
        cve_municipio: normalized_map.get("cve. municipio").unwrap_or(&"".to_string()).to_string(),
        municipio: normalized_map.get("municipio").unwrap_or(&"".to_string()).to_string(),
        bien_juridico_afectado: normalized_map
            .get("bien juridico afectado")
            .unwrap_or(&"".to_string())
            .to_string(),
        tipo_de_delito,
        subtipo_de_delito: normalized_map
            .get("subtipo de delito")
            .unwrap_or(&"".to_string())
            .to_string(),
        modalidad: normalized_map.get("modalidad").unwrap_or(&"".to_string()).to_string(),
        enero: parse_optional_u32(normalized_map.get("enero").unwrap_or(&"".to_string())),
        febrero: parse_optional_u32(normalized_map.get("febrero").unwrap_or(&"".to_string())),
        marzo: parse_optional_u32(normalized_map.get("marzo").unwrap_or(&"".to_string())),
        abril: parse_optional_u32(normalized_map.get("abril").unwrap_or(&"".to_string())),
        mayo: parse_optional_u32(normalized_map.get("mayo").unwrap_or(&"".to_string())),
        junio: parse_optional_u32(normalized_map.get("junio").unwrap_or(&"".to_string())),
        julio: parse_optional_u32(normalized_map.get("julio").unwrap_or(&"".to_string())),
        agosto: parse_optional_u32(normalized_map.get("agosto").unwrap_or(&"".to_string())),
        septiembre: parse_optional_u32(normalized_map.get("septiembre").unwrap_or(&"".to_string())),
        octubre: parse_optional_u32(normalized_map.get("octubre").unwrap_or(&"".to_string())),
        noviembre: parse_optional_u32(normalized_map.get("noviembre").unwrap_or(&"".to_string())),
        diciembre: parse_optional_u32(normalized_map.get("diciembre").unwrap_or(&"".to_string())),
    })
}

fn parse_optional_u32(s: &str) -> Option<u32> {
    if s.is_empty() {
        None
    } else {
        s.parse::<u32>().ok()
    }
}

fn map_crime_to_category(crime_type: &str) -> String {
    let crime_type = crime_type.trim().to_lowercase();

    let general_categories: HashMap<&str, &str> = [
        ("homicidio", "Delitos contra la vida"),
        ("lesiones", "Delitos contra la vida"),
        ("feminicidio", "Feminicidio"),
    ]
    .iter()
    .cloned()
    .collect();

    general_categories
        .get(crime_type.as_str())
        .unwrap_or(&"Otros delitos")
        .to_string()
}

fn aggregate_data(records: Vec<CrimeRecord>) -> HashMap<String, HashMap<String, u32>> {
    let mut aggregated_data: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for record in records {
        let total: u32 = record.enero.unwrap_or(0)
            + record.febrero.unwrap_or(0)
            + record.marzo.unwrap_or(0)
            + record.abril.unwrap_or(0)
            + record.mayo.unwrap_or(0)
            + record.junio.unwrap_or(0)
            + record.julio.unwrap_or(0)
            + record.agosto.unwrap_or(0)
            + record.septiembre.unwrap_or(0)
            + record.octubre.unwrap_or(0)
            + record.noviembre.unwrap_or(0)
            + record.diciembre.unwrap_or(0);

        let category = map_crime_to_category(&record.tipo_de_delito);
        let municipality = record.municipio.clone();

        aggregated_data
            .entry(municipality)
            .or_insert_with(HashMap::new)
            .entry(category)
            .and_modify(|e| *e += total)
            .or_insert(total);
    }

    aggregated_data
}

fn prepare_data_for_clustering(
    aggregated_data: &HashMap<String, HashMap<String, u32>>,
    categories: &Vec<String>,
) -> (Vec<String>, Vec<Vec<f64>>) {
    let mut municipalities = Vec::new();
    let mut data_matrix = Vec::new();

    for (municipality, category_totals) in aggregated_data {
        municipalities.push(municipality.clone());
        let mut data_row = Vec::new();
        for category in categories {
            let value = *category_totals.get(category).unwrap_or(&0);
            data_row.push(value as f64);
        }
        data_matrix.push(data_row);
    }

    (municipalities, data_matrix)
}

fn kmeans(data: &Vec<Vec<f64>>, k: usize, max_iterations: usize) -> Vec<usize> {
    if data.is_empty() {
        return Vec::new();
    }

    let n_samples = data.len();
    let n_features = data[0].len();

    let mut centroids = Vec::new();
    let mut unique_data = data.clone();
    unique_data.dedup();

    for i in 0..k {
        if i < unique_data.len() {
            centroids.push(unique_data[i].clone());
        } else {
            centroids.push(vec![0.0; n_features]);
        }
    }

    let mut labels = vec![0; n_samples];

    for _iteration in 0..max_iterations {
        let mut clusters_changed = false;

        for i in 0..n_samples {
            let mut min_distance = f64::MAX;
            let mut min_index = 0;
            for (j, centroid) in centroids.iter().enumerate() {
                let distance = euclidean_distance(&data[i], centroid);
                if distance < min_distance {
                    min_distance = distance;
                    min_index = j;
                }
            }
            if labels[i] != min_index {
                clusters_changed = true;
                labels[i] = min_index;
            }
        }

        if !clusters_changed {
            break;
        }

        let mut new_centroids = vec![vec![0.0; n_features]; k];
        let mut counts = vec![0; k];

        for i in 0..n_samples {
            let label = labels[i];
            counts[label] += 1;
            for j in 0..n_features {
                new_centroids[label][j] += data[i][j];
            }
        }

        for i in 0..k {
            if counts[i] > 0 {
                for j in 0..n_features {
                    new_centroids[i][j] /= counts[i] as f64;
                }
            } else {
                new_centroids[i] = centroids[i].clone();
            }
        }

        centroids = new_centroids;
    }

    labels
}

fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[ic_cdk::query]
fn clustering() -> String {
    let records = get_crime_data();

    if records.is_empty() {
        return serde_json::to_string_pretty(&Vec::<HashMap<String, String>>::new()).unwrap();
    }

    let aggregated_data = aggregate_data(records);

    if aggregated_data.is_empty() {
        return serde_json::to_string_pretty(&Vec::<HashMap<String, String>>::new()).unwrap();
    }

    let categories: Vec<String> = vec![
        "Delitos contra la vida".to_string(),
        "Delitos contra la libertad personal".to_string(),
        "Delitos contra la seguridad sexual".to_string(),
        "Delitos contra el patrimonio".to_string(),
        "Delitos contra la familia".to_string(),
        "Delitos contra la sociedad".to_string(),
        "Delitos contra la salud publica".to_string(),
        "Otros delitos".to_string(),
        "Feminicidio".to_string(),
    ];

    let (municipalities, mut data_matrix) =
        prepare_data_for_clustering(&aggregated_data, &categories);

    if data_matrix.is_empty() || municipalities.is_empty() {
        return serde_json::to_string_pretty(&Vec::<HashMap<String, String>>::new()).unwrap();
    }

    for row in &mut data_matrix {
        for value in row.iter_mut() {
            *value = value.ln_1p();
        }
    }

    let n_features = data_matrix[0].len();
    let mut means = vec![0.0; n_features];
    let mut std_devs = vec![0.0; n_features];
    let n_samples = data_matrix.len() as f64;

    for row in &data_matrix {
        for (i, value) in row.iter().enumerate() {
            means[i] += value;
        }
    }
    for mean in &mut means {
        *mean /= n_samples;
    }

    for row in &data_matrix {
        for (i, value) in row.iter().enumerate() {
            std_devs[i] += (value - means[i]).powi(2);
        }
    }
    for std_dev in &mut std_devs {
        *std_dev = (*std_dev / n_samples).sqrt();
        if *std_dev == 0.0 {
            *std_dev = 1.0;
        }
    }

    for row in &mut data_matrix {
        for (i, value) in row.iter_mut().enumerate() {
            *value = (*value - means[i]) / std_devs[i];
        }
    }

    let k = 4;
    let max_iterations = 100;
    let labels = kmeans(&data_matrix, k, max_iterations);

    let mut results = Vec::new();
    for (i, municipality) in municipalities.iter().enumerate() {
        let mut data_point = HashMap::new();
        data_point.insert("Municipio".to_string(), municipality.clone());
        data_point.insert("Cluster".to_string(), labels[i].to_string());

        for (j, category) in categories.iter().enumerate() {
            data_point.insert(category.clone(), data_matrix[i][j].to_string());
        }

        results.push(data_point);
    }

    serde_json::to_string_pretty(&results).unwrap()
}