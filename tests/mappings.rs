use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use nuclear_tritone::test_app;

#[tokio::test]
async fn put_mapping_with_valid_body_returns_201() {
    let app = test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/mappings")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&serde_json::json!({
                    "author_id": "user-1",
                    "artist": "Black Sabbath",
                    "title": "War Pigs",
                    "source": "youtube",
                    "stream_id": "abc123"
                })).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
