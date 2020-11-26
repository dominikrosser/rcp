use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HACCPValue {
    /* Refers to specific HACCP guidelines relevant to this step. */
    control_point: String,

    /* Refers to specific HACCP guidelines relevant to this step, which are critical to the safety outcome of this recipe.
     * For instance, “Cook until the food reaches an internal temperature of 165F.” */
    critical_control_point: String,
}
