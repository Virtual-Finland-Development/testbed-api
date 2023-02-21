use itertools::Itertools;
use serde::Deserialize;
use serde_json::from_str as deserialize_json_from_string;

//
// Inputs from the frontend
//
#[derive(Deserialize)]
struct Occupation {
    pub uri: Option<String>,
    pub broader: Option<Vec<String>>,
}

pub fn extend_job_occupations(occupations: Vec<String>) -> Vec<String> {
    let mut extended_occupations: Vec<String> = Vec::new();
    let codesets = get_esco_occupations_codeset();

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

fn get_esco_occupations_codeset() -> Vec<Occupation> {
    let contents =
        include_str!("../../../../resources/testbed/jobs/esco-1.1.0-occupations.json");
    deserialize_json_from_string::<Vec<Occupation>>(contents)
        .expect("Failed to parse the ESCO occupations codeset")
}
