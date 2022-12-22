use http::HeaderMap;
use serde::{ Deserialize, Serialize };

//
// Inputs from the frontend
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JobsRequestFromFrontend {
    pub query: String,
    pub location: RequestLocation,
    pub requirements: RequestRequirements,
    pub paging: RequestPagingFromFrontend,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestPagingFromFrontend {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
}

//
// Inputs to the productizers
//
#[derive(Debug)]
pub struct ProductizerRequest {
    pub endpoint_urls: Vec<String>,
    pub request_input: JobsRequest,
    pub headers: HeaderMap,
    pub original_input: JobsRequestFromFrontend,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JobsRequest {
    pub query: String,
    pub location: RequestLocation,
    pub requirements: RequestRequirements,
    pub paging: RequestPaging,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestLocation {
    pub countries: Vec<String>,
    pub regions: Vec<String>,
    pub municipalities: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestRequirements {
    pub occupations: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPaging {
    pub limit: i32,
    pub offset: i32,
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