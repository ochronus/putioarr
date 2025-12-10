#[cfg(test)]
mod tests {
    use super::super::putio::*;

    #[test]
    fn test_putio_transfer_is_downloadable_with_file_id() {
        let transfer = PutIOTransfer {
            id: 1,
            hash: Some("abc123".to_string()),
            name: Some("Test".to_string()),
            size: Some(1000),
            downloaded: Some(1000),
            finished_at: Some("2024-01-01T00:00:00".to_string()),
            estimated_time: Some(0),
            status: "COMPLETED".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: Some(123),
            userfile_exists: true,
        };

        assert!(transfer.is_downloadable());
    }

    #[test]
    fn test_putio_transfer_is_not_downloadable_without_file_id() {
        let transfer = PutIOTransfer {
            id: 1,
            hash: Some("abc123".to_string()),
            name: Some("Test".to_string()),
            size: Some(1000),
            downloaded: Some(500),
            finished_at: None,
            estimated_time: Some(100),
            status: "DOWNLOADING".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: None,
            file_id: None,
            userfile_exists: false,
        };

        assert!(!transfer.is_downloadable());
    }

    #[test]
    fn test_putio_account_info_serialization() {
        let account_info = PutIOAccountInfo {
            username: "testuser".to_string(),
            mail: "test@example.com".to_string(),
            account_active: true,
        };

        let json = serde_json::to_string(&account_info).unwrap();
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"mail\":\"test@example.com\""));
        assert!(json.contains("\"account_active\":true"));
    }

    #[test]
    fn test_putio_account_info_deserialization() {
        let json = r#"{
            "username": "testuser",
            "mail": "test@example.com",
            "account_active": true
        }"#;

        let account_info: PutIOAccountInfo = serde_json::from_str(json).unwrap();
        assert_eq!(account_info.username, "testuser");
        assert_eq!(account_info.mail, "test@example.com");
        assert!(account_info.account_active);
    }

    #[test]
    fn test_putio_transfer_deserialization_full() {
        let json = r#"{
            "id": 123,
            "hash": "abc123def456",
            "name": "Test Download",
            "size": 1000000,
            "downloaded": 500000,
            "finished_at": "2024-01-01T12:00:00",
            "estimated_time": 300,
            "status": "DOWNLOADING",
            "started_at": "2024-01-01T11:00:00",
            "error_message": null,
            "file_id": 456,
            "userfile_exists": true
        }"#;

        let transfer: PutIOTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.id, 123);
        assert_eq!(transfer.hash, Some("abc123def456".to_string()));
        assert_eq!(transfer.name, Some("Test Download".to_string()));
        assert_eq!(transfer.size, Some(1000000));
        assert_eq!(transfer.downloaded, Some(500000));
        assert_eq!(transfer.estimated_time, Some(300));
        assert_eq!(transfer.status, "DOWNLOADING");
        assert_eq!(transfer.file_id, Some(456));
        assert!(transfer.userfile_exists);
    }

    #[test]
    fn test_putio_transfer_deserialization_minimal() {
        let json = r#"{
            "id": 789,
            "hash": null,
            "name": null,
            "size": null,
            "downloaded": null,
            "finished_at": null,
            "estimated_time": null,
            "status": "QUEUED",
            "started_at": null,
            "error_message": null,
            "file_id": null,
            "userfile_exists": false
        }"#;

        let transfer: PutIOTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.id, 789);
        assert!(transfer.hash.is_none());
        assert!(transfer.name.is_none());
        assert!(transfer.size.is_none());
        assert!(transfer.downloaded.is_none());
        assert!(transfer.file_id.is_none());
        assert!(!transfer.userfile_exists);
    }

    #[test]
    fn test_list_transfer_response_deserialization() {
        let json = r#"{
            "transfers": [
                {
                    "id": 1,
                    "hash": "hash1",
                    "name": "Transfer 1",
                    "size": 1000,
                    "downloaded": 1000,
                    "finished_at": "2024-01-01T00:00:00",
                    "estimated_time": 0,
                    "status": "COMPLETED",
                    "started_at": "2024-01-01T00:00:00",
                    "error_message": null,
                    "file_id": 1,
                    "userfile_exists": true
                },
                {
                    "id": 2,
                    "hash": "hash2",
                    "name": "Transfer 2",
                    "size": 2000,
                    "downloaded": 1000,
                    "finished_at": null,
                    "estimated_time": 100,
                    "status": "DOWNLOADING",
                    "started_at": "2024-01-01T00:00:00",
                    "error_message": null,
                    "file_id": null,
                    "userfile_exists": false
                }
            ]
        }"#;

        let response: ListTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transfers.len(), 2);
        assert_eq!(response.transfers[0].id, 1);
        assert_eq!(response.transfers[1].id, 2);
        assert!(response.transfers[0].is_downloadable());
        assert!(!response.transfers[1].is_downloadable());
    }

    #[test]
    fn test_get_transfer_response_deserialization() {
        let json = r#"{
            "transfer": {
                "id": 123,
                "hash": "abc123",
                "name": "Single Transfer",
                "size": 5000,
                "downloaded": 5000,
                "finished_at": "2024-01-01T00:00:00",
                "estimated_time": 0,
                "status": "COMPLETED",
                "started_at": "2024-01-01T00:00:00",
                "error_message": null,
                "file_id": 999,
                "userfile_exists": true
            }
        }"#;

        let response: GetTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transfer.id, 123);
        assert_eq!(response.transfer.name, Some("Single Transfer".to_string()));
    }

    #[test]
    fn test_file_response_deserialization() {
        let json = r#"{
            "content_type": "video/mp4",
            "id": 123,
            "name": "test.mp4",
            "file_type": "VIDEO"
        }"#;

        let file: FileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(file.content_type, "video/mp4");
        assert_eq!(file.id, 123);
        assert_eq!(file.name, "test.mp4");
        assert_eq!(file.file_type, "VIDEO");
    }

    #[test]
    fn test_list_file_response_deserialization() {
        let json = r#"{
            "files": [
                {
                    "content_type": "video/mp4",
                    "id": 1,
                    "name": "video.mp4",
                    "file_type": "VIDEO"
                },
                {
                    "content_type": "application/x-directory",
                    "id": 2,
                    "name": "subfolder",
                    "file_type": "FOLDER"
                }
            ],
            "parent": {
                "content_type": "application/x-directory",
                "id": 0,
                "name": "root",
                "file_type": "FOLDER"
            }
        }"#;

        let response: ListFileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.files.len(), 2);
        assert_eq!(response.files[0].file_type, "VIDEO");
        assert_eq!(response.files[1].file_type, "FOLDER");
        assert_eq!(response.parent.name, "root");
    }

    #[test]
    fn test_url_response_deserialization() {
        let json = r#"{
            "url": "https://example.com/download/file.mp4"
        }"#;

        let response: URLResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.url, "https://example.com/download/file.mp4");
    }

    #[test]
    fn test_putio_transfer_with_error_message() {
        let transfer = PutIOTransfer {
            id: 1,
            hash: Some("error_hash".to_string()),
            name: Some("Failed Transfer".to_string()),
            size: Some(1000),
            downloaded: Some(100),
            finished_at: None,
            estimated_time: None,
            status: "ERROR".to_string(),
            started_at: Some("2024-01-01T00:00:00".to_string()),
            error_message: Some("Network timeout".to_string()),
            file_id: None,
            userfile_exists: false,
        };

        assert!(!transfer.is_downloadable());
        assert_eq!(transfer.status, "ERROR");
        assert_eq!(transfer.error_message, Some("Network timeout".to_string()));
    }

    #[test]
    fn test_putio_transfer_various_statuses() {
        let statuses = vec![
            "WAITING",
            "DOWNLOADING",
            "COMPLETED",
            "SEEDING",
            "ERROR",
            "QUEUED",
            "IN_QUEUE",
        ];

        for status in statuses {
            let transfer = PutIOTransfer {
                id: 1,
                hash: Some("test".to_string()),
                name: Some("Test".to_string()),
                size: Some(1000),
                downloaded: Some(0),
                finished_at: None,
                estimated_time: None,
                status: status.to_string(),
                started_at: None,
                error_message: None,
                file_id: None,
                userfile_exists: false,
            };

            assert_eq!(transfer.status, status);
        }
    }

    #[test]
    fn test_account_info_response_deserialization() {
        let json = r#"{
            "info": {
                "username": "testuser",
                "mail": "test@example.com",
                "account_active": true
            }
        }"#;

        let response: PutIOAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.info.username, "testuser");
        assert_eq!(response.info.mail, "test@example.com");
        assert!(response.info.account_active);
    }

    #[test]
    fn test_account_info_inactive_account() {
        let json = r#"{
            "info": {
                "username": "inactive",
                "mail": "inactive@example.com",
                "account_active": false
            }
        }"#;

        let response: PutIOAccountResponse = serde_json::from_str(json).unwrap();
        assert!(!response.info.account_active);
    }
}
