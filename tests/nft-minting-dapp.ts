import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMintingDapp } from "../target/types/nft_minting_dapp";

describe("nft-minting-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMintingDapp as Program<NftMintingDapp>;

  it("Can call mint method!", async () => {
    // Add your test here.
    const tx = await program.methods.mint().rpc();
    console.log("Your transaction signature", tx);
  });
});
