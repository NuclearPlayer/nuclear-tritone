mod repository;
mod routes;

pub use repository::MappingRepository;
pub use routes::routes;

pub struct Mapping {
    pub artist: String,
    pub title: String,
    pub source: String,
    pub stream_id: String,
    pub author_id: String,
}
