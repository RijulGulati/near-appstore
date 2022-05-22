use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::TreeMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, Promise,
};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct App {
    id: u64,
    title: String,
    genre: String,
    price: u128,
    published_on: u64,
    developer: String,
}

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
pub struct AppStore {
    apps: TreeMap<u64, App>,
}

#[near_bindgen]
impl AppStore {
    #[init]
    pub fn new() -> Self {
        Self {
            apps: TreeMap::new(b"t"),
        }
    }

    pub fn publish_app(&mut self, title: String, genre: String, yocto_price: String) -> bool {
        let price = match yocto_price.parse::<u128>() {
            Ok(p) => p,
            Err(e) => {
                env::panic_str(e.to_string().as_str());
            }
        };

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
                    developer: env::signer_account_id().to_string(),
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
    pub fn buy_app(&mut self, id: u64) -> bool {
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

        let store_fee = app.price / 2;
        let developer_fee = app.price - store_fee;
        Promise::new(AccountId::new_unchecked(app.developer.to_string())).transfer(developer_fee);
        Promise::new(env::current_account_id()).transfer(store_fee);
        true
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId};

    use super::*;

    fn get_context(predecessor: AccountId, attach_deposit: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        if attach_deposit {
            builder.attached_deposit(5 * near_sdk::ONE_NEAR);
        }
        builder
    }

    #[test]
    fn publish_app() {
        let predecessor_account = AccountId::new_unchecked("alice.near".to_string());
        let context = get_context(predecessor_account, false);
        testing_env!(context.build());

        let mut store = AppStore::new();
        let result = store.publish_app(
            "Pokemon".to_string(),
            "games".to_string(),
            (5 * near_sdk::ONE_NEAR).to_string(),
        );
        assert!(result);

        let apps = store.list_apps();
        assert_eq!(1, apps.len());
    }

    #[test]
    fn buy_app() {
        let predecessor_account = AccountId::new_unchecked("alice.near".to_string());
        let context = get_context(predecessor_account, true);
        testing_env!(context.build());

        let mut store = AppStore::new();
        let result = store.publish_app(
            "Pokemon".to_string(),
            "games".to_string(),
            (3 * near_sdk::ONE_NEAR).to_string(),
        );
        assert!(result);

        let buy_result = store.buy_app(1);
        assert!(buy_result);
    }
}
