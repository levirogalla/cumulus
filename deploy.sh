#!/bin/bash
set -e

PORT=3001
SERVER=100.117.20.4
SERVER_USER=tadster
SERVER_PROJECT_DIR=/home/$SERVER_USER/projects/cumulus

LOCATION=$1
OPTION=${2:-}


# so svelte knows where to accepts requests from
: "${ORIGIN:=http://localhost:$PORT}"

deploy_local() {
  FETCH=false
  FORCE=false

  # parse optional arguments
  for arg in "$@"; do
    if [ "$arg" = "--yes" ]; then
      FORCE=true
    elif [ "$arg" = "--fetch" ]; then
      FETCH=true
    fi
  done

  if $FETCH; then
    # check for uncommitted changes
    if ! git diff --quiet || ! git diff --cached --quiet; then
      if [ "$FORCE" != "true" ]; then
        echo "working tree has uncommitted changes. use --yes to force"
        exit 1
      fi
    fi

    # fetch remote changes
    git fetch origin main

    # check if local branch differs from remote
    LOCAL=$(git rev-parse HEAD)
    REMOTE=$(git rev-parse origin/main)
    if [ "$LOCAL" != "$REMOTE" ]; then
      if [ "$FORCE" != "true" ]; then
        echo "local branch differs from remote. use --yes to force"
        exit 1
      fi
    fi

    echo "pulling latest changes..."
    git reset --hard origin/main
  fi

  echo "building and deploying for $ORIGIN..."
  ORIGIN=$ORIGIN docker compose up --build
}

if ping -c1 "$SERVER" >/dev/null 2>&1; then
  echo "server reachable."
else
  echo "server not reachable. is it up? is tailscale running?"
  exit 1
fi

if [ "$LOCATION" = "local" ]; then
  deploy_local "$OPTION"
elif [ "$LOCATION" = "remote" ]; then
  ssh "$SERVER_USER@$SERVER" "cd $SERVER_PROJECT_DIR && git pull && ORIGIN=http://$SERVER:$PORT ./deploy.sh local --yes --fetch"
else
  echo "please provide location to deploy to (local or remote)"
  exit 1
fi