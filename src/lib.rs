// use near_sdk::collections::HashMap;
use near_sdk::{env, near, near_bindgen, AccountId};
use std::collections::HashMap;

#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct House {
    // Add relevant house details here
    // Example:
    pub address: String,
    pub rent_per_annum: u32, // Rent amount in NEAR tokens
    pub price: u32,
}

#[near(contract_state)]
pub struct Contract {
    pub landlords: HashMap<AccountId, Option<Vec<House>>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            landlords: HashMap::new(),
        }
    }
}
// {
//     "address": "Somewhere in Nigeria",
//     "rent_per_annum": "350",
//     "price": "105200"
// }

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            landlords: HashMap::new(),
        }
    }

    // Function for landlords to register their houses
    pub fn register_house(&mut self, house: House) -> Option<House> {
        let account_id = env::predecessor_account_id();
        let landlord_houses = self.landlords.get(&account_id);

        let mut houses: Vec<House> = match landlord_houses {
            Some(houses) => houses.clone().unwrap_or(Vec::new()), // Clone the inner Vec if it exists
            None => Vec::new(),
        };

        houses.push(house.clone());
        self.landlords.insert(account_id, Some(houses.to_vec()));
        Some(House {
            address: house.address,
            rent_per_annum: house.rent_per_annum,
            price: house.price,
        }) // Return the added house for confirmation
    }

    // Function to retrieve a landlord's houses
    pub fn get_landlord_houses(&self, account_id: AccountId) -> Option<Vec<House>> {
        self.landlords.get(&account_id).cloned()?
    }
}

// deployed code
// https://explorer.testnet.near.org/transactions/DgDUccaNVuiaXaN2uDV7ZDVd4BbV6m2R4Fju6KAc1hjG
