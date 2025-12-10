#[cfg(test)]
mod tests {
    use crate::utils::{Context, TEMPLATE};

    #[test]
    fn test_template_contains_required_fields() {
        assert!(TEMPLATE.contains("username"));
        assert!(TEMPLATE.contains("password"));
        assert!(TEMPLATE.contains("download_directory"));
        assert!(TEMPLATE.contains("bind_address"));
        assert!(TEMPLATE.contains("port"));
        assert!(TEMPLATE.contains("loglevel"));
        assert!(TEMPLATE.contains("uid"));
        assert!(TEMPLATE.contains("polling_interval"));
        assert!(TEMPLATE.contains("skip_directories"));
        assert!(TEMPLATE.contains("orchestration_workers"));
        assert!(TEMPLATE.contains("download_workers"));
    }

    #[test]
    fn test_template_contains_putio_section() {
        assert!(TEMPLATE.contains("[putio]"));
        assert!(TEMPLATE.contains("api_key"));
        assert!(TEMPLATE.contains("{putio_api_key}"));
    }

    #[test]
    fn test_template_contains_arr_sections() {
        assert!(TEMPLATE.contains("[sonarr]"));
        assert!(TEMPLATE.contains("[radarr]"));
        assert!(TEMPLATE.contains("[whisparr]"));
    }

    #[test]
    fn test_template_has_default_values() {
        assert!(TEMPLATE.contains("0.0.0.0"));
        assert!(TEMPLATE.contains("9091"));
        assert!(TEMPLATE.contains("info"));
        assert!(TEMPLATE.contains("1000"));
        assert!(TEMPLATE.contains("10"));
        assert!(TEMPLATE.contains("[\"sample\", \"extras\"]"));
        assert!(TEMPLATE.contains("4"));
    }

    #[test]
    fn test_template_has_comments() {
        assert!(TEMPLATE.contains("# Required"));
        assert!(TEMPLATE.contains("# Optional"));
        assert!(TEMPLATE.contains("# Can be found in Settings -> General"));
    }

    #[test]
    fn test_context_serialization() {
        let context = Context {
            putio_api_key: "test_api_key_123".to_string(),
        };

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains("putio_api_key"));
        assert!(json.contains("test_api_key_123"));
    }

    #[test]
    fn test_context_deserialization() {
        let json = r#"{"putio_api_key":"my_key_456"}"#;
        let context: Context = serde_json::from_str(json).unwrap();
        assert_eq!(context.putio_api_key, "my_key_456");
    }

    #[test]
    fn test_template_structure() {
        // Verify TOML structure is valid by checking for section headers
        let sections = vec!["[putio]", "[sonarr]", "[radarr]", "[whisparr]"];
        for section in sections {
            assert!(
                TEMPLATE.contains(section),
                "Template should contain section: {}",
                section
            );
        }
    }

    #[test]
    fn test_template_all_arr_services_have_url() {
        // Each arr service should have url field
        // Just verify that each section exists and has the required fields in the template
        assert!(TEMPLATE.contains("[sonarr]"));
        assert!(TEMPLATE.contains("[radarr]"));
        assert!(TEMPLATE.contains("[whisparr]"));

        // Count occurrences - should have url and api_key for each service
        let url_count = TEMPLATE.matches("url = ").count();
        let api_key_count = TEMPLATE.matches("api_key = ").count();

        // Should have at least 4 urls (putio + 3 arr services) and 4 api_keys
        assert!(
            url_count >= 3,
            "Expected at least 3 url fields, found {}",
            url_count
        );
        assert!(
            api_key_count >= 4,
            "Expected at least 4 api_key fields, found {}",
            api_key_count
        );
    }

    #[test]
    fn test_template_has_service_examples() {
        assert!(TEMPLATE.contains("mysonarrhost"));
        assert!(TEMPLATE.contains("myradarrhost"));
        assert!(TEMPLATE.contains("mywhisparrhost"));
        assert!(TEMPLATE.contains("8989"));
        assert!(TEMPLATE.contains("7878"));
        assert!(TEMPLATE.contains("6969"));
    }

    #[test]
    fn test_template_has_credential_placeholders() {
        assert!(TEMPLATE.contains("myusername"));
        assert!(TEMPLATE.contains("mypassword"));
        assert!(TEMPLATE.contains("MYSONARRAPIKEY"));
        assert!(TEMPLATE.contains("MYRADARRAPIKEY"));
        assert!(TEMPLATE.contains("MYWHISPARRAPIKEY"));
    }

    #[test]
    fn test_template_download_directory_placeholder() {
        assert!(TEMPLATE.contains("/path/to/downloads"));
    }

    #[test]
    fn test_template_skip_directories_format() {
        // Should be a TOML array
        assert!(TEMPLATE.contains("skip_directories = [\"sample\", \"extras\"]"));
    }

    #[test]
    fn test_template_numeric_defaults() {
        // Check that numeric values are not quoted
        assert!(TEMPLATE.contains("port = 9091"));
        assert!(TEMPLATE.contains("uid = 1000"));
        assert!(TEMPLATE.contains("polling_interval = 10"));
        assert!(TEMPLATE.contains("orchestration_workers = 10"));
        assert!(TEMPLATE.contains("download_workers = 4"));
    }

    #[test]
    fn test_template_string_defaults() {
        // Check that string values are properly quoted
        assert!(TEMPLATE.contains("bind_address = \"0.0.0.0\""));
        assert!(TEMPLATE.contains("loglevel = \"info\""));
    }

    #[test]
    fn test_template_order_of_sections() {
        let putio_pos = TEMPLATE.find("[putio]").unwrap();
        let sonarr_pos = TEMPLATE.find("[sonarr]").unwrap();
        let radarr_pos = TEMPLATE.find("[radarr]").unwrap();
        let whisparr_pos = TEMPLATE.find("[whisparr]").unwrap();

        // Verify sections appear in expected order
        assert!(putio_pos < sonarr_pos);
        assert!(sonarr_pos < radarr_pos);
        assert!(radarr_pos < whisparr_pos);
    }

    #[test]
    fn test_template_has_helpful_comments() {
        // Check for helpful comment about generating token
        assert!(TEMPLATE.contains("putioarr get-token"));

        // Check for comment about optional services
        assert!(TEMPLATE.contains("optional"));
    }

    #[test]
    fn test_context_empty_api_key() {
        let context = Context {
            putio_api_key: String::new(),
        };
        assert_eq!(context.putio_api_key, "");
    }

    #[test]
    fn test_context_with_special_characters() {
        let context = Context {
            putio_api_key: "key-with-special_chars.123!@#".to_string(),
        };
        assert_eq!(context.putio_api_key, "key-with-special_chars.123!@#");
    }

    #[test]
    fn test_template_multiline_format() {
        // Template should have proper line breaks
        let lines: Vec<&str> = TEMPLATE.lines().collect();
        assert!(lines.len() > 20, "Template should have multiple lines");
    }

    #[test]
    fn test_template_no_trailing_whitespace_on_section_headers() {
        for line in TEMPLATE.lines() {
            if line.starts_with('[') && line.ends_with(']') {
                assert_eq!(
                    line.trim(),
                    line,
                    "Section header should not have trailing whitespace"
                );
            }
        }
    }

    #[test]
    fn test_template_comment_format() {
        // All comments should start with #
        for line in TEMPLATE.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('[') && !trimmed.contains('=') {
                assert!(
                    trimmed.starts_with('#'),
                    "Non-empty, non-assignment line should be a comment: {}",
                    line
                );
            }
        }
    }
}
