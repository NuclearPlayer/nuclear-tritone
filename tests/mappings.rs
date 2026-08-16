use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt;

use nuclear_tritone::test_app;

fn json_request(method: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}

#[tokio::test]
async fn put_mapping_with_valid_body_returns_201() {
    let app = test_app();

    let response = app
        .oneshot(json_request("PUT", "/mappings", json!({
            "author_id": "user-1",
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube",
            "stream_id": "abc123"
        })))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn put_mapping_with_missing_fields_returns_400() {
    let app = test_app();

    let response = app
        .oneshot(json_request("PUT", "/mappings", json!({
            "artist": "Black Sabbath"
        })))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn top_returns_404_when_no_mappings_exist() {
    let app = test_app();

    let response = app
        .oneshot(json_request("POST", "/mappings/top", json!({
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        })))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_mapping_returns_200() {
    let app = test_app();

    let response = app
        .oneshot(json_request("DELETE", "/mappings", json!({
            "author_id": "user-1",
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        })))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
