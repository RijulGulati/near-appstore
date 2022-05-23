MASTER_ACCOUNT=$1;

if [ -z $MASTER_ACCOUNT ]
then
    echo "error: master account not supplied"
    return 0;
fi

echo "\nFetching balance\n"

STORE_BAL=$(near state $1 | grep -o "'.*'" | sed "s/'//g" | tail -1)
DEVELOPER_BAL=$(near state developer-$1 | grep -o "'.*'" | sed "s/'//g" | tail -1)
BUYER_BAL=$(near state buyer-$1 | grep -o "'.*'" | sed "s/'//g" | tail -1)

echo "$MASTER_ACCOUNT: $STORE_BAL NEAR (appstore)"
echo "developer-$MASTER_ACCOUNT: $DEVELOPER_BAL NEAR (developer)"
echo "buyer-$MASTER_ACCOUNT: $BUYER_BAL NEAR (buyer)\n"