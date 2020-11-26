use serde::{Deserialize, Serialize};

// A dict of items, describing an ingredient, and how much of that ingredient to use.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    data: IngredientData,

    /* This field is a list of ingredients, in exactly the same format as a regular ingredient list item, minus the substitutions field.
     * For instance, it must contain amounts, and may also contain processing, usda_num, notes, etc. */
    substitutions: Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    amount: f32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngredientData {
    /* A list of dicts which describe the amounts to use. Normally, the list will only contain one dict.
     * In cases where multiple yields need to be stored (i.e. 50 cookies vs 100 cookes vs 250 cookies),
     * each yield will have its own dict in this list, in the same order as the recipe’s yield field. */
    amounts: Vec<Amount>,

    /* A list of tags which describe the processing of this item. For instance, “whole”, “large dice”, “minced”, “raw”, “steamed”, etc. */
    processing: Vec<String>,

    /* Any notes specific to this ingredient. */
    notes: String,

    /* This corresponds with the index keys in the USDA Standard Reference. It is generally used for easy lookup of nutritional data.
     * If possible, this should be used, and USDA data, when available, is preferable to any other nutritional data source. */
    usda_num: Option<String>,
}