#[cfg(test)]
mod tests {
    use super::super::putio::PutIOTransfer;
    use super::super::transmission::*;
    use chrono::Utc;

    #[test]
    fn test_transmission_config_default() {
        let config = TransmissionConfig::default();
        assert_eq!(config.rpc_version, "18");
        assert_eq!(config.version, "14.0.0");
        assert_eq!(config.download_dir, "/");
        assert_eq!(config.seed_ratio_limit, 1.0);
        assert!(config.seed_ratio_limited);
        assert_eq!(config.idle_seeding_limit, 100);
        assert!(!config.idle_seeding_limit_enabled);
    }

    #[test]
    fn test_transmission_torrent_from_putio_transfer_completed() {
        let putio_transfer = PutIOTransfer {
            id: 123,
            hash: Some("abc123def456".to_string()),
            name: Some("Test Download".to_string()),
            size: Some(1000000),
            downloaded: Some(1000000),
            finished_at: Some(Utc::now().format("%FT%T").to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some(Utc::now().format("%FT%T").to_string()),
            error_message: None,
            file_id: Some(456),
            userfile_exists: true,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(transmission_torrent.id, 123);
        assert_eq!(
            transmission_torrent.hash_string,
            Some("abc123def456".to_string())
        );
        assert_eq!(transmission_torrent.name, "Test Download");
        assert_eq!(transmission_torrent.total_size, 1000000);
        assert_eq!(transmission_torrent.left_until_done, 0);
        assert!(transmission_torrent.is_finished);
        assert_eq!(transmission_torrent.eta, 0);
        assert_eq!(transmission_torrent.downloaded_ever, 1000000);
        assert!(transmission_torrent.error_string.is_none());
        assert_eq!(transmission_torrent.file_count, 1);
    }

    #[test]
    fn test_transmission_torrent_from_putio_transfer_downloading() {
        let putio_transfer = PutIOTransfer {
            id: 789,
            hash: Some("xyz789".to_string()),
            name: Some("Downloading Item".to_string()),
            size: Some(5000000),
            downloaded: Some(2500000),
            finished_at: None,
            estimated_time: Some(300),
            status: "DOWNLOADING".to_string(),
            started_at: Some(Utc::now().format("%FT%T").to_string()),
            error_message: None,
            file_id: Some(999),
            userfile_exists: false,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(transmission_torrent.id, 789);
        assert_eq!(transmission_torrent.total_size, 5000000);
        assert_eq!(transmission_torrent.left_until_done, 2500000);
        assert!(!transmission_torrent.is_finished);
        assert_eq!(transmission_torrent.eta, 300);
        assert!(matches!(
            transmission_torrent.status,
            TransmissionTorrentStatus::Downloading
        ));
    }

    #[test]
    fn test_transmission_torrent_from_putio_transfer_with_error() {
        let putio_transfer = PutIOTransfer {
            id: 111,
            hash: Some("error123".to_string()),
            name: Some("Failed Download".to_string()),
            size: Some(1000),
            downloaded: Some(500),
            finished_at: None,
            estimated_time: None,
            status: "ERROR".to_string(),
            started_at: Some(Utc::now().format("%FT%T").to_string()),
            error_message: Some("Network error".to_string()),
            file_id: None,
            userfile_exists: false,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(
            transmission_torrent.error_string,
            Some("Network error".to_string())
        );
        assert!(matches!(
            transmission_torrent.status,
            TransmissionTorrentStatus::Stopped
        ));
    }

    #[test]
    fn test_transmission_torrent_from_putio_transfer_unknown_name() {
        let putio_transfer = PutIOTransfer {
            id: 222,
            hash: None,
            name: None,
            size: Some(1000),
            downloaded: Some(0),
            finished_at: None,
            estimated_time: None,
            status: "QUEUED".to_string(),
            started_at: None,
            error_message: None,
            file_id: None,
            userfile_exists: false,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(transmission_torrent.name, "Unknown");
        assert_eq!(transmission_torrent.hash_string, None);
    }

    #[test]
    fn test_transmission_torrent_left_until_done_negative_protection() {
        // Test that left_until_done never goes negative
        let putio_transfer = PutIOTransfer {
            id: 333,
            hash: Some("test".to_string()),
            name: Some("Test".to_string()),
            size: Some(1000),
            downloaded: Some(1500), // Downloaded more than size
            finished_at: None,
            estimated_time: None,
            status: "DOWNLOADING".to_string(),
            started_at: Some(Utc::now().format("%FT%T").to_string()),
            error_message: None,
            file_id: Some(1),
            userfile_exists: false,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(transmission_torrent.left_until_done, 0);
    }

    #[test]
    fn test_transmission_torrent_status_stopped() {
        assert!(matches!(
            TransmissionTorrentStatus::from("STOPPED".to_string()),
            TransmissionTorrentStatus::Stopped
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("COMPLETED".to_string()),
            TransmissionTorrentStatus::Stopped
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("ERROR".to_string()),
            TransmissionTorrentStatus::Stopped
        ));
    }

    #[test]
    fn test_transmission_torrent_status_check_wait() {
        assert!(matches!(
            TransmissionTorrentStatus::from("CHECKWAIT".to_string()),
            TransmissionTorrentStatus::CheckWait
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("PREPARING_DOWNLOAD".to_string()),
            TransmissionTorrentStatus::CheckWait
        ));
    }

    #[test]
    fn test_transmission_torrent_status_check() {
        assert!(matches!(
            TransmissionTorrentStatus::from("CHECK".to_string()),
            TransmissionTorrentStatus::Check
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("COMPLETING".to_string()),
            TransmissionTorrentStatus::Check
        ));
    }

    #[test]
    fn test_transmission_torrent_status_queued() {
        assert!(matches!(
            TransmissionTorrentStatus::from("QUEUED".to_string()),
            TransmissionTorrentStatus::Queued
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("IN_QUEUE".to_string()),
            TransmissionTorrentStatus::Queued
        ));
    }

    #[test]
    fn test_transmission_torrent_status_downloading() {
        assert!(matches!(
            TransmissionTorrentStatus::from("DOWNLOADING".to_string()),
            TransmissionTorrentStatus::Downloading
        ));
    }

    #[test]
    fn test_transmission_torrent_status_seeding_wait() {
        assert!(matches!(
            TransmissionTorrentStatus::from("SEEDINGWAIT".to_string()),
            TransmissionTorrentStatus::SeedingWait
        ));
    }

    #[test]
    fn test_transmission_torrent_status_seeding() {
        assert!(matches!(
            TransmissionTorrentStatus::from("SEEDING".to_string()),
            TransmissionTorrentStatus::Seeding
        ));
    }

    #[test]
    fn test_transmission_torrent_status_unknown() {
        // Unknown status should default to CheckWait
        assert!(matches!(
            TransmissionTorrentStatus::from("UNKNOWN_STATUS".to_string()),
            TransmissionTorrentStatus::CheckWait
        ));
    }

    #[test]
    fn test_transmission_torrent_status_case_insensitive() {
        assert!(matches!(
            TransmissionTorrentStatus::from("downloading".to_string()),
            TransmissionTorrentStatus::Downloading
        ));
        assert!(matches!(
            TransmissionTorrentStatus::from("SeEdInG".to_string()),
            TransmissionTorrentStatus::Seeding
        ));
    }

    #[test]
    fn test_transmission_response_serialization() {
        let response = TransmissionResponse {
            result: "success".to_string(),
            arguments: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"result\":\"success\""));
        assert!(json.contains("\"arguments\":null"));
    }

    #[test]
    fn test_transmission_request_deserialization() {
        let json = r#"{"method":"torrent-get","arguments":null}"#;
        let request: TransmissionRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.method, "torrent-get");
        assert!(request.arguments.is_none());
    }

    #[test]
    fn test_transmission_request_with_arguments() {
        let json = r#"{"method":"torrent-add","arguments":{"filename":"magnet:?xt=urn"}}"#;
        let request: TransmissionRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.method, "torrent-add");
        assert!(request.arguments.is_some());
    }

    #[test]
    fn test_transmission_config_serialization() {
        let config = TransmissionConfig {
            rpc_version: "18".to_string(),
            version: "14.0.0".to_string(),
            download_dir: "/downloads".to_string(),
            seed_ratio_limit: 2.0,
            seed_ratio_limited: true,
            idle_seeding_limit: 200,
            idle_seeding_limit_enabled: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"rpc-version\":\"18\""));
        assert!(json.contains("\"download-dir\":\"/downloads\""));
        assert!(json.contains("\"seedRatioLimit\":2.0"));
    }

    #[test]
    fn test_putio_transfer_none_values() {
        let putio_transfer = PutIOTransfer {
            id: 1,
            hash: None,
            name: None,
            size: None,
            downloaded: None,
            finished_at: None,
            estimated_time: None,
            status: "QUEUED".to_string(),
            started_at: None,
            error_message: None,
            file_id: None,
            userfile_exists: false,
        };

        let transmission_torrent: TransmissionTorrent = putio_transfer.into();

        assert_eq!(transmission_torrent.total_size, 0);
        assert_eq!(transmission_torrent.downloaded_ever, 0);
        assert_eq!(transmission_torrent.left_until_done, 0);
        assert_eq!(transmission_torrent.eta, 0);
    }
}
