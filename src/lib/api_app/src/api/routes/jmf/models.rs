use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, ToSchema)]
pub struct Skill {
    pub uri: String,
    pub score: f64,
    pub label: String,
    #[serde(rename = "skillType")]
    pub skill_type: String,
    #[serde(rename = "reuseLevel")]
    pub reuse_level: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, ToSchema)]
pub struct Occupation {
    pub uri: String,
    pub score: f64,
    pub label: String,
}

/**
 * Request model
 */
#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct RecommendationsRequest {
    text: String,
    #[serde(rename = "maxNumberOfSkills")]
    max_number_of_skills: i32,
    #[serde(rename = "maxNumberOfOccupations")]
    max_number_of_occupations: i32,
    language: String,
}

/**
 * Response model
 */
#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct RecommendationsResponse {
    skills: Vec<Skill>,
    occupations: Vec<Occupation>,
    language: String,
}
