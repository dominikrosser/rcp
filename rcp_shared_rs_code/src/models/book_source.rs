use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BookSource {
    /* This is a list. Refers to the author(s) of this recipe. Can be the same as source_authors, if appropriate.
     * If there was only one author, then they would be the only item in the list. */
    pub authors: Vec<String>,

    /* Title of the book. This is a single value, not a list. */
    pub title: String,

    /* International Standard Book Number, if available. */
    pub isbn: Option<String>,

    /* Any information about the book that does not fit into another field. */
    pub notes: Option<String>,
}
