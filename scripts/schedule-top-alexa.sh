#!/bin/bash
INQUISITOR_BIN="/home/hamersaw/Development/rust/inquest/target/debug/inquisitor"
IP_ADDRESS="127.0.0.1"
PORT="12289"
MAX_COUNT=10000
TMP_DIR="/tmp"

#download alexa top 1 million domains
wget -O $TMP_DIR/top-1m.csv.zip http://s3.amazonaws.com/alexa-static/top-1m.csv.zip
unzip -p $TMP_DIR/top-1m.csv.zip > $TMP_DIR/top-1m.csv

#read file
COUNT=0
while read LINE
do
    #parse domain
    DOMAIN=`echo $LINE | cut -f 2 -d ','`

    #TODO specify ip address and port of configuration server
    echo "scheduling $DOMAIN"
    $INQUISITOR_BIN schedule http.$DOMAIN --http $DOMAIN --follow --interval=3600

    #increment counter
    COUNT=$[COUNT+1]
    if [ $COUNT == $MAX_COUNT ];
    then
        break
    fi
done < $TMP_DIR/top-1m.csv

#perform some cleanup
rm $TMP_DIR/top-1m.csv*
