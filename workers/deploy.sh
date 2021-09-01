#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

if [[ "$#" != "3" ]]; then
  echo "Expected syntax: $0 [worker] [environment] [sha]" >&2
  exit 1
fi

worker="$1"
environment="$2"
sha="$3"

case "$environment" in
  staging)
    domain="staging.initiative.sh"
    ;;
  production)
    domain="initiative.sh"
    ;;
  *)
    echo "Invalid environment: $environment" >&2
    exit 1
esac

echo "Deploying worker $worker build $sha to $environment..."
cd "$worker"

echo "Installing Wrangler"
npm i @cloudflare/wrangler -g
wrangler whoami

echo "Publishing worker..."
sed 's/^GITHUB_SHA\b.*/GITHUB_SHA = "'"$sha"'"/g' -i wrangler.toml
wrangler publish --env "$environment"

echo "Verifying build..."
set -x
for _ in $(seq 1 6); do
  sleep 10
  if [[ "$(curl "https://$domain/$worker/healthcheck")" == "Health check OK on build ${sha}" ]]; then
    echo "Verification successful"
    exit 0
  fi
done

echo "Verification failed"
exit 1
