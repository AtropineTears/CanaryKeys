# CanaryKeys: A New Extendable Blockchain For Digital Certificates, Web of Trust, and Social Media

> **Notice:** This is a work in progress and is not fully implemented yet. It is currently mostly an idea but some basic functionality has been implemented.

CanaryKeys is a blockchain project that is **extendable** and **can support multiple chains for different purposes** due to its use of the **Block Lattice Data Structure**.

## Current Implemented Chains

### The Transaction Chain

This is the primary chain and the one where all monetary transactions take place. It will also support tokens.

### The Voting Chain

This chain is used to vote for delegates to act as your representative to produce blocks. This may be merged into the transaction chain.

### The Certificate Chain (supports Web of Trust)

The Certificate Chain is used to sign other people's certificates and it naturally supports Web of Trust. It uses Schnorr Signatures.

### The Social Chain

The Social Chain contains verified and unverified links to your social media accounts.

### The Project Chain

The Project Chain contains any projects that will be built on top of CanaryKeys. It can also be used to raise funds.
