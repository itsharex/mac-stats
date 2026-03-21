#!/bin/bash
# Temporary script to query Redmine time entries
source src-tauri/.config.env

if [ -z "$REDMINE_URL" ] || [ -z "$REDMINE_API_KEY" ]; then
  echo "Redmine not configured"
  exit 1
fi

ACTION="${1:-users}"

case "$ACTION" in
  users)
    curl -s -H "X-Redmine-API-Key: $REDMINE_API_KEY" "$REDMINE_URL/users.json?limit=100"
    ;;
  time)
    FROM="${2:-2026-03-09}"
    TO="${3:-2026-03-15}"
    USER_ID="${4:-}"
    URL="$REDMINE_URL/time_entries.json?from=$FROM&to=$TO&limit=100"
    if [ -n "$USER_ID" ]; then
      URL="$URL&user_id=$USER_ID"
    fi
    curl -s -H "X-Redmine-API-Key: $REDMINE_API_KEY" "$URL"
    ;;
esac
