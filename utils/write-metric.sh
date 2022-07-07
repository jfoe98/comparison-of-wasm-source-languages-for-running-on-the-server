#!/bin/sh

language=$1 # source language (i. e. csharp, c, java etc.)
name=$2     # type of metric (i. e. memory, compile time, performance etc.)
context=$3  # used algorithm / program (i. e. fibonacci, i/o etc.)
value=$4    # measured value

echo "Writing Metric: Language $language, Name $name, Context $context, Value $value"

JSON_BODY=$(cat <<-END
    {
        "language": "${language}", 
        "name": "${name}", 
        "context": "${context}", 
        "value": "${value}"
    }
END
)

curl -i \
    -H "Content-Type:application/json" \
    -X POST --data "${JSON_BODY}" "http://10.95.9.113:3000/metrics"
