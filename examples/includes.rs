extern crate musicbrainz_rs;

use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .with_releases()
        .with_works()
        .with_artist_relations()
        .with_event_relations()
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "parent"));
    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "main performer"));
}
