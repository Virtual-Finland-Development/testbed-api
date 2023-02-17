pub mod strings {
    pub fn truncate_too_long_string(
        string: impl Into<String>,
        max_length: usize,
        postfix: &str,
    ) -> String {
        let text = string.into();
        if text.len() > max_length {
            return text[..max_length].to_string() + postfix;
        }
        text
    }

    pub fn cut_string_by_delimiter_keep_right(
        string: impl Into<String>,
        delimiter: &str,
    ) -> String {
        let text = string.into();
        let split = text.split(delimiter);
        split.last().unwrap().to_string()
    }

    pub fn trim_left_slashes(text: impl Into<String>) -> String {
        let mut result = text.into();
        while result.starts_with('/') {
            result = result[1..].to_string();
        }
        result
    }

    pub fn parse_comma_separated_list(string: impl Into<String>) -> Vec<String> {
        let text = string.into();
        let split = text.split(',');
        let result: Vec<String> = split.map(|s| s.trim().to_string()).collect();
        result
    }
}

pub mod api {
    use http::{
        header::{HeaderMap, HeaderName},
        HeaderValue,
    };

    /**
     * Cors preflight response headers.
     */
    pub fn get_cors_response_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_static("access-control-allow-origin"),
            HeaderValue::from_static("*"),
        );

        headers.insert(
            HeaderName::from_static("access-control-allow-methods"),
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );

        headers.insert(
            HeaderName::from_static("access-control-allow-headers"),
            HeaderValue::from_static(
                "content-type, authorization, x-authorization-provider, x-authorization-context, x-consent-token"
            )
        );

        headers
    }

    pub fn get_default_headers() -> HeaderMap {
        let mut cors_headers = get_cors_response_headers();

        cors_headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );

        cors_headers
    }

    pub fn get_plain_headers() -> HeaderMap {
        let mut cors_headers = get_cors_response_headers();

        cors_headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/plain"),
        );

        cors_headers
    }
}

pub mod environment {
    pub fn get_stage() -> String {
        std::env::var("STAGE").unwrap_or_else(|_| "local".to_string())
    }
}
