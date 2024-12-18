use std::collections::HashMap;

pub fn destructure_compound(compound: &str) {

    let (compound_amount, compound): (u32, &str) = get_compound_amount(compound);

    let mut result: HashMap<String, u32> = HashMap::new();
    let mut atom: String = String::from("");
    let mut amount: u32 = 0;

    for char in compound.chars() {
        if char.is_uppercase() {
            if !atom.is_empty() {
                result.insert(atom.to_string(), if amount == 0 { 1 } else { amount });
                amount = 0;
            }
            atom = String::from(char);

        } else if char.is_lowercase() {
            atom.push(char);

        } else if char.is_numeric() {
            if amount == 0 {
                amount = char.to_string().parse().unwrap();
            } else {
                amount = add_number_on_first_index_of_number(amount, char);
            }
        }
    }

    result.insert(atom.to_string(), if amount == 0 { 1 } else { amount });
    println!("Comp-Qqt: {}", compound_amount);
    println!("{:#?}", result)
}


// 0 if no number is in front
// x if a number is in front
fn get_compound_amount(mut compound: &str) -> (u32, &str) {

    let chars = compound.chars();
    let mut string_amount: String = String::new();

    for char in chars {
        if !char.is_numeric() {
            break;
        } else {
            string_amount.push(char);
        }
    }

    let final_amount: u32 = string_amount.parse().unwrap_or_default();
    if final_amount != 0 {
        compound = &compound[final_amount.to_string().len()..compound.to_string().len()];
    }

    (final_amount, compound)
}


fn add_number_on_first_index_of_number(base: u32, to_add: char) -> u32 {
    let mut result: String = String::from(to_add);
    result.push_str(&base.to_string());
    result.parse().unwrap()
}