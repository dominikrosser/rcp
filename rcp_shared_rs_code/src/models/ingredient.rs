use serde::{Deserialize, Serialize};

// A dict of items, describing an ingredient, and how much of that ingredient to use.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Ingredient {
    // Wrapper enclosing the relevant data
    pub ingredient: IngredientData,

    /* This field is a list of ingredients, in exactly the same format as a regular ingredient list item, minus the substitutions field.
     * For instance, it must contain amounts, and may also contain processing, usda_num, notes, etc. */
    pub substitutions: Vec<IngredientData>,
}

impl Ingredient {
    pub fn new() -> Self {
        Self {
            ingredient: IngredientData::new(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Amount {
    pub amount: f64,
    pub unit: String,
}

impl Amount {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngredientData {
    /* A list of dicts which describe the amounts to use. Normally, the list will only contain one dict.
     * In cases where multiple yields need to be stored (i.e. 50 cookies vs 100 cookes vs 250 cookies),
     * each yield will have its own dict in this list, in the same order as the recipe’s yield field. */
    pub amounts: Vec<Amount>,

    /* A list of tags which describe the processing of this item. For instance, “whole”, “large dice”, “minced”, “raw”, “steamed”, etc. */
    pub processing: Vec<String>,

    /* Any notes specific to this ingredient. */
    pub notes: Option<String>,

    /* This corresponds with the index keys in the USDA Standard Reference. It is generally used for easy lookup of nutritional data.
     * If possible, this should be used, and USDA data, when available, is preferable to any other nutritional data source. */
    // We removed usda_num in favor of an ingredient name
    pub ingredient_name: String,
}

impl IngredientData {
    pub fn new() -> Self {
        Self {
            amounts: vec![Amount::new()],
            ..Default::default()
        }
    }

    pub fn get_amount_str(&self, idx: usize) -> String {
        if idx < self.amounts.len() {
            let mut amount_str = self.amounts[idx].amount.to_string();
            if !self.amounts[idx].unit.is_empty() {
                amount_str.push(' ');
            }
            amount_str.push_str(&self.amounts[idx].unit);
            amount_str
        } else {
            "".to_string()
        }
    }
}
