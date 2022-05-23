
MASTER_ACCOUNT=$1;

if [ -z $MASTER_ACCOUNT ]
then
    echo "error: master account not supplied"
    return 0;
fi

echo "CREATING ACCOUNTS"
near create-account developer-$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT --initialBalance 10
near create-account buyer-$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT --initialBalance 20
