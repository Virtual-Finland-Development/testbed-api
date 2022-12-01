use serde::{ Deserialize, Serialize };

//
// Inputs from the front app to the productizers
// 
#[derive(Deserialize, Serialize, Debug)]
pub struct JobsRequest {
    pub query: String,
    pub location: RequestLocation,
    pub paging: RequestPaging,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestLocation {
    pub countries: Vec<String>,
    pub regions: Vec<String>,
    pub municipalities: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPaging {
    pub limit: usize,
    pub offset: usize,
}

//
// Outputs from the productizer APIs
//
#[derive(Deserialize, Serialize, Debug)]
pub struct JobPostingResponse<T> {
    pub results: Vec<T>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct JobPosting {
    pub employer: String,
    pub location: Location,
    #[serde(rename = "basicInfo")]
    pub basic_info: BasicInfo,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "applicationEndDate")]
    pub application_end_date: String,
    #[serde(rename = "applicationUrl")]
    pub application_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Location {
    pub municipality: String,
    pub postcode: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct BasicInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "workTimeType")]
    pub work_time_type: String,
}

//
// Transformed outputs for the frontend app
// @TODO: there must be a better way to do this in rust
//
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct JobPostingForFrontend {
    pub id: String,
    #[serde(rename = "jobsSource")]
    pub jobs_source: String,
    pub employer: String,
    pub location: Location,
    #[serde(rename = "basicInfo")]
    pub basic_info: BasicInfo,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "applicationEndDate")]
    pub application_end_date: String,
    #[serde(rename = "applicationUrl")]
    pub application_url: Option<String>,
}