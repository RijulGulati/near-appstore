# NEAR App Store

This project is built for NCD (NEAR Certified Developer) demonstration.

# About

NEAR App store is an app store simulation smart contract. The flow is as follows:

- Developers can publish application to store. They need to specify app `title`, `genre` and `yocto_price` (yoctoNEAR). (function: `publish_app`)
- Buyers can buy apps from app store. They need to attach deposit to contract call. (function: `buy_app`)
- Apps can be listed with the help of exposed view function (function: `list_apps`)

**Store economics**

- For every app bought by the buyer, 50% of app price is paid to the developer, and remaining 50% of price is paid to the store.
  - Example: If an app costs 2 NEAR, 1 NEAR is sent to developer, and remaining 1 NEAR is sent to app store.
- If buyer's deposit is less than app price, contract panics with error message.
- **If buyer's deposit is greater than app price, the additional amount is refunded back to the buyer.**

# Initial Setup

## Build and Deploy contract

This requires Rust and NEAR CLI installed in system.

```sh
$ cd scripts
$ ./deploy.sh
```

Success message looks something like this -

```
Transaction Id 13THp9TjmmsH9Spi6FydcHqdZqNiTGxc4fRpWb6oZgQS
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/13THp9TjmmsH9Spi6FydcHqdZqNiTGxc4fRpWb6oZgQS
Done deploying and initializing dev-1653307261772-38523729167455
```

Note the developer account name (Name will be different for you)

```sh
$ NEAR_APPSTORE=dev-1653307261772-38523729167455
```

## Setup accounts

After deployment, we need to setup two accounts:

- Buyer account: Account from which buyer pays for application. The account is of format: `developer-$NEAR_APPSTORE`.
- Developer account: Account developer uses to publish application and receive royalties. The account is of format: `buyer-$NEAR_APPSTORE`.

The provided helper script will setup these accounts for us.

```sh
$ ./setup-accounts.sh $NEAR_APPSTORE
```

**This completes initial setup!**

---

# Core functionalities

## List apps in store

```sh
$ near view $NEAR_APPSTORE list_apps
```

## Publish new app to store

Assuming the App Developer publishes new app to store using account `developer-$NEAR_APPSTORE` account (account `developer-$NEAR_APPSTORE` was created with `setup-accounts.sh` script)

```sh
$ near call $NEAR_APPSTORE publish_app '{"title":"Youtube", "genre":"entertainment", "yocto_price":"9000000000000000000000000"}' --accountId developer-$NEAR_APPSTORE
```

## Check balances of accounts

```sh
$ cd scripts
$ ./get-balance.sh $NEAR_APPSTORE
```

## Buy app

Assuming a buyer buys app from store:

- The buyer signs the transaction and will have to deposit app price equivalent of NEAR.
- 50% of app price is paid to developer.
- Remaining 50% is paid to app store.
- If supplied deposit is greater than app price, remaining amount is refunded back to buyer

```sh
$ near call $MASTER_ACCOUNT buy_app '{"id": 2}' --deposit 2 --accountId buyer-$MASTER_ACCOUNT
```

### Recheck accounts balance

```
$ ./get-balance.sh $NEAR_APPSTORE
```
