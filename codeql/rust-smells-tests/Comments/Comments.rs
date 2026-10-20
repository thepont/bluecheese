pub struct CommentFixture {
    // brief note
    // An explanatory comment contains more than five useful words.
    pub value: i32,
}

pub fn unfinished() {
    // TODO: revisit this after the next release.
}

/// Documentation comments stay excluded despite their length and detail.
pub fn documented() {}
