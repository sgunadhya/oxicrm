#!/bin/bash
set -e

BASE_URL="http://localhost:3001"

echo "Waiting for server to be ready..."
until curl -s "$BASE_URL/health" > /dev/null; do
  echo "Waiting for server..."
  sleep 2
done
echo "Server is up!"

echo "1. Creating Object 'Ticket'..."
RESPONSE=$(curl -s -X POST "$BASE_URL/api/objects" \
  -H "Content-Type: application/json" \
  -d '{
    "name_singular": "Ticket",
    "name_plural": "Tickets",
    "description": "Support Tickets"
  }')
echo "Response: $RESPONSE"
OBJECT_ID=$(echo $RESPONSE | jq -r '.id')
echo "Object ID: $OBJECT_ID"

if [ "$OBJECT_ID" == "null" ]; then
  echo "Failed to create object."
  exit 1
fi

echo "2. Creating Field 'Priority'..."
curl -s -X POST "$BASE_URL/api/objects/$OBJECT_ID/fields" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Priority",
    "field_type": "Text",
    "settings": {}
  }'
echo ""

echo "3. Creating Record (Priority: High)..."
RESPONSE=$(curl -s -X POST "$BASE_URL/api/objects/$OBJECT_ID/records" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "Priority": "High",
      "Description": "Server is down"
    }
  }')
echo "Response: $RESPONSE"
RECORD_ID=$(echo $RESPONSE | jq -r '.id')
echo "Record ID: $RECORD_ID"

echo "4. Listing Records..."
curl -s "$BASE_URL/api/objects/$OBJECT_ID/records" | jq .
echo ""

echo "5. Updating Record (Priority: Low)..."
curl -s -X PUT "$BASE_URL/api/records/$RECORD_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "Priority": "Low",
      "Description": "Server is down"
    }
  }' | jq .
echo ""

echo "6. Getting Record..."
curl -s "$BASE_URL/api/records/$RECORD_ID" | jq .
echo ""

echo "7. Deleting Record..."
curl -s -X DELETE "$BASE_URL/api/records/$RECORD_ID"

echo "8. Verifying UI Routes..."
echo "Checking /settings/objects..."
curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/settings/objects" | grep 200 && echo "OK" || echo "Failed"

echo "Checking /settings/objects/new..."
curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/settings/objects/new" | grep 200 && echo "OK" || echo "Failed"

echo "Checking /app/objects/$OBJECT_ID..."
curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/app/objects/$OBJECT_ID" | grep 200 && echo "OK" || echo "Failed"

echo "Checking /app/objects/$OBJECT_ID/new..."
curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/app/objects/$OBJECT_ID/new" | grep 200 && echo "OK" || echo "Failed"

echo "Done."
