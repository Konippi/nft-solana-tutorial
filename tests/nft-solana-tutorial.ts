import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftSolanaTutorial } from "../target/types/nft_solana_tutorial";
import { createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { getMasterEdition, getMetadata } from "./utils";

describe("nft-solana-tutorial", async () => {
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet;
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftSolanaTutorial as Program<NftSolanaTutorial>;

  it("Is initialized!", async () => {
    const tokenMetadataProgramId = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
    const mintKey = anchor.web3.Keypair.generate();
    const nftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      wallet.publicKey,
    );
    console.log("NFT Account: ", nftTokenAccount.toBase58());

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey,
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        nftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey,
      ),
    );

    const account = await provider.sendAndConfirm(mint_tx, [mintKey]);
    console.log("Account: ", account);
    console.log("Mint Key: ", mintKey.publicKey.toString());
    console.log("User: ", wallet.publicKey.toString());

    const metadata = getMetadata(tokenMetadataProgramId, mintKey.publicKey);
    const masterEdition = getMasterEdition(tokenMetadataProgramId, mintKey.publicKey);
    console.log("Metadata Account: ", metadata.toBase58());
    console.log("Master Edition: ", masterEdition.toBase58());

    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
      "Konippi/nft-solana-tutorial",
    ).accounts({
      mintAuthority: wallet.publicKey,
      mintAccount: mintKey.publicKey,
      tokenAccount: nftTokenAccount,
      tokenMetadataProgram: tokenMetadataProgramId,
      metadata,
      payer: wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      masterEdition,
    }).rpc();
    console.log("Transaction Signature: ", tx);
  });
});
