import * as anchor from "@project-serum/anchor";
import { NftMintingDapp } from "../target/types/nft_minting_dapp";

describe("nft-minting-dapp", () => {
  // Declare the constant values
  const TEST_NFT_TITLE = "Collection"
  const TEST_NFT_SYMBOL = "CLCTN"
  const TEST_NFT_URI = "https://berzan.xyz"

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  const wallet = provider.wallet as anchor.Wallet
  anchor.setProvider(provider);
  const program = anchor.workspace.NftMintingDapp as anchor.Program<NftMintingDapp>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")


  it("Can call mint method!", async () => {
    // Derive the mint address and the associated token account address
    const mintKeypair = anchor.web3.Keypair.generate()

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey,
    })

    console.log('New token:', mintKeypair.publicKey.toBase58())

    // Derive the metadata and master edition addresses
    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    ))[0]

    console.log('Metadata initialized')

    const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    ))[0]

    console.log('Master Edition Metadata initialized')

    
    // Transact with the "mint" function in our program
    const tx = await program.methods.mint(TEST_NFT_TITLE, TEST_NFT_SYMBOL, TEST_NFT_URI)
    .accounts({
      masterEdition: masterEditionAddress,
      metadata: metadataAddress,
      mint: mintKeypair.publicKey, 
      tokenAccount: tokenAddress,
      mintAuthority: wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([mintKeypair])
    .rpc()
    console.log('Minting is succesful')
    console.log('Mint:', mintKeypair.publicKey.toBase58())
    console.log('Mint Authority:', wallet.publicKey.toBase58())
    
    
    
  });
});
