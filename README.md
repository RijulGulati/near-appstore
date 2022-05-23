# NEAR App Store

App store on NEAR protocol. This project is built for NCD (NEAR Certified Developer) demonstration.

## Initial Setup

### Build and Deploy contract

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

### Setup accounts

After deployment, we will setup a few accounts. Simply run `setup-accounts.sh` script.

```sh
$ ./setup-accounts.sh $NEAR_APPSTORE
```

**This completes initial setup!**

## Core functionalities

### List apps in store

```sh
$ near view $NEAR_APPSTORE list_apps
```

### Publish new app to store

Assuming the App Developer publishes new app to store using account `developer-$NEAR_APPSTORE` account (account `developer-$NEAR_APPSTORE` was created with `setup-accounts.sh` script)

```sh
$ near call $NEAR_APPSTORE publish_app '{"title":"Youtube", "genre":"entertainment", "yocto_price":"9000000000000000000000000"}' --accountId developer-$NEAR_APPSTORE
```

### Check balances of accounts

```sh
$ cd scripts
$ ./get-balance.sh $NEAR_APPSTORE
```

### Buy app

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
