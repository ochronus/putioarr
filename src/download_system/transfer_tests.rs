#[cfg(test)]
mod tests {
    use super::super::transfer::*;
    use crate::{services::putio::PutIOTransfer, AppData, ArrConfig, Config, PutioConfig};
    use actix_web::web;

    fn create_test_config() -> Config {
        Config {
            bind_address: "127.0.0.1".to_string(),
            download_directory: "/tmp/downloads".to_string(),
            download_workers: 4,
            loglevel: "info".to_string(),
            orchestration_workers: 10,
            password: "test".to_string(),
            polling_interval: 10,
            port: 9091,
            skip_directories: vec!["sample".to_string(), "extras".to_string()],
            uid: 1000,
            username: "test".to_string(),
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
        }
    }

    fn create_test_app_data() -> web::Data<AppData> {
        web::Data::new(AppData {
            config: create_test_config(),
        })
    }

    #[test]
    fn test_download_target_display() {
        let target = DownloadTarget {
            from: Some("https://example.com/file.mp4".to_string()),
            to: "/downloads/test/file.mp4".to_string(),
            target_type: TargetType::File,
            top_level: true,
            transfer_hash: "abcd1234".to_string(),
        };

        let display = format!("{}", target);
        assert!(display.contains("abcd"));
        assert!(display.contains("/downloads/test/file.mp4"));
    }

    #[test]
    fn test_download_target_file() {
        let target = DownloadTarget {
            from: Some("https://example.com/file.mp4".to_string()),
            to: "/downloads/file.mp4".to_string(),
            target_type: TargetType::File,
            top_level: false,
            transfer_hash: "xyz789".to_string(),
        };

        assert_eq!(target.target_type, TargetType::File);
        assert!(!target.top_level);
        assert_eq!(
            target.from,
            Some("https://example.com/file.mp4".to_string())
        );
    }

    #[test]
    fn test_download_target_directory() {
        let target = DownloadTarget {
            from: None,
            to: "/downloads/folder".to_string(),
            target_type: TargetType::Directory,
            top_level: true,
            transfer_hash: "hash123".to_string(),
        };

        assert_eq!(target.target_type, TargetType::Directory);
        assert!(target.top_level);
        assert!(target.from.is_none());
    }

    #[test]
    fn test_target_type_equality() {
        assert_eq!(TargetType::File, TargetType::File);
        assert_eq!(TargetType::Directory, TargetType::Directory);
        assert_ne!(TargetType::File, TargetType::Directory);
    }

    #[test]
    fn test_download_target_serialization() {
        let target = DownloadTarget {
            from: Some("https://example.com/test.mp4".to_string()),
            to: "/downloads/test.mp4".to_string(),
            target_type: TargetType::File,
            top_level: true,
            transfer_hash: "abc123".to_string(),
        };

        let json = serde_json::to_string(&target).unwrap();
        assert!(json.contains("\"from\":"));
        assert!(json.contains("\"to\":"));
        assert!(json.contains("\"target_type\":"));
    }

    #[test]
    fn test_download_target_deserialization() {
        let json = r#"{
            "from": "https://example.com/video.mp4",
            "to": "/downloads/video.mp4",
            "target_type": "File",
            "top_level": true,
            "transfer_hash": "def456"
        }"#;

        let target: DownloadTarget = serde_json::from_str(json).unwrap();
        assert_eq!(
            target.from,
            Some("https://example.com/video.mp4".to_string())
        );
        assert_eq!(target.to, "/downloads/video.mp4");
        assert_eq!(target.target_type, TargetType::File);
        assert!(target.top_level);
        assert_eq!(target.transfer_hash, "def456");
    }

    #[test]
    fn test_download_target_directory_serialization() {
        let target = DownloadTarget {
            from: None,
            to: "/downloads/season1".to_string(),
            target_type: TargetType::Directory,
            top_level: false,
            transfer_hash: "ghi789".to_string(),
        };

        let json = serde_json::to_string(&target).unwrap();
        let deserialized: DownloadTarget = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.from, None);
        assert_eq!(deserialized.to, "/downloads/season1");
        assert_eq!(deserialized.target_type, TargetType::Directory);
        assert!(!deserialized.top_level);
    }

    #[test]
    fn test_transfer_from_putio_transfer() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 123,
            hash: Some("abc123def456".to_string()),
            name: Some("Test Movie".to_string()),
            size: Some(1000000),
            downloaded: Some(1000000),
            finished_at: Some("2024-01-01T00:00:00".to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(456),
            userfile_exists: true,
        };

        let transfer = Transfer::from(app_data, &putio_transfer);

        assert_eq!(transfer.transfer_id, 123);
        assert_eq!(transfer.name, "Test Movie");
        assert_eq!(transfer.file_id, Some(456));
        assert_eq!(transfer.hash, Some("abc123def456".to_string()));
        assert!(transfer.targets.is_none());
    }

    #[test]
    fn test_transfer_from_putio_transfer_without_name() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 789,
            hash: Some("xyz789".to_string()),
            name: None,
            size: Some(5000),
            downloaded: Some(2500),
            finished_at: None,
            estimated_time: Some(100),
            status: "DOWNLOADING".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(999),
            userfile_exists: false,
        };

        let transfer = Transfer::from(app_data, &putio_transfer);

        assert_eq!(transfer.name, "Unknown");
    }

    #[test]
    fn test_transfer_display() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 123,
            hash: Some("abcdef1234567890".to_string()),
            name: Some("My Transfer".to_string()),
            size: Some(1000),
            downloaded: Some(500),
            finished_at: None,
            estimated_time: Some(50),
            status: "DOWNLOADING".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(1),
            userfile_exists: false,
        };

        let transfer = Transfer::from(app_data, &putio_transfer);
        let display = format!("{}", transfer);

        assert!(display.contains("abcd")); // First 4 chars of hash
        assert!(display.contains("My Transfer"));
    }

    #[test]
    fn test_transfer_display_without_hash() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 456,
            hash: None,
            name: Some("No Hash Transfer".to_string()),
            size: Some(1000),
            downloaded: Some(1000),
            finished_at: Some("2024-01-01T00:00:00".to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(789),
            userfile_exists: true,
        };

        let transfer = Transfer::from(app_data, &putio_transfer);
        let display = format!("{}", transfer);

        assert!(display.contains("0000")); // Default hash when None
        assert!(display.contains("No Hash Transfer"));
    }

    #[test]
    fn test_transfer_get_top_level() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 1,
            hash: Some("test".to_string()),
            name: Some("Test".to_string()),
            size: Some(1000),
            downloaded: Some(1000),
            finished_at: Some("2024-01-01T00:00:00".to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(1),
            userfile_exists: true,
        };

        let mut transfer = Transfer::from(app_data, &putio_transfer);
        transfer.targets = Some(vec![
            DownloadTarget {
                from: None,
                to: "/downloads/folder".to_string(),
                target_type: TargetType::Directory,
                top_level: true,
                transfer_hash: "test".to_string(),
            },
            DownloadTarget {
                from: Some("https://example.com/file.mp4".to_string()),
                to: "/downloads/folder/file.mp4".to_string(),
                target_type: TargetType::File,
                top_level: false,
                transfer_hash: "test".to_string(),
            },
        ]);

        let top_level = transfer.get_top_level();
        assert!(top_level.top_level);
        assert_eq!(top_level.target_type, TargetType::Directory);
    }

    #[test]
    fn test_transfer_clone() {
        let app_data = create_test_app_data();
        let putio_transfer = PutIOTransfer {
            id: 1,
            hash: Some("test".to_string()),
            name: Some("Test".to_string()),
            size: Some(1000),
            downloaded: Some(1000),
            finished_at: Some("2024-01-01T00:00:00".to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(1),
            userfile_exists: true,
        };

        let transfer = Transfer::from(app_data, &putio_transfer);
        let cloned = transfer.clone();

        assert_eq!(transfer.transfer_id, cloned.transfer_id);
        assert_eq!(transfer.name, cloned.name);
        assert_eq!(transfer.file_id, cloned.file_id);
        assert_eq!(transfer.hash, cloned.hash);
    }

    #[test]
    fn test_download_target_hash_truncation_in_display() {
        let long_hash = "a".repeat(100);
        let target = DownloadTarget {
            from: None,
            to: "/test".to_string(),
            target_type: TargetType::Directory,
            top_level: true,
            transfer_hash: long_hash.clone(),
        };

        let display = format!("{}", target);
        // Should only show first 4 characters
        assert!(display.contains("aaaa"));
        assert!(!display.contains(&"a".repeat(50)));
    }

    #[test]
    fn test_target_type_clone() {
        let file_type = TargetType::File;
        let cloned = file_type.clone();
        assert_eq!(file_type, cloned);

        let dir_type = TargetType::Directory;
        let cloned_dir = dir_type.clone();
        assert_eq!(dir_type, cloned_dir);
    }
}
