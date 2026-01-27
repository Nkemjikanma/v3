# IPFS + ENS Deployment Guide

A guide for building a custom lightweight Rust implementation to deploy static sites to IPFS and update ENS contenthash records for eth.limo access.

## Architecture Overview

```
┌──────────────────────────────────────────────────────────────┐
│                    ipfs-ens-deploy (Rust CLI)                │
├──────────────────────────────────────────────────────────────┤
│  1. Build sites (optional - call npm/zola)                   │
│  2. Upload to IPFS (Pinata API)                              │
│  3. Update ENS contenthash (alloy + wallet signing)          │
└──────────────────────────────────────────────────────────────┘
```

**Note:** eth.limo is just a gateway - no integration needed. It automatically resolves ENS → reads contenthash → fetches from IPFS → serves to browser.

---

## Phase 1: Understand the Concepts

### 1. Learn how IPFS content addressing works
- What is a CID (Content Identifier)
- Difference between CIDv0 (`Qm...`) and CIDv1 (`bafy...`)
- How directories are represented in IPFS (DAG structures)

### 2. Learn how ENS contenthash records work
- ENS namehash algorithm (how `nkem.eth` becomes a bytes32 node)
- Contenthash encoding format (multicodec prefixes for IPFS)
- ENS Resolver contract interface (`setContenthash` function)

### 3. Understand eth.limo
- It's just a gateway - no integration needed
- Resolves ENS → reads contenthash → fetches from IPFS → serves to browser

---

## Phase 2: Build the IPFS Upload Component

### 1. Choose a pinning service API to integrate with
- Pinata, web3.storage, or Filebase
- Study their REST API documentation

### 2. Implement directory upload
- Recursively walk a directory
- Create multipart form data
- Send to pinning API
- Parse response to get CID

### 3. Handle both blog and client
- **Option A:** Build both, merge into single `dist/` folder, upload once
- **Option B:** Upload separately, use ENS subdomains (`blog.nkem.eth`, `nkem.eth`)
- **Option C:** Create a root `index.html` that routes to `/blog` and `/app` subdirectories

---

## Phase 3: Build the ENS Update Component

### 1. Implement namehash algorithm
- Research EIP-137
- Recursive keccak256 hashing of labels

### 2. Implement contenthash encoding
- Research EIP-1577
- Encode CID with proper multicodec prefix (`0xe3010170` for IPFS)

### 3. Interact with ENS Resolver contract
- Set up an Ethereum provider connection
- Load wallet from private key
- Call `setContenthash(node, hash)` on the Public Resolver
- Handle transaction signing and confirmation

---

## Phase 4: CLI Interface

### 1. Design your CLI arguments
- Input directory path(s)
- ENS name
- Optional flags (dry-run, ipfs-only, etc.)

### 2. Wire up the components
- Build sites (optional - shell out to npm/zola)
- Upload to IPFS
- Update ENS
- Print results

---

## Phase 5: CI/CD Integration

### 1. GitHub Actions workflow
- Trigger on push to main
- Path filters for `client/**` and `blog/**`
- Build both sites
- Run your deploy tool
- Securely handle secrets

---

## Resources to Study

| Topic | Resource |
|-------|----------|
| IPFS CIDs | https://docs.ipfs.tech/concepts/content-addressing/ |
| ENS Contenthash | EIP-1577 specification |
| ENS Namehash | EIP-137 specification |
| Pinata API | https://docs.pinata.cloud/api-reference |
| alloy (Rust Ethereum) | https://alloy.rs |
| ENS Resolver | https://docs.ens.domains/resolvers |

---

## Suggested Order

1. Get Pinata API working (upload a test folder manually first via curl)
2. Write Rust code to replicate that upload
3. Manually set contenthash via ENS app UI to verify your CID works
4. Write Rust code to automate the ENS update
5. Combine into CLI
6. Add GitHub Action

This incremental approach lets you verify each piece works before moving on.

---

## Rust Crates to Explore

- `alloy` - Ethereum interactions (provider, signer, contracts)
- `reqwest` - HTTP client for Pinata API
- `cid` - IPFS CID parsing and encoding
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `anyhow` - Error handling
