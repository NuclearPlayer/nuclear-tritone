mod repository;
mod routes;

pub use repository::MappingRepository;
pub use routes::routes;

#[derive(serde::Deserialize)]
pub struct Mapping {
    pub artist: String,
    pub title: String,
    pub source: String,
    pub stream_id: String,
    pub author_id: String,
}

impl Mapping {
    pub fn new(
        artist: impl Into<String>,
        title: impl Into<String>,
        source: impl Into<String>,
        stream_id: impl Into<String>,
        author_id: impl Into<String>,
    ) -> Self {
        Self {
            artist: artist.into(),
            title: title.into(),
            source: source.into(),
            stream_id: stream_id.into(),
            author_id: author_id.into(),
        }
    }
}
