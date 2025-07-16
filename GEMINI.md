# Gemini CLI Project Context

This file serves as a persistent memory for the Gemini CLI agent regarding the `paperWalletVerification/near-protocol` project.

## Project Overview

**Goal:** Build an automated system to securely record blockchain account information (non-NEAR accounts) and their hashed passwords from a Supabase database onto the NEAR Protocol blockchain. One account can have multiple hashed passwords. The passwords in Supabase are already hashed and will be stored on NEAR without further modification.

**Architecture:** User App -> Supabase DB (Webhook) -> Backend (Supabase Function) -> NEAR Smart Contract

## Current Status

**Phase 1: Environment Setup & Smart Contract Development - COMPLETE**
*   Rust project initialized (`onchain-registry/`).
*   `Cargo.toml` configured with `near-sdk` (v5.0.0-alpha.1 with `unit-testing`, `legacy` features) and `borsh`.
*   NEAR Smart Contract (`onchain-registry/src/lib.rs`) implemented:
    *   Storage: `UnorderedMap<AccountId, Vec<String>>`
    *   Functions: `new`, `add_password`, `get_passwords`
*   Unit tests implemented and passing.
*   Contract compiled to `.wasm` (`onchain_registry.wasm`).
*   Contract successfully deployed to NEAR Testnet (`geminitest.testnet`).
    *   Transaction ID: `32iEQWkaaU48ejNv13UUXP9d5xnYmerR8aQjwDYLi3xd`
    *   Explorer Link: `https://testnet.nearblocks.io/txns/32iEQWkaaU48ejNv13UUXP9d5xnYmerR8aQjwDYLi3xd`

**Known Issues/Warnings:**
*   `AccountId::new_unvalidated` deprecation warning in `src/lib.rs`. This is a warning, not an error, and does not block compilation or functionality.

## Next Steps

**Phase 2: Backend Service Development (Supabase Function)**
*   **First Task:** Set up local Supabase environment (`supabase init`, `supabase start`).

## Key Decisions & Troubleshooting Highlights

*   **NEAR SDK Version:** Settled on `5.0.0-alpha.1` (resolves to `5.15.1`) with `unit-testing` and `legacy` features to overcome `parity-secp256k1` and `core` compilation issues.
*   **`AccountId` Construction:** Reverted to `AccountId::new_unvalidated` due to `new_validated` not being supported in the chosen `near-sdk` version.
*   **WASM Compilation:** Added `crate-type = ["cdylib"]` to `Cargo.toml` to ensure `.wasm` file generation.
*   **`near-cli` Credentials:** Resolved persistent "do not have credentials locally" error by using `near login` (after `rustup` reinstallation) and confirming successful web login.
*   **Rust Toolchain Issues:** Resolved `can't find crate for core` error through a full `rustup` reinstallation and careful management of `near-sdk` versions/features.

## How to Resume

When you return, please instruct me to:
`read_file(absolute_path='/Users/namhyeongseog/Desktop/TODO/paperWalletVerification/near-protocol/GEMINI.md')`
or simply say:
`Load project context.`
