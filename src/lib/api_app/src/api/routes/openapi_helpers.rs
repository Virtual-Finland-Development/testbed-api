use utoipa::openapi::{OpenApi, PathItem, PathItemType};

pub fn get_openapi_operation_id(openapi: OpenApi, method: &str, path: &str) -> String {
    let path = openapi.paths.get_path_item(path);
    match path {
        Some(path) => resolve_operation_id(path, method),
        None => "".to_string(),
    }
}

fn resolve_operation_id(path: &PathItem, method: &str) -> String {
    let path_item_type = get_path_item_type(method);
    let operationable = path.operations.get(&path_item_type);
    if operationable.is_none() {
        return "".to_string();
    }
    let operation = operationable.unwrap();
    let operation_id = operation.operation_id.clone();
    operation_id.unwrap_or_default()
}

fn get_path_item_type(method: &str) -> PathItemType {
    match method {
        "GET" => PathItemType::Get,
        "POST" => PathItemType::Post,
        "PUT" => PathItemType::Put,
        "DELETE" => PathItemType::Delete,
        "OPTIONS" => PathItemType::Options,
        "HEAD" => PathItemType::Head,
        "PATCH" => PathItemType::Patch,
        "TRACE" => PathItemType::Trace,
        _ => PathItemType::Get,
    }
}
