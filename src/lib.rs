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
}

#[near_bindgen]
impl AppStore {
    #[init]
    pub fn new(developer_account: AccountId) -> Self {
        Self {
            apps: AppStore::init_apps(developer_account),
        }
    }

    fn init_apps(developer_account: AccountId) -> TreeMap<u64, App> {
        let mut apps: TreeMap<u64, App> = TreeMap::new(b"t");
        apps.insert(
            &1,
            &App {
                id: 1,
                title: "Pokemon".to_string(),
                genre: "games".to_string(),
                price: 1_000_000_000_000_000_000_000_000,
                published_on: env::block_timestamp_ms(),
                developer: developer_account.clone(),
            },
        );
        apps.insert(
            &2,
            &App {
                id: 2,
                title: "GTA V".to_string(),
                genre: "games".to_string(),
                price: 2_000_000_000_000_000_000_000_000,
                published_on: env::block_timestamp_ms(),
                developer: developer_account.clone(),
            },
        );
        apps.insert(
            &3,
            &App {
                id: 3,
                title: "Uncharted".to_string(),
                genre: "games".to_string(),
                price: 3_000_000_000_000_000_000_000_000,
                published_on: env::block_timestamp_ms(),
                developer: developer_account.clone(),
            },
        );
        apps.insert(
            &4,
            &App {
                id: 4,
                title: "Telegram".to_string(),
                genre: "entertainment".to_string(),
                price: 4_000_000_000_000_000_000_000_000,
                published_on: env::block_timestamp_ms(),
                developer: developer_account.clone(),
            },
        );
        apps.insert(
            &5,
            &App {
                id: 5,
                title: "Discord".to_string(),
                genre: "entertainment".to_string(),
                price: 5_000_000_000_000_000_000_000_000,
                published_on: env::block_timestamp_ms(),
                developer: developer_account,
            },
        );
        apps
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

    // // When buying app:
    // // 1. Transaction is signed by buyer
    // // 2. Store gets 50% of app price
    // // 3. Remaining 50% goes to app owner
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

        p1
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId};

    use super::*;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        let developer_account = AccountId::new_unchecked("developer.near".to_string());
        builder.signer_account_id(developer_account.clone());
        builder.predecessor_account_id(developer_account);
        builder
    }

    #[test]
    fn publish_app() {
        let context = get_context();
        testing_env!(context.build());

        let mut store = AppStore::new(env::signer_account_id());
        assert_eq!(store.list_apps().len(), 5);
        let result = store.publish_app(
            "Red Dead Redemption".to_string(),
            "games".to_string(),
            U128(8 * near_sdk::ONE_NEAR),
        );
        assert!(result);
        assert_eq!(6, store.list_apps().len());
    }
}
