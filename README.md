# oz-combined-wasms

Pre-built Soroban Wasm files for every curated combination of
[OpenZeppelin Stellar Contracts](https://github.com/OpenZeppelin/stellar-contracts),
published to the [Stellar Registry](https://rgstry.xyz).

Instead of writing boilerplate, grab the combination you need directly from
Registry and deploy it.

## Available combinations

| Name | Description |
|---|---|
| `ft-standard` | SEP-41 fungible token, Ownable, Burnable, Upgradeable |
| `ft-capped` | + maximum supply cap |
| `ft-pausable` | + emergency pause |
| `ft-capped-pausable` | + both |
| `ft-allowlist` | AllowList transfer restriction, AccessControl |
| `ft-allowlist-capped` | + cap |
| `ft-allowlist-pausable` | + pause |
| `ft-allowlist-capped-pausable` | + both |
| `ft-blocklist` | BlockList transfer restriction, AccessControl |
| `ft-blocklist-capped` | + cap |
| `ft-blocklist-pausable` | + pause |
| `ft-blocklist-capped-pausable` | + both |
| `ft-votes` | Governance token with voting power tracking |
| `ft-votes-capped` | + cap |
| `ft-votes-pausable` | + pause |
| `ft-votes-capped-pausable` | + both |
| `ft-vault` | ERC-4626 tokenized vault |
| `nft-standard` | Sequential-minting NFT, Ownable, Burnable, Upgradeable |
| `nft-pausable` | + pause |
| `nft-royalties` | + ERC-2981 royalties, AccessControl |
| `nft-royalties-pausable` | + both |
| `nft-enumerable` | On-chain token enumeration |
| `nft-enumerable-pausable` | + pause |
| `nft-enumerable-royalties` | + royalties, AccessControl |
| `nft-enumerable-royalties-pausable` | + both |
| `nft-consecutive` | Efficient batch minting |
| `nft-consecutive-pausable` | + pause |
| `nft-consecutive-royalties` | + royalties, AccessControl |
| `nft-consecutive-royalties-pausable` | + both |
| `nft-votes` | NFT with voting power tracking |
| `nft-votes-pausable` | + pause |

All combinations are compiled against the OZ package versions pinned in
`Cargo.toml` (`[workspace.dependencies]`).

## Repository structure

```
.
├── src/                        # Generator source (Rust)
│   ├── main.rs                 # Entry point — run with `cargo run`
│   ├── config.rs               # Curated list of combinations
│   └── fragments/              # Code-generation helpers per feature
├── generated/                  # ⚠️  AUTO-GENERATED — do not edit manually
│   ├── README.md
│   └── <combination>/          # One Cargo crate per combination
├── scripts/
│   ├── check-extensions.py    # Detects new OZ extension directories
│   └── publish.sh             # Downloads Wasms from latest release and publishes to Registry
├── known-extensions.toml      # Extensions the generator knows about
└── .github/workflows/
    ├── release.yml             # Tags on version bump, builds + attests, publishes to Registry
    ├── update.yml              # Updates OZ versions and regenerates
    └── poll.yml                # Hourly: detects new OZ releases
```

## Regenerating `generated/`

```bash
cargo run
```

This deletes and recreates `generated/` from scratch. Run it after editing
`src/config.rs` or bumping OZ dependency versions in `Cargo.toml`.

## Adding a new combination

1. Edit `src/config.rs` — add an entry to `COMBINATIONS`.
2. If the combination uses a new extension type, also update:
   - The relevant enum in `src/config.rs`
   - `src/fragments/imports.rs`
   - `src/fragments/constructor.rs`
   - `src/fragments/nft.rs` or `src/fragments/fungible.rs`
   - `src/fragments/extensions.rs`
   - `known-extensions.toml`
3. Run `cargo run` and commit the updated `generated/`.

## Updating the OZ version

The `poll.yml` workflow checks crates.io hourly and fires the `update.yml`
workflow automatically when a new `stellar-tokens` version appears. That
workflow opens a pull request with:

- Bumped `stellar-*` pins in `Cargo.toml`
- Regenerated `generated/`
- A GitHub Issue if new extension directories were detected

You can also trigger it manually from the Actions tab or via the API:

```bash
curl -X POST \
  -H "Authorization: Bearer $GH_TOKEN" \
  -H "Content-Type: application/json" \
  https://api.github.com/repos/stellar-registry/oz-combined-wasms/dispatches \
  -d '{"event_type":"oz-release","client_payload":{"version":"0.7.3"}}'
```

## Releasing & publishing

**Merging the update PR is the release.** On every push to `main`,
`release.yml` (built from the reusable
[stellar-registry/actions](https://github.com/stellar-registry/actions)
workflows) runs the whole chain in one workflow run:

1. **detect-releases** — if `[workspace.package].version` has no matching
   `v<version>` tag yet (i.e. the merged PR bumped it), tag it. Pure git — no
   manual `git tag` step.
2. **build** — compile all 31 combinations in a single job with
   `stellar contract build`, attach every WASM to one GitHub release, submit
   hashes to stellar.expert contract validation, and sign a build provenance
   attestation for each. Each WASM carries `source_repo` and `home_domain`
   metadata.
3. **publish** — verify each asset's attestation, then publish each to the
   `oz/` sub-registry with `stellar-registry publish`, skipping combinations
   already on-chain (same hash) and refusing a same-version different-hash
   asset. A `registry-publish-receipts.json` is attached to the release.

Because the tag is created with `GITHUB_TOKEN` and the downstream jobs run via
`needs:` in the same run, no GitHub App or manual tag push is involved — a
manually pushed tag does nothing.

Required repo configuration:

- secret `REGISTRY_PUBLISH_SECRET_KEY` — `S…` seed of the account bound as
  author of the `oz/*` names (fee source + publish signer)
- optional var `REGISTRY_AUTHOR_ADDRESS` — `G…` author override; unset means
  the signing key's own account

**Recovery:** if the on-chain half fails transiently, run the `Release`
workflow manually (`workflow_dispatch`) with `publish_tag: v<version>` — it
republishes the existing attested assets without rebuilding, skipping
whatever already landed. If the *build* half fails, use the failed run's
"Re-run failed jobs" (deleting the GitHub release first if one was created;
keep the tag).

### Manual fallback: `scripts/publish.sh`

The publish half can still be run locally from a machine holding the admin
key:

```bash
# Dry-run first — prints every command without submitting
./scripts/publish.sh

# Then execute for real
./scripts/publish.sh --execute
```

The script downloads WASMs from the latest GitHub release into
`target/wasms/<tag>/`, checks `stellar registry fetch-hash oz/<name>` before
each publish, and skips already-published WASMs — safe to re-run after
transient failures.

Config via env vars: `STELLAR_NETWORK` (default `mainnet`), `REGISTRY_ADMIN` (default `theahaco`),
`TAG` (default: latest release), `WORK_DIR` (default: `target/wasms/<tag>/`).
