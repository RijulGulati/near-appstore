# NEAR App Store

This project is built for NCD (NEAR Certified Developer) demonstration.

# About

NEAR App store is an app store simulation smart contract. Following actions can be performed:

- Developers can publish application to store. They need to specify app `title`, `genre` and `yocto_price` (yoctoNEAR). (function: `publish_app`)
- Buyers can buy apps from app store. They need to attach deposit to contract call. (function: `buy_app`).
- List all available apps in appstore (function: `list_apps`)
- List apps bought by a particular buyer (function: `list_buyer_apps`)

**Store economics**

- For every app bought by the buyer, 50% of app price is paid to the developer, and remaining 50% of price is paid to the store.
  - Example: If an app costs 2 NEAR, 1 NEAR is sent to developer, and remaining 1 NEAR is sent to app store.
- If buyer's deposit is less than app price, contract panics with appropriate error message.
- If buyer's deposit is greater than app price, the additional amount is refunded back to the buyer.

# Initial Setup

## 1. Build and Deploy contract

This requires Rust and NEAR CLI installed in system.

```sh
cd scripts
./deploy.sh
```

Success message looks something like this -

```
Transaction Id 13THp9TjmmsH9Spi6FydcHqdZqNiTGxc4fRpWb6oZgQS
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/13THp9TjmmsH9Spi6FydcHqdZqNiTGxc4fRpWb6oZgQS
Done deploying and initializing dev-1653307261772-38523729167455
```

Note the developer account name from above (`dev-1653307261772-38523729167455`) (Name will be different for you). Let's assign this account name to a variable for easy access.

```sh
NEAR_APPSTORE=dev-1653307261772-38523729167455 # account name will be different for you
```

## 2. Setup accounts

After deployment, we need to setup two accounts:

- Buyer account: Account from which buyer pays for application. The account is of format: `buyer-$NEAR_APPSTORE`.
- Developer account: Account developer uses to publish application and receive royalties. The account is of format: `developer-$NEAR_APPSTORE`.

The provided helper script will setup these accounts for us.

```sh
cd scripts
./setup-accounts.sh $NEAR_APPSTORE
```

## 3. Publish sample apps to store (optional)

```sh
cd scripts
./publish-sample-apps.sh $NEAR_APPSTORE
```

This script publishes 5 sample apps to appstore. All these apps have developer set to `developer-$NEAR_APPSTORE`. Alternatively, NEAR CLI can be used directly to publish app (as mentioned below).

**This completes initial setup!**

---

# Core functionalities

## List apps in store

```sh
near view $NEAR_APPSTORE list_apps
```

Sample respnose -

```json
[
  {
    "id": 1,
    "title": "Pokemon",
    "genre": "games",
    "price": 2.5e24,
    "published_on": 1653383267193,
    "developer": "developer-dev-1653383057039-59136275303926"
  },
  {
    "id": 2,
    "title": "PUBG",
    "genre": "games",
    "price": 2e24,
    "published_on": 1653383272261,
    "developer": "developer-dev-1653383057039-59136275303926"
  },
  {
    "id": 3,
    "title": "WhatsApp",
    "genre": "entertainment",
    "price": 3e23,
    "published_on": 1653383278450,
    "developer": "developer-dev-1653383057039-59136275303926"
  },
  {
    "id": 4,
    "title": "Discord",
    "genre": "entertainment",
    "price": 4e23,
    "published_on": 1653383286354,
    "developer": "developer-dev-1653383057039-59136275303926"
  },
  {
    "id": 5,
    "title": "Telegram",
    "genre": "entertainment",
    "price": 5e23,
    "published_on": 1653383291710,
    "developer": "developer-dev-1653383057039-59136275303926"
  }
]
```

## Publish new app to store

Assuming the App Developer publishes new app to store using account `developer-$NEAR_APPSTORE` account (account `developer-$NEAR_APPSTORE` was created with `setup-accounts.sh` script)

```sh
near call $NEAR_APPSTORE publish_app '{"title":"Youtube", "genre":"entertainment", "yocto_price":"6000000000000000000000000"}' --accountId developer-$NEAR_APPSTORE
```

## Check balances of accounts

This script checks balances for accounts created with `setup-accounts.sh`

```sh
cd scripts
./get-balance.sh $NEAR_APPSTORE
```

Sample response -

```
Fetching balance

dev-1653372617003-54598823973395: 170.9967835824049317 NEAR (appstore)
developer-dev-1653372617003-54598823973395: 10.9964139969337521 NEAR (developer)
buyer-dev-1653372617003-54598823973395: 17.9992326788740635 NEAR (buyer)
```

Balance is in NEAR.

## Buy app

- The buyer signs the transaction and will have to attach deposit equivalent to app price.
  - 50% of app price is paid to developer (`developer-$NEAR_APPSTORE`)
  - Remaining 50% is paid to app store (`$NEAR_APPSTORE`)
- If supplied deposit is greater than app price, remaining amount is refunded back to buyer (`buyer-$NEAR_APPSTORE`)
- If supplied deposit is less than app price, contract panics with appropriate error message.

```sh
near call $NEAR_APPSTORE buy_app '{"id": 2}' --deposit 2 --accountId buyer-$NEAR_APPSTORE
```

**Assuming app price is 2N - On success, re-check account balances. We should see the following result:**

- Developer account is credited with 1N.
- Store account is credited with 1N.
- Buyer account is debited with 2N.

(Actual deductions/final balance might be slightly different because of gas fee).

```
./get-balance.sh $NEAR_APPSTORE
```

## List apps bought by buyer

```sh
near view $NEAR_APPSTORE list_buyer_apps '{"buyer": "buyer-'"$NEAR_APPSTORE"'"}'
```

# License

[MIT](https://github.com/RijulGulati/near-appstore/blob/main/LICENSE)
