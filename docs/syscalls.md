# ordinal.cash

TLDR: Solana programs are limited to 200k instruction rendering ZK-SNARKs impossible.

## Problem

We are currently building ordinal.cash which is a ZK-SNARK token mixer running on Solana.

Currently, privacy solutions such as tornado.cash are prohibitively expensive due to high gas cost. Mixing 0.1 ETH, the minimum anonymity set on tornado.cash, costs 0.05 ETH in gas, despite having [precompiled contracts](https://ethereum.stackexchange.com/questions/15479/list-of-pre-compiled-contracts) for ZK-SNARKS.

Solana as the fastest blockchain should have native support for common ZK-SNARKs functions.

## How it works: (simplified version)

1. Users send equal amount of tokens to a central anonymity pool along with a commitment
2. Using the commitment and other private inputs, generate a ZK-SNARK proof
3. Using a separate account, or through a relayer, call contract with the generated proof
4. On-chain verifier can then verify the proof and transfer

## How it works: (extended version)

The onchain program maintains a Merkle tree to prevent double spend. The same tree is used for proof generation. So the hash function must be both cheap to compute on chain and not too slow for proof generation offchain:

hash     | gas   | # of constraints
---------|-------|------------------
SHA256   | 60    | 28k
MiMC     | 8.9k  | 646
Poseidon | 58.4k | 317

This table shows the gas cost and number of constraints which scales exponentially for tree levels. SHA256 is prohibitively expensive for proof while [MiMC is the sweet spot](https://eprint.iacr.org/2020/156.pdf). This is also the hash function chosen by most Ethereum ZK-SNARK smart contracts such as tornado.cash.

## The need for ZK-SNARKS syscalls

The most common implementation of MiMCSponge hasher is written in [EVM bytecode](https://github.com/iden3/circomlib/blob/master/src/mimcsponge_gencontract.js). Here n=220 due to speed-security tradeoff.

In order to guesstimate how many BPF instructions needed for each MiMCSponge call, I used [solenoid](https://github.com/0b01/solenoid/blob/master/examples/mimcsponge.rs) to compile EVM assembly to BPF assembly via LLVM. The optimized IR(using i256) contains ~27,000 instructions while The BPF assembly contains ~160,000 instructions. So for a 20-level Merkle Tree, adding a node costs 20*160,000 = 3,200,000 instructions while initializing the tree takes 2^20 * 160,000 = 167,772,160,000 instructions. So it is unrealistic under the current instruction count limit regardless of potential adjustments to the limit.

Instruction counts are really high mainly due to the fact that these functions operate on 256 bit numbers while BPF is a 32 bit virtual machine. By lifting them out of the VM into the runtime, it can also boost performance.

Similar arguments can be made for bn curve related functions. Here is a list of syscalls that need to be implemented in rbpf runtime to enable common ZK-SNARKS scenarios:

1. MiMCSponge
2. Addition on elliptic curve alt_bn128 (EIP 196)
3. Scalar multiplication on elliptic curve alt_bn128 (EIP 196)
4. Checking a pairing equation on curve alt_bn128 (EIP 197)

I can provide Rust implementation for those if needed.
