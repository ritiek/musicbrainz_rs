use crate::entity::release::Release;
use lucene_query_builder::QueryBuilder;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, QueryBuilder, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Disc {
    pub id: String,
    pub offset_count: u32,
    pub sectors: u32,
    pub releases: Option<Vec<Release>>,
    pub offsets : Vec<u32>
}
