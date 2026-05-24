use crate::models::requests::{GetStreamsQuery, LoginRequest};
use crate::models::responses::{LoginResponse, StreamCatalogItem, StreamCatalogResponse};

#[test]
fn test_deserialize_login_request() {
    let json_data = r#"{"email": "test@test.com", "password": "password123"}"#;
    let req: LoginRequest = serde_json::from_str(json_data).unwrap();
    assert_eq!(req.email, "test@test.com");
    assert_eq!(req.password, "password123");
}

#[test]
fn test_deserialize_login_request_missing_field() {
    let json_data = r#"{"email": "test@test.com"}"#;
    let res: Result<LoginRequest, _> = serde_json::from_str(json_data);
    assert!(res.is_err(), "Should fail when required field is missing");
}

#[test]
fn test_deserialize_get_streams_query() {
    let json_data = r#"{"limit": 50, "cursor": "abc"}"#;
    let req: GetStreamsQuery = serde_json::from_str(json_data).unwrap();
    assert_eq!(req.limit, Some(50));
    assert_eq!(req.cursor, Some("abc".to_string()));
}

#[test]
fn test_deserialize_get_streams_query_empty() {
    let json_data = r#"{}"#;
    let req: GetStreamsQuery = serde_json::from_str(json_data).unwrap();
    assert_eq!(req.limit, None);
    assert_eq!(req.cursor, None);
}

#[test]
fn test_serialize_login_response() {
    let res = LoginResponse {
        access_token: "access123".to_string(),
        refresh_token: "refresh123".to_string(),
        expires_in: 3600,
    };
    let json_data = serde_json::to_string(&res).unwrap();
    assert!(json_data.contains(r#""access_token":"access123""#));
    assert!(json_data.contains(r#""refresh_token":"refresh123""#));
    assert!(json_data.contains(r#""expires_in":3600"#));
}

#[test]
fn test_serialize_stream_catalog_response() {
    let item = StreamCatalogItem {
        stream_id: "stream-1".to_string(),
        title: "Road to Global".to_string(),
        category: "cs2".to_string(),
        viewers_count: 1500,
    };
    let res = StreamCatalogResponse {
        data: vec![item],
        next_cursor: "next_abc".to_string(),
    };
    let json_data = serde_json::to_string(&res).unwrap();
    assert!(json_data.contains(r#""stream_id":"stream-1""#));
    assert!(json_data.contains(r#""viewers_count":1500"#));
    assert!(json_data.contains(r#""next_cursor":"next_abc""#));
}
