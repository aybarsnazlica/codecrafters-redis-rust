#[derive(Debug)]
pub struct BulkString {
    pub cmd: String,
    pub value: String,
}

pub async fn parse(request: &str) -> BulkString {
    let parts: Vec<&str> = request.split("\r\n").collect();
    let command = parts[2];

    if parts.len() >= 5 {
        return BulkString {
            cmd: command.to_string().to_lowercase(),
            value: parts[4].to_string(),
        };
    }

    BulkString {
        cmd: command.to_string().to_lowercase(),
        value: "".to_string(),
    }
}
