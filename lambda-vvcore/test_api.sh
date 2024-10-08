#!/bin/bash

set -ea

API_ENDPOINT=${API_ENDPOINT:-"http://localhost:9000/2015-03-31/functions/function/invocations"}
LAMBDA_APIKEY=${LAMBDA_APIKEY:-"test"}

date_str=$(date "+%Y年%m月%d日 %H時%M分%S秒")
JSON='{"text": "現在の時刻は '$date_str' です。"}'
echo "$JSON"

for speaker_id in 0 1 2
do
    JSON='{"text": "現在の時刻は '$date_str' です。", "speaker_id": '$speaker_id'}'
    response=$(curl -XPOST "$API_ENDPOINT" -H "Content-Type: application/json" -d "$JSON" -H "Authorization: Bearer $LAMBDA_APIKEY")
    # もし API_ENDPOINT が http: で 始まる場合は 2段階の jq で取り出す
    if [[ $API_ENDPOINT == "http:"* ]]; then
        echo $response | jq -r '.body' | jq -r '.wav' | base64 -d > "response${speaker_id}.wav"
    else
        echo $response | jq -r '.wav' | base64 -d > "response${speaker_id}.wav"
    fi
    echo "WAV file saved as response${speaker_id}.wav"
done
