use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Yield {
    /* The amount, relevant to the unit. */
    pub amount: f64,

    /* Generally “servings”, but up to the user. Can be “packages”, “cups”, “glasses”, etc. */
    pub unit: String,
}
