extern crate musicbrainz_rs;
use musicbrainz_rs::model::release;
use musicbrainz_rs::model::release::Media;
use musicbrainz_rs::model::release::Release;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_release_release_groups() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::ReleaseGroup)
        .execute()
        .unwrap();

    assert_eq!(justice_cross.release_group.unwrap().title, "✝");
}

#[test]
fn should_get_release_media() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Recordings)
        .execute()
        .unwrap();

    assert!(justice_cross
        .media
        .unwrap()
        .iter()
        .any(|media| media.format.as_ref().unwrap() == "CD"));
}

#[test]
fn should_get_release_recordings() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Recordings)
        .execute()
        .unwrap();

    let medias: Vec<Media> = justice_cross.media.unwrap();
    let cd = medias.iter().next().unwrap();

    assert!(cd
        .tracks
        .as_ref()
        .unwrap()
        .iter()
        .any(|track| track.title == "D.A.N.C.E."));
}

#[test]
fn should_get_release_label() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Labels)
        .execute()
        .unwrap();

    assert!(justice_cross
        .label_info
        .unwrap()
        .iter()
        .any(|label_info| label_info.label.name == "Ed Banger Records"));
}

#[test]
fn should_get_release_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Tags)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "hip hop"));
}

#[test]
fn should_get_release_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Aliases)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.aliases.is_some());
}

#[test]
fn should_get_release_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Genres)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.genres.is_some());
}

#[test]
fn should_get_release_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let pieds_nus_sur_la_braise = Release::fetch()
        .id("bdb24cb5-404b-4f60-bba4-7b730325ae47")
        .include(release::Include::Annotation)
        .execute()
        .unwrap();

    assert!(pieds_nus_sur_la_braise.annotation.is_some());
}
