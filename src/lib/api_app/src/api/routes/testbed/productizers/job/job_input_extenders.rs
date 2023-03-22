use lazy_static::lazy_static;
use memory_cache::MemoryCache;

use std::{env, sync::Mutex};

use itertools::Itertools;
use serde::Deserialize;
use serde_json::from_str as deserialize_json_from_string;

//
// Inputs from the frontend
//
#[derive(Deserialize, Clone)]
struct Occupation {
    pub uri: Option<String>,
    pub broader: Option<Vec<String>>,
}

pub async fn extend_job_occupations(occupations: Vec<String>) -> Vec<String> {
    let mut extended_occupations: Vec<String> = Vec::new();
    let codesets = get_esco_occupations_codeset().await;

    for occupation_uri in occupations {
        if !occupation_uri.contains("://data.europa.eu/esco/occupation/") {
            let mut extended_occupation_uris =
                extend_job_occupation_uris(occupation_uri, &codesets);
            extended_occupations.append(&mut extended_occupation_uris);
        } else {
            extended_occupations.push(occupation_uri);
        }
    }
    extended_occupations.into_iter().unique().collect()
}

fn extend_job_occupation_uris(
    occupation_uri: String,
    codesets: &Vec<Occupation>,
) -> Vec<String> {
    let mut extended_occupation_uris: Vec<String> = Vec::new();

    for codeset in codesets {
        if codeset.uri.is_none() || codeset.broader.is_none() {
            continue;
        }

        let uri = codeset
            .uri
            .clone()
            .expect("Failed to retrieve the URI of the ESCO occupation codeset");
        let broader = codeset
            .broader
            .clone()
            .expect("Failed to retrieve the broader of the ESCO occupation codeset");

        if broader.contains(&occupation_uri) {
            extended_occupation_uris.push(uri.clone());
            let mut sub_occupation_uris = extend_job_occupation_uris(uri, codesets);
            extended_occupation_uris.append(&mut sub_occupation_uris);
        }
    }
    extended_occupation_uris
}

async fn get_esco_occupations_codeset() -> Vec<Occupation> {
    let cache_key = String::from("esco_occupations_codeset");

    lazy_static! {
        static ref CACHE: Mutex<MemoryCache<String, Vec<Occupation>>> =
            Mutex::new(MemoryCache::new());
    }

    if let Some(codesets) = CACHE.lock().unwrap().get(&cache_key) {
        return codesets.to_vec();
    }

    let codesets_base_url =
        env::var("CODESETS_BASE_URL").expect("CODESETS_BASE_URL must be set");
    let codesets_occupation_url = format!("{}/resources/OccupationsEscoURL", codesets_base_url);
    let contents = reqwest::get(codesets_occupation_url)
        .await
        .expect("Failed to get the ESCO occupations codeset")
        .text()
        .await
        .expect("Failed to parse the ESCO occupations codeset");

    let occupations = deserialize_json_from_string::<Vec<Occupation>>(&contents)
        .expect("Failed to deserialize the ESCO occupations codeset");

    CACHE
        .lock()
        .unwrap()
        .insert(cache_key, occupations.clone(), None);

    occupations
}
