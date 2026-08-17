use std::collections::HashMap;

use super::Mapping;

#[derive(serde::Serialize)]
pub struct TopStream {
    pub stream_id: String,
    pub score: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_verified: Option<bool>,
}

pub fn top_stream(mappings: &[Mapping], author_id: Option<&str>) -> Option<TopStream> {
    let authors_own = author_id.and_then(|author| {
        mappings.iter().find(|mapping| mapping.author_id == author)
    });

    if let Some(mapping) = authors_own {
        return Some(TopStream {
            stream_id: mapping.stream_id.clone(),
            score: score_of(mappings, &mapping.stream_id),
            self_verified: Some(true),
        });
    }

    scores(mappings)
        .into_iter()
        .max_by_key(|(_, score)| *score)
        .map(|(stream_id, score)| TopStream {
            stream_id: stream_id.to_owned(),
            score,
            self_verified: None,
        })
}

fn score_of(mappings: &[Mapping], stream_id: &str) -> usize {
    mappings
        .iter()
        .filter(|mapping| mapping.stream_id == stream_id)
        .count()
}

fn scores(mappings: &[Mapping]) -> HashMap<&str, usize> {
    mappings.iter().fold(HashMap::new(), |mut scores, mapping| {
        *scores.entry(mapping.stream_id.as_str()).or_insert(0) += 1;
        scores
    })
}
