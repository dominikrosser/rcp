use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Yield {
    /* The amount, relevant to the unit. */
    pub amount: f64,

    /* Generally “servings”, but up to the user. Can be “packages”, “cups”, “glasses”, etc. */
    pub unit: String,
}

impl Yield {
    fn new() -> Self {
        Self {
            unit: "servings".to_string(),
            ..Default::default()
        }
    }
}
