#[cfg(test)]
mod tests {
    use crate::{ArrConfig, Config, PutioConfig};
    use figment::{
        providers::{Format, Serialized, Toml},
        Figment,
    };
    use serde_json;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "testpass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec!["sample".to_string(), "extras".to_string()],
            uid: 1000,
            username: "testuser".to_string(),
            putio: PutioConfig {
                api_key: "test_key".to_string(),
            },
            sonarr: Some(ArrConfig {
                url: "http://localhost:8989".to_string(),
                api_key: "sonarr_key".to_string(),
            }),
            radarr: Some(ArrConfig {
                url: "http://localhost:7878".to_string(),
                api_key: "radarr_key".to_string(),
            }),
            whisparr: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"bind_address\":\"0.0.0.0\""));
        assert!(json.contains("\"download_directory\":\"/downloads\""));
        assert!(json.contains("\"testuser\""));
        assert!(json.contains("\"testpass\""));
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "bind_address": "127.0.0.1",
            "download_directory": "/tmp/downloads",
            "download_workers": 8,
            "loglevel": "debug",
            "orchestration_workers": 20,
            "password": "mypass",
            "polling_interval": 30,
            "port": 8080,
            "skip_directories": ["sample"],
            "uid": 1001,
            "username": "myuser",
            "putio": {
                "api_key": "my_api_key"
            },
            "sonarr": {
                "url": "http://sonarr:8989",
                "api_key": "sonarr_api"
            },
            "radarr": null,
            "whisparr": null
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.bind_address, "127.0.0.1");
        assert_eq!(config.download_directory, "/tmp/downloads");
        assert_eq!(config.download_workers, 8);
        assert_eq!(config.loglevel, "debug");
        assert_eq!(config.orchestration_workers, 20);
        assert_eq!(config.password, "mypass");
        assert_eq!(config.polling_interval, 30);
        assert_eq!(config.port, 8080);
        assert_eq!(config.skip_directories, vec!["sample"]);
        assert_eq!(config.uid, 1001);
        assert_eq!(config.username, "myuser");
        assert_eq!(config.putio.api_key, "my_api_key");
        assert!(config.sonarr.is_some());
        assert!(config.radarr.is_none());
        assert!(config.whisparr.is_none());
    }

    #[test]
    fn test_putio_config() {
        let putio = PutioConfig {
            api_key: "test_api_key_123".to_string(),
        };

        let json = serde_json::to_string(&putio).unwrap();
        assert!(json.contains("\"api_key\":\"test_api_key_123\""));
    }

    #[test]
    fn test_arr_config() {
        let arr = ArrConfig {
            url: "http://localhost:8989/sonarr".to_string(),
            api_key: "sonarr_key_456".to_string(),
        };

        let json = serde_json::to_string(&arr).unwrap();
        assert!(json.contains("\"url\":\"http://localhost:8989/sonarr\""));
        assert!(json.contains("\"api_key\":\"sonarr_key_456\""));
    }

    #[test]
    fn test_config_with_all_arr_services() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "pass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec![],
            uid: 1000,
            username: "user".to_string(),
            putio: PutioConfig {
                api_key: "key".to_string(),
            },
            sonarr: Some(ArrConfig {
                url: "http://sonarr:8989".to_string(),
                api_key: "sonarr_key".to_string(),
            }),
            radarr: Some(ArrConfig {
                url: "http://radarr:7878".to_string(),
                api_key: "radarr_key".to_string(),
            }),
            whisparr: Some(ArrConfig {
                url: "http://whisparr:6969".to_string(),
                api_key: "whisparr_key".to_string(),
            }),
        };

        assert!(config.sonarr.is_some());
        assert!(config.radarr.is_some());
        assert!(config.whisparr.is_some());
    }

    #[test]
    fn test_config_with_no_arr_services() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "pass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec![],
            uid: 1000,
            username: "user".to_string(),
            putio: PutioConfig {
                api_key: "key".to_string(),
            },
            sonarr: None,
            radarr: None,
            whisparr: None,
        };

        assert!(config.sonarr.is_none());
        assert!(config.radarr.is_none());
        assert!(config.whisparr.is_none());
    }

    #[test]
    fn test_config_toml_parsing() {
        let toml_content = r#"
username = "testuser"
password = "testpass"
download_directory = "/downloads"
bind_address = "0.0.0.0"
port = 9091
loglevel = "info"
uid = 1000
polling_interval = 10
skip_directories = ["sample", "extras"]
orchestration_workers = 10
download_workers = 4

[putio]
api_key = "test_api_key"

[sonarr]
url = "http://localhost:8989"
api_key = "sonarr_key"
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), toml_content).unwrap();

        let config: Config = Figment::new()
            .merge(Toml::file(temp_file.path()))
            .extract()
            .unwrap();

        assert_eq!(config.username, "testuser");
        assert_eq!(config.password, "testpass");
        assert_eq!(config.download_directory, "/downloads");
        assert_eq!(config.putio.api_key, "test_api_key");
        assert!(config.sonarr.is_some());
        assert_eq!(config.sonarr.unwrap().api_key, "sonarr_key");
    }

    #[test]
    fn test_config_with_defaults() {
        let config: Config = Figment::new()
            .join(Serialized::default("bind_address", "0.0.0.0"))
            .join(Serialized::default("download_workers", 4))
            .join(Serialized::default("orchestration_workers", 10))
            .join(Serialized::default("loglevel", "info"))
            .join(Serialized::default("polling_interval", 10))
            .join(Serialized::default("port", 9091))
            .join(Serialized::default("uid", 1000))
            .join(Serialized::default(
                "skip_directories",
                vec!["sample", "extras"],
            ))
            .join(Serialized::default("username", "default_user"))
            .join(Serialized::default("password", "default_pass"))
            .join(Serialized::default("download_directory", "/tmp"))
            .join(Serialized::default(
                "putio",
                PutioConfig {
                    api_key: "default_key".to_string(),
                },
            ))
            .extract()
            .unwrap();

        assert_eq!(config.bind_address, "0.0.0.0");
        assert_eq!(config.download_workers, 4);
        assert_eq!(config.orchestration_workers, 10);
        assert_eq!(config.loglevel, "info");
        assert_eq!(config.polling_interval, 10);
        assert_eq!(config.port, 9091);
        assert_eq!(config.uid, 1000);
        assert_eq!(config.skip_directories, vec!["sample", "extras"]);
    }

    #[test]
    fn test_config_skip_directories_empty() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "pass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec![],
            uid: 1000,
            username: "user".to_string(),
            putio: PutioConfig {
                api_key: "key".to_string(),
            },
            sonarr: None,
            radarr: None,
            whisparr: None,
        };

        assert!(config.skip_directories.is_empty());
    }

    #[test]
    fn test_config_skip_directories_multiple() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "pass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec![
                "sample".to_string(),
                "extras".to_string(),
                "behind the scenes".to_string(),
                "deleted scenes".to_string(),
            ],
            uid: 1000,
            username: "user".to_string(),
            putio: PutioConfig {
                api_key: "key".to_string(),
            },
            sonarr: None,
            radarr: None,
            whisparr: None,
        };

        assert_eq!(config.skip_directories.len(), 4);
        assert!(config.skip_directories.contains(&"sample".to_string()));
        assert!(config
            .skip_directories
            .contains(&"behind the scenes".to_string()));
    }

    #[test]
    fn test_config_different_ports() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "pass".to_string(),
            polling_interval: 10,
            port: 8080,
            skip_directories: vec![],
            uid: 1000,
            username: "user".to_string(),
            putio: PutioConfig {
                api_key: "key".to_string(),
            },
            sonarr: None,
            radarr: None,
            whisparr: None,
        };

        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_config_different_bind_addresses() {
        let addresses = vec!["0.0.0.0", "127.0.0.1", "192.168.1.100", "::1"];

        for addr in addresses {
            let config = Config {
                bind_address: addr.to_string(),
                download_directory: "/downloads".to_string(),
                download_workers: 4,
                loglevel: "info".to_string(),
                orchestration_workers: 10,
                password: "pass".to_string(),
                polling_interval: 10,
                port: 9091,
                skip_directories: vec![],
                uid: 1000,
                username: "user".to_string(),
                putio: PutioConfig {
                    api_key: "key".to_string(),
                },
                sonarr: None,
                radarr: None,
                whisparr: None,
            };

            assert_eq!(config.bind_address, addr);
        }
    }

    #[test]
    fn test_config_different_log_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error"];

        for level in levels {
            let config = Config {
                bind_address: "0.0.0.0".to_string(),
                download_directory: "/downloads".to_string(),
                download_workers: 4,
                loglevel: level.to_string(),
                orchestration_workers: 10,
                password: "pass".to_string(),
                polling_interval: 10,
                port: 9091,
                skip_directories: vec![],
                uid: 1000,
                username: "user".to_string(),
                putio: PutioConfig {
                    api_key: "key".to_string(),
                },
                sonarr: None,
                radarr: None,
                whisparr: None,
            };

            assert_eq!(config.loglevel, level);
        }
    }

    #[test]
    fn test_config_clone() {
        let config = Config {
            bind_address: "0.0.0.0".to_string(),
            download_directory: "/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "testpass".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec!["sample".to_string()],
            uid: 1000,
            username: "testuser".to_string(),
            putio: PutioConfig {
                api_key: "test_key".to_string(),
            },
            sonarr: None,
            radarr: None,
            whisparr: None,
        };

        let cloned = config.clone();
        assert_eq!(config.username, cloned.username);
        assert_eq!(config.password, cloned.password);
        assert_eq!(config.port, cloned.port);
        assert_eq!(config.putio.api_key, cloned.putio.api_key);
    }

    #[test]
    fn test_putio_config_clone() {
        let putio = PutioConfig {
            api_key: "test_key".to_string(),
        };

        let cloned = putio.clone();
        assert_eq!(putio.api_key, cloned.api_key);
    }

    #[test]
    fn test_arr_config_clone() {
        let arr = ArrConfig {
            url: "http://localhost:8989".to_string(),
            api_key: "test_key".to_string(),
        };

        let cloned = arr.clone();
        assert_eq!(arr.url, cloned.url);
        assert_eq!(arr.api_key, cloned.api_key);
    }
}
