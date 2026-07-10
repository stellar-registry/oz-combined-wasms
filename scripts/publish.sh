#!/usr/bin/env bash
#
# Download all WASMs from the latest oz-combined-wasms GitHub release and
# publish each to the OZ sub-registry on the Stellar network.
#
# Phases:
#   0. Preflight  — verify tools, network config, and admin key
#   1. Download   — fetch all *.wasm files from the latest GitHub release
#   2. Publish    — `stellar registry publish --wasm-name oz/<name>` for each WASM
#
# SAFETY: dry-run by default. Pass --execute to actually submit transactions.
#
set -euo pipefail

# ---------------------------------------------------------------------------
# Config (override via env)
# ---------------------------------------------------------------------------
NETWORK="${STELLAR_NETWORK:-mainnet}"
ADMIN="${REGISTRY_ADMIN:-theahaco}"
RPC_URL="${STELLAR_RPC_URL:-https://mainnet.sorobanrpc.com}"
PASSPHRASE="${STELLAR_NETWORK_PASSPHRASE:-Public Global Stellar Network ; September 2015}"
GH_REPO="stellar-registry/oz-combined-wasms"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/.."
# TAG and WORK_DIR are resolved in download_wasms once we know the latest release.
TAG="${TAG:-}"       # override to pin a specific release
WORK_DIR="${WORK_DIR:-}"  # defaults to target/wasms/<tag>/ if unset

# ---------------------------------------------------------------------------
# Args
# ---------------------------------------------------------------------------
DRY_RUN=1

usage() {
    cat <<EOF
Usage: $0 [options]

  --execute       Actually submit transactions (default: dry-run, prints only)
  -n, --dry-run   Force dry-run (default)
  -h, --help      Show this help

Downloads all WASMs from the latest GitHub release of $GH_REPO and
publishes each to the OZ sub-registry.

Env overrides: STELLAR_NETWORK ($NETWORK), REGISTRY_ADMIN ($ADMIN),
               STELLAR_RPC_URL, STELLAR_NETWORK_PASSPHRASE, WORK_DIR
EOF
}

for arg in "$@"; do
    case "$arg" in
        --execute)    DRY_RUN=0 ;;
        -n|--dry-run) DRY_RUN=1 ;;
        -h|--help)    usage; exit 0 ;;
        *) echo "Unknown option: $arg" >&2; usage >&2; exit 1 ;;
    esac
done

# ---------------------------------------------------------------------------
# Helpers (mirrored from ../contracts/contracts/registry/deploy_mainnet.sh)
# ---------------------------------------------------------------------------
log()  { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m!!\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31mERROR:\033[0m %s\n' "$*" >&2; exit 1; }

run() {
    if [ "$DRY_RUN" -eq 1 ]; then
        printf '[dry-run] '
        printf '%q ' "$@"
        printf '\n'
    else
        "$@"
    fi
}

net_args=(--network "$NETWORK" --source "$ADMIN" --inclusion-fee 10000000)

# ---------------------------------------------------------------------------
# Phase 0 — preflight
# ---------------------------------------------------------------------------
preflight() {
    log "Phase 0: preflight (network=$NETWORK admin=$ADMIN)"

    command -v stellar >/dev/null 2>&1 || die "stellar CLI not found on PATH"
    command -v gh >/dev/null 2>&1      || die "gh (GitHub CLI) not found on PATH"

    if ! stellar network ls 2>/dev/null | grep -qx "$NETWORK"; then
        warn "network '$NETWORK' not configured; adding it"
        run stellar network add "$NETWORK" \
            --rpc-url "$RPC_URL" \
            --network-passphrase "$PASSPHRASE"
    fi

    if [ "$PASSPHRASE" != "Public Global Stellar Network ; September 2015" ]; then
        warn "passphrase override in effect — this is NOT the standard Pubnet passphrase"
    fi

    stellar keys public-key "$ADMIN" >/dev/null 2>&1 \
        || die "admin key '$ADMIN' not found (add it with 'stellar keys add $ADMIN')"

    if [ "$DRY_RUN" -eq 1 ]; then
        warn "DRY-RUN: no transactions will be submitted. Re-run with --execute to publish."
    else
        warn "EXECUTE mode: this WILL submit MAINNET transactions and spend real XLM as '$ADMIN'."
        printf 'Type "mainnet" to continue: '
        read -r confirm
        [ "$confirm" = "mainnet" ] || die "aborted"
    fi
}

# ---------------------------------------------------------------------------
# Phase 1 — download WASMs (cached in target/wasms/<tag>/)
# ---------------------------------------------------------------------------
download_wasms() {
    if [ -z "$TAG" ]; then
        TAG="$(gh release view --repo "$GH_REPO" --json tagName -q .tagName)"
    fi
    if [ -z "$WORK_DIR" ]; then
        WORK_DIR="$REPO_ROOT/target/wasms/$TAG"
    fi

    if ls "$WORK_DIR"/*.wasm 2>/dev/null | grep -q .; then
        log "Phase 1: using cached WASMs from $WORK_DIR"
        return
    fi

    log "Phase 1: downloading WASMs from $GH_REPO release $TAG"
    mkdir -p "$WORK_DIR"
    gh release download "$TAG" --repo "$GH_REPO" --dir "$WORK_DIR" --pattern "*.wasm"
    local count
    count=$(ls "$WORK_DIR"/*.wasm 2>/dev/null | wc -l | tr -d ' ')
    log "Downloaded $count WASM(s) to $WORK_DIR"
}

# ---------------------------------------------------------------------------
# Phase 2 — publish each WASM to the OZ sub-registry
# ---------------------------------------------------------------------------
publish_wasms() {
    log "Phase 2: publishing WASMs (wasm-name prefix: oz/)"
    for wasm in "$WORK_DIR"/*.wasm; do
        local filename name binver
        filename="$(basename "$wasm")"
        # Filename format from build.yml: <name>_v<version>.wasm
        # e.g. ft-standard_v0.7.2.wasm
        name="${filename%_v*.wasm}"
        binver="${filename#*_v}"
        binver="${binver%.wasm}"

        # Skip if already published (allows safe retries)
        local existing_hash
        existing_hash="$(stellar registry fetch-hash "oz/$name" \
            --version "$binver" \
            "${net_args[@]}" 2>/dev/null)" || true
        if [ -n "$existing_hash" ]; then
            log "  oz/$name v$binver already published (hash: $existing_hash), skipping"
            continue
        fi

        log "Publishing oz/$name v$binver"
        run stellar registry publish \
            --wasm "$wasm" \
            --wasm-name "oz/$name" \
            --binver "$binver" \
            --author "$ADMIN" \
            "${net_args[@]}"
    done
}

# ---------------------------------------------------------------------------
main() {
    preflight
    download_wasms
    publish_wasms
    log "Done."
}

main
