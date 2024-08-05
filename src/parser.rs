#[derive(Debug)]
pub struct BulkString {
    pub cmd: String,
    pub value1: String,
    pub value2: String,
    pub expiration: Option<u64>, // in milliseconds
}

pub async fn parse(request: &str) -> BulkString {
    let parts: Vec<&str> = request.split("\r\n").collect();
    let command = parts.get(2).unwrap_or(&"").to_lowercase();
    let mut bulk_string = BulkString {
        cmd: command,
        value1: "".to_string(),
        value2: "".to_string(),
        expiration: None,
    };

    if parts.len() >= 6 {
        bulk_string.value1 = parts.get(4).unwrap_or(&"").to_string();
        bulk_string.value2 = parts.get(6).unwrap_or(&"").to_string();
    } else if parts.len() >= 5 {
        bulk_string.value1 = parts.get(4).unwrap_or(&"").to_string();
    }

    // Check for expiration
    if parts.len() >= 10 && parts.get(8).unwrap_or(&"") == &"px" {
        if let Ok(exp) = parts.get(10).unwrap_or(&"").parse::<u64>() {
            bulk_string.expiration = Some(exp);
        }
    }

    bulk_string
}
