use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum MobQuality {
    Normal,
    Boss,
}
