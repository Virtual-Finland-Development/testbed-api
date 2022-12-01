use serde::{ Deserialize, Serialize };

/**
 * Population query parameters
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct PopulationQuery {
    city: String,
    year: String, // Note: front apps send strings, not numbers
}

/**
 * Population response
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct PopulationResponse {
    description: String,
    #[serde(rename = "sourceName")]
    source_name: String,
    population: i128,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}
