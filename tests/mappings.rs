use axum_test::TestServer;
use serde_json::json;

use nuclear_tritone::mappings::Mapping;
use nuclear_tritone::test_app;

fn server() -> TestServer {
    TestServer::new(test_app().build())
}

fn server_with(mappings: Vec<Mapping>) -> TestServer {
    TestServer::new(test_app().with_mappings(mappings).build())
}

#[tokio::test]
async fn put_mapping_with_valid_body_returns_201() {
    let server = server();

    server
        .put("/mappings")
        .json(&json!({
            "author_id": "user-1",
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube",
            "stream_id": "abc123"
        }))
        .await
        .assert_status(axum::http::StatusCode::CREATED);
}

#[tokio::test]
async fn put_mapping_with_missing_fields_returns_400() {
    let server = server();

    server
        .put("/mappings")
        .json(&json!({
            "artist": "Black Sabbath"
        }))
        .await
        .assert_status(axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn top_returns_404_when_no_mappings_exist() {
    let server = server();

    server
        .post("/mappings/top")
        .json(&json!({
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        }))
        .await
        .assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn top_returns_highest_scored_stream() {
    let server = server_with(vec![
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "popular", "user-1"),
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "popular", "user-2"),
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "unpopular", "user-3"),
    ]);

    server
        .post("/mappings/top")
        .json(&json!({
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        }))
        .await
        .assert_status_ok()
        .assert_json(&json!({
            "stream_id": "popular",
            "score": 2
        }));
}

#[tokio::test]
async fn top_returns_authors_own_mapping_even_when_another_has_higher_score() {
    let server = server_with(vec![
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "popular", "user-1"),
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "popular", "user-2"),
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "popular", "user-3"),
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "my-pick", "me"),
    ]);

    server
        .post("/mappings/top")
        .json(&json!({
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube",
            "author_id": "me"
        }))
        .await
        .assert_status_ok()
        .assert_json(&json!({
            "stream_id": "my-pick",
            "score": 1,
            "self_verified": true
        }));
}

#[tokio::test]
async fn delete_mapping_removes_it_from_top_results() {
    let server = server_with(vec![
        Mapping::new("Black Sabbath", "War Pigs", "youtube", "only-stream", "user-1"),
    ]);

    server
        .delete("/mappings")
        .json(&json!({
            "author_id": "user-1",
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        }))
        .await
        .assert_status_ok();

    server
        .post("/mappings/top")
        .json(&json!({
            "artist": "Black Sabbath",
            "title": "War Pigs",
            "source": "youtube"
        }))
        .await
        .assert_status(axum::http::StatusCode::NOT_FOUND);
}
