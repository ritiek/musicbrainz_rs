extern crate musicbrainz_rs;

use lucene_query_builder::QueryBuilder;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Search;
use std::{thread, time};

#[test]
fn should_search_artist() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let query = Artist::query_builder()
        .name("Nirvana")
        .and()
        .artist_type("Group")
        .build();

    let result = Artist::search(query).execute().unwrap();

    assert!(!result.entities.is_empty());
}
