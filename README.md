# CryptographyLab

A laboratory for manual implementation of cryptographic primitives. This repository focuses on **algorithmic reconstruction** rather than API consumption, emphasizing bit-level logic and mathematical invariants.

## Core Objectives
* **Manual Implementation**
* **Low-Level Logic**
* **Security Constraints**
* **Memory Safety**

## Tech Stack
* **Rust**

## Repository Structure
```text
├── hashing/      # SHA-256/512, BLAKE3, HMAC (no dependencies)
├── symmetric/    # AES (Rijndael core, GCM), ChaCha20
├── asymmetric/   # RSA (CRT, Miller-Rabin), ECC (P-256, Ed25519)
├── research/     # Side-channel analysis & math foundations
└── utils/        # PRNGs, entropy tests, and encoding tools
