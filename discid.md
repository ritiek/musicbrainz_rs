[discid lookup](https://musicbrainz.org/doc/MusicBrainz_API#discid) is now possible with:
```rust
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
```
https://musicbrainz.org/ws/2/discid/I5l9cCSFccLKFEKS.7wqSZAorPU-?fmt=json

Not sure how should I write tests for this. The response for this endpoint call is pretty large, probably not a good idea to test it the same way as other entities in [`tests/fetch.rs`](https://github.com/oknozor/musicbrainz_rs/blob/b72c31ebc75d2386479e2d3263f7795633ae9f1d/tests/fetch.rs). 

Addresses #11.
