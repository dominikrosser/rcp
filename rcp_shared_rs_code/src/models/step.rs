use serde::{Deserialize, Serialize};

use super::haccp_value::HACCPValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    /* The only item in the dict that is absolutely required. */
    step: String,

    /* A dict, which can contain either a control_point or a critical_control_point. Should not contain both. */
    haccp: Option<HACCPValue>,

    /* A list of notes relevant to this step. Often known as “bench notes” to professionals. */
    notes: Option<String>,
}
