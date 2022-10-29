# Interaction

Basics of interacting with smart contracts via CLI - [Cosmos-Notebook](https://github.com/LeTurt333/cosmos-notebook/blob/main/cheat-sheet/cli.md)

---

**Prep**
- Edit the `cw721-neon-peepz`, `cw721-shitty-kittyz`, and `cw20-base` contracts to your liking

---

**Step 1**
- Compile & Upload `hub`, `cw721-your-update`, `cw721-your-update-2`, `cw20-base` contracts

---

**Step 2**
- Instantiate `hub` contract

---

**Step 3** 
- Execute the `hub` contract, these will instantiate your cw721/cw20 contracts & store them as faucet-addresses     
```rust
InitFaucetNeonPeepz{code_id: u64} // code_id of a cw721 upload
// &
InitFaucetShittyKittyz{code_id: u64} // code_id of a cw721 upload
// &
InitFaucetCw20One{code_id: u64} // code_id of cw20-base
// &
InitFaucetCw20Two{code_id: u64} // code_id of cw20-base
// &
InitFaucetCw20Tre{code_id: u64} // code_id of cw20-base
```

---

**Step 4**
- Execute the `hub` contract, non-admin can only call these if also sending 5 junox with execute call
- - junox will be owned by contract
```rust
HitFaucetNft{} // pay 5junox to mint 2 NFTs

HitFaucetCw20s{} // pay 5junox to mint 69 of each cw20 token
```

