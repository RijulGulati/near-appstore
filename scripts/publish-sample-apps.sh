MASTER_ACCOUNT=$1;

if [ -z $MASTER_ACCOUNT ]
then
    echo "error: master account not supplied"
    return 0;
fi

near call $MASTER_ACCOUNT publish_app '{"title":"Pokemon", "genre":"games", "yocto_price":"2500000000000000000000000"}' --accountId developer-$MASTER_ACCOUNT
near call $MASTER_ACCOUNT publish_app '{"title":"PUBG", "genre":"games", "yocto_price":"2000000000000000000000000"}' --accountId developer-$MASTER_ACCOUNT
near call $MASTER_ACCOUNT publish_app '{"title":"WhatsApp", "genre":"entertainment", "yocto_price":"300000000000000000000000"}' --accountId developer-$MASTER_ACCOUNT
near call $MASTER_ACCOUNT publish_app '{"title":"Discord", "genre":"entertainment", "yocto_price":"400000000000000000000000"}' --accountId developer-$MASTER_ACCOUNT
near call $MASTER_ACCOUNT publish_app '{"title":"Telegram", "genre":"entertainment", "yocto_price":"500000000000000000000000"}' --accountId developer-$MASTER_ACCOUNT