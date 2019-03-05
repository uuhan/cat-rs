use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub servers: Vec<ServerConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    #[serde(rename(deserialize = "http-port"))]
    pub httpPort: u16,
}

#[cfg(test)]
mod tests {
    use super::ClientConfig;
    use serde_json::Error;

    #[test]
    fn test_client_config() {
        let data = r#"{
         "servers": [
            {
              "ip": "127.0.0.1",
              "port": 2280,
              "http-port": 2040
            },
            {
              "ip": "127.0.0.1",
              "port": 2280,
              "http-port": 2040
            },
            {
              "ip": "127.0.0.1",
              "port": 2280,
              "http-port": 2040
            }
          ]
        }"#;

        let config: ClientConfig = serde_json::from_str(data).unwrap();
        assert_eq!(config.servers.len(), 3);
    }
}
