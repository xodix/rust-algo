use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("{:?}", generate_test_inventory(90));
}

fn update_inventory(
    curr_inventory: Vec<(u32, String)>,
    delivered_inventory: Vec<(u32, String)>,
) -> Vec<(u32, String)> {
    let mut o_n_hack: HashMap<String, u32> = HashMap::new();
    for (amount, item) in curr_inventory {
        o_n_hack.insert(item, amount);
    }
    for (amount, item) in delivered_inventory {
        if o_n_hack.contains_key(&item) {
            let curr_state = *o_n_hack.get(&item).unwrap();
            o_n_hack.insert(item, curr_state);
        } else {
            o_n_hack.insert(item, amount);
        }
    }
    let mut fin = Vec::new();
    for (item, amount) in o_n_hack {
        fin.push((amount, item))
    }
    fin
}

fn generate_test_inventory(num_of_indexes: usize) -> Vec<(u32, String)> {
    let mut test_inventory = Vec::new();
    let possible_items = vec![
        "Potatoes".to_string(),
        "Vodka".to_string(),
        "Bread".to_string(),
        "Beer".to_string(),
        "Steak".to_string(),
        "Coca-Cola".to_string(),
        "Apples".to_string(),
        "Grapes".to_string(),
        "Hazelnuts".to_string(),
        "Juice".to_string(),
    ];
    for _i in 0..num_of_indexes {
        let random_amount: u32 = rand::thread_rng().gen_range(1..=100);
        let random_item: usize = rand::thread_rng().gen_range(0..10);
        test_inventory.push((random_amount, possible_items[random_item].clone()));
    }
    test_inventory
}
