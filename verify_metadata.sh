#!/bin/bash
set -e

BASE_URL="http://localhost:3001"

echo "1. Creating Object 'Project Beta'..."
RESPONSE=$(curl -s -X POST "$BASE_URL/api/objects" \
  -H "Content-Type: application/json" \
  -d '{
    "name_singular": "Project Beta",
    "name_plural": "Projects Beta",
    "description": "Project Management Beta"
  }')
echo "Response: $RESPONSE"
OBJECT_ID=$(echo $RESPONSE | jq -r '.id')
echo "Object ID: $OBJECT_ID"

if [ "$OBJECT_ID" == "null" ]; then
  echo "Failed to create object or object already exists."
  exit 1
fi

echo "2. Creating Field 'Budget'..."
curl -s -X POST "$BASE_URL/api/objects/$OBJECT_ID/fields" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Budget",
    "field_type": "Number",
    "settings": {}
  }'
echo ""

echo "3. Creating View 'All Projects'..."
curl -s -X POST "$BASE_URL/api/views" \
  -H "Content-Type: application/json" \
  -d '{
    "object_metadata_id": "'"$OBJECT_ID"'",
    "name": "All Projects",
    "view_type": "Table",
    "filters": {},
    "sort": {}
  }'
echo ""

echo "4. Getting Schema..."
curl -s "$BASE_URL/api/schema" | jq .
echo ""

echo "5. Listing Views..."
curl -s "$BASE_URL/api/views?object_metadata_id=$OBJECT_ID" | jq .
echo ""
