extern crate musicbrainz_rs;

use musicbrainz_rs::entity::disc::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let disc = Disc::fetch()
        .id("I5l9cCSFccLKFEKS.7wqSZAorPU-")
        .execute()
        .unwrap();

    println!("{:?}", disc);
}
