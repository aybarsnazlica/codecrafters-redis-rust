#[derive(Debug)]
pub struct BulkString {
    pub cmd: String,
    pub value1: String,
    pub value2: String,
}

pub async fn parse(request: &str) -> BulkString {
    let parts: Vec<&str> = request.split("\r\n").collect();
    let command = parts.get(2).unwrap_or(&"").to_lowercase();

    if parts.len() >= 6 {
        return BulkString {
            cmd: command,
            value1: parts.get(4).unwrap_or(&"").to_string(),
            value2: parts.get(6).unwrap_or(&"").to_string(),
        };
    } else if parts.len() >= 5 {
        return BulkString {
            cmd: command,
            value1: parts.get(4).unwrap_or(&"").to_string(),
            value2: "".to_string(),
        };
    }

    BulkString {
        cmd: command,
        value1: "".to_string(),
        value2: "".to_string(),
    }
}
