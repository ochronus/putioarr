#[cfg(test)]
mod tests {
    use super::super::routes::*;
    use crate::{
        services::transmission::{TransmissionRequest, TransmissionResponse},
        AppData, ArrConfig, Config, PutioConfig,
    };
    use actix_web::{
        http::header::{HeaderValue, AUTHORIZATION},
        test, web, App,
    };
    use base64::Engine;

    fn create_test_config() -> Config {
        Config {
            bind_address: "127.0.0.1".to_string(),
            download_directory: "/tmp/downloads".to_string(),
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
                api_key: "test_api_key".to_string(),
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

    fn create_basic_auth_header(username: &str, password: &str) -> HeaderValue {
        let credentials = format!("{}:{}", username, password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap()
    }

    #[actix_web::test]
    async fn test_rpc_get_with_valid_auth() {
        let app_data = create_test_app_data();
        let app = test::init_service(App::new().app_data(app_data).service(rpc_get)).await;

        let req = test::TestRequest::get()
            .uri("/transmission/rpc")
            .insert_header((
                AUTHORIZATION,
                create_basic_auth_header("testuser", "testpass"),
            ))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 409); // Conflict status with session ID
    }

    #[actix_web::test]
    async fn test_rpc_get_with_invalid_auth() {
        let app_data = create_test_app_data();
        let app = test::init_service(App::new().app_data(app_data).service(rpc_get)).await;

        let req = test::TestRequest::get()
            .uri("/transmission/rpc")
            .insert_header((
                AUTHORIZATION,
                create_basic_auth_header("wronguser", "wrongpass"),
            ))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403); // Forbidden
    }

    #[actix_web::test]
    async fn test_rpc_get_without_auth() {
        let app_data = create_test_app_data();
        let app = test::init_service(App::new().app_data(app_data).service(rpc_get)).await;

        let req = test::TestRequest::get()
            .uri("/transmission/rpc")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_rpc_post_session_get() {
        let app_data = create_test_app_data();
        let app = test::init_service(App::new().app_data(app_data).service(rpc_post)).await;

        let request_body = TransmissionRequest {
            method: "session-get".to_string(),
            arguments: None,
        };

        let req = test::TestRequest::post()
            .uri("/transmission/rpc")
            .insert_header((
                AUTHORIZATION,
                create_basic_auth_header("testuser", "testpass"),
            ))
            .set_json(&request_body)
            .to_request();

        let resp: TransmissionResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.result, "success");
        assert!(resp.arguments.is_some());
    }

    #[actix_web::test]
    async fn test_rpc_post_invalid_auth() {
        let app_data = create_test_app_data();
        let app = test::init_service(App::new().app_data(app_data).service(rpc_post)).await;

        let request_body = TransmissionRequest {
            method: "session-get".to_string(),
            arguments: None,
        };

        let req = test::TestRequest::post()
            .uri("/transmission/rpc")
            .insert_header((
                AUTHORIZATION,
                create_basic_auth_header("wronguser", "wrongpass"),
            ))
            .set_json(&request_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 409); // Conflict without valid session
    }

    #[actix_web::test]
    async fn test_transmission_request_deserialization() {
        let json = r#"{"method":"torrent-get","arguments":null}"#;
        let request: TransmissionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.method, "torrent-get");
        assert!(request.arguments.is_none());
    }

    #[actix_web::test]
    async fn test_transmission_response_serialization() {
        let response = TransmissionResponse {
            result: "success".to_string(),
            arguments: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"result\":\"success\""));
    }

    #[actix_web::test]
    async fn test_session_id_constant() {
        assert_eq!(SESSION_ID, "useless-session-id");
    }

    #[actix_web::test]
    async fn test_create_basic_auth_header_format() {
        let header = create_basic_auth_header("user", "pass");
        let header_str = header.to_str().unwrap();
        assert!(header_str.starts_with("Basic "));
        assert!(header_str.len() > 6);
    }

    #[actix_web::test]
    async fn test_config_username_password() {
        let config = create_test_config();
        assert_eq!(config.username, "testuser");
        assert_eq!(config.password, "testpass");
    }

    #[actix_web::test]
    async fn test_app_data_structure() {
        let app_data = create_test_app_data();
        assert_eq!(app_data.config.username, "testuser");
        assert_eq!(app_data.config.password, "testpass");
        assert_eq!(app_data.config.download_directory, "/tmp/downloads");
    }
}
