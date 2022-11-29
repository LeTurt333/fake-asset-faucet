# Fake Asset Faucet

A simple 3 contract structure

- Hub: Accepts payment in ujunox, mints cw721 or cw20 which are owned by caller
- cw721-_: NFT contracts with metadata extension from [cw721-metadata-onchain](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain)


### Hub
- Instantiates cw721 & cw20 contracts with itself as `Minter`
- Allows `HitFaucet_` calls from contract admin, or any arbitrary address if they also send in 5junox with execute call
- junox stored in contract permanently, you'll need to add logic if you want to withdraw it 
- Caller of `HitFaucet_` is the owner of the newly minted cw20/cw721
- When minting NFTs, the Hub contract calls the [Nois](https://github.com/noislabs) proxy contract and uses the returned randomness to mint NFTs

### cw721-___
- NFT contracts with metadata extension