# oz-combined-wasms

Pre-built Soroban WASM files for every curated combination of
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
│   └── check-extensions.py    # Detects new OZ extension directories
├── known-extensions.toml      # Extensions the generator knows about
└── .github/workflows/
    ├── build.yml               # Builds WASMs on tag push
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

## Releasing

After merging an update PR, push a version tag to trigger `build.yml`:

```bash
git tag v0.7.3 && git push origin v0.7.3
```

This builds all 31 combinations in parallel using
[soroban-build-workflow](https://github.com/stellar-expert/soroban-build-workflow)
and attaches the WASM files to a GitHub release.
