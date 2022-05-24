use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::TreeMap,
    env,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, Promise,
};

#[derive(PanicOnDefault, BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct App {
    id: u64,
    title: String,
    genre: String,
    price: u128,
    published_on: u64,
    developer: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
pub struct AppStore {
    apps: TreeMap<u64, App>,
    buyers: TreeMap<u64, Vec<AccountId>>,
}

#[near_bindgen]
impl AppStore {
    #[init]
    pub fn new() -> Self {
        Self {
            apps: TreeMap::new(b"t"),
            buyers: TreeMap::new(b"b"),
        }
    }

    pub fn publish_app(&mut self, title: String, genre: String, yocto_price: U128) -> bool {
        let price = yocto_price.0;

        if env::current_account_id() == env::signer_account_id() {
            env::panic_str("Store itself cannot publish application");
        }

        if title.is_empty() {
            env::panic_str("title not provided");
        }

        if genre.is_empty() {
            env::panic_str("genre not provided");
        }

        if price <= 0 {
            env::panic_str("price must be greater than 0");
        }

        if !genre.eq_ignore_ascii_case("games") && !genre.eq_ignore_ascii_case("entertainment") {
            env::panic_str("Genre must be 'games' or 'entertainment'.");
        }

        let id = self.apps.len() + 1;

        self.apps
            .insert(
                &id,
                &App {
                    title,
                    genre,
                    price,
                    id,
                    published_on: env::block_timestamp_ms(),
                    developer: env::signer_account_id(),
                },
            )
            .is_none()
    }

    pub fn list_apps(&self) -> Vec<App> {
        let res: Vec<App> = self.apps.iter().map(|(_, app)| app).collect();
        res
    }

    /* When buying app:
        1. Transaction is signed by buyer
        2. Store gets 50% of app price
        3. Remaining 50% goes to app developer
    */
    #[payable]
    pub fn buy_app(&mut self, id: u64) -> Promise {
        if env::current_account_id() == env::signer_account_id() {
            env::panic_str("appstore itself cannot buy apps");
        }

        if id <= 0 {
            env::panic_str("invalid app id provided");
        }

        if !self.apps.contains_key(&id) {
            env::panic_str(format!("app with id {} not found", id).as_str());
        }

        let app = self.apps.get(&id).unwrap();

        if self.is_app_bought(&app.id, &env::signer_account_id()) {
            env::panic_str("Specified buyer has already bought this app before")
        }

        let deposit = env::attached_deposit();
        if deposit < app.price {
            env::panic_str(
                format!(
                    "provided deposit '{}' is less than app price '{}'",
                    deposit, app.price
                )
                .as_str(),
            );
        }

        let mut amount_to_return = 0;

        if deposit > app.price {
            amount_to_return = deposit - app.price;
        }

        let store_fee = app.price / 2;
        let developer_fee = app.price - store_fee;

        let mut p1 = Promise::new(AccountId::new_unchecked(app.developer.to_string()))
            .transfer(developer_fee);

        if amount_to_return > 0 {
            let p2 = Promise::new(env::signer_account_id()).transfer(amount_to_return);
            p1 = p1.then(p2);
        }

        if let Some(mut buyer) = self.buyers.get(&app.id) {
            self.buyers.remove(&app.id);
            buyer.push(env::signer_account_id());
            self.buyers.insert(&app.id, &buyer);
        } else {
            self.buyers.insert(&app.id, &vec![env::signer_account_id()]);
        }
        p1
    }

    fn get_app_buyers(&self, id: &u64) -> Vec<AccountId> {
        match self.buyers.get(id) {
            Some(buyers) => buyers,
            None => vec![],
        }
    }

    fn is_app_bought(&self, app_id: &u64, buyer: &AccountId) -> bool {
        let buyers = self.get_app_buyers(app_id);
        buyers.contains(buyer)
    }

    pub fn list_buyer_apps(&self, buyer: AccountId) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        self.buyers.into_iter().for_each(|(id, buyers)| {
            if buyers.contains(&buyer) {
                result.push(self.apps.get(&id).unwrap().title)
            }
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    use super::*;

    fn get_developer_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.account_balance(5 * near_sdk::ONE_NEAR);
        builder
    }

    fn get_buyer_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id("buyer.near".parse().unwrap());
        builder.attached_deposit(2 * near_sdk::ONE_NEAR);
        builder
    }

    #[test]
    fn publish_app() {
        let context = get_developer_context();
        testing_env!(context.build());

        let mut store = AppStore::new();
        assert_eq!(store.list_apps().len(), 0);
        let result = store.publish_app(
            "Red Dead Redemption".to_string(),
            "games".to_string(),
            U128(8 * near_sdk::ONE_NEAR),
        );
        assert!(result);
        assert_eq!(1, store.list_apps().len());
    }

    #[test]
    fn buy_app() {
        let context = get_buyer_context();
        testing_env!(context.build());

        let mut store = AppStore::new();
        store.publish_app(
            "Red Dead Redemption".to_string(),
            "games".to_string(),
            U128(3 * near_sdk::ONE_NEAR),
        );

        store.publish_app(
            "GTA V".to_string(),
            "games".to_string(),
            U128(2 * near_sdk::ONE_NEAR),
        );

        store.buy_app(2);
        let account_id = store.buyers.get(&2).unwrap();
        assert_eq!(account_id[0].as_str(), "buyer.near".to_string());
    }
}
