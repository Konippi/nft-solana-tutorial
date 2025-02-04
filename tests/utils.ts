import * as anchor from "@coral-xyz/anchor";

/**
 * Get the metadata of the NFT
 * 
 * @param {anchor.web3.PublicKey} programId The program ID of the NFT metadata program
 * @param {anchor.web3.PublicKey} mint The mint of the NFT
 * @returns {anchor.web3.PublicKey} The metadata of the NFT
 */
export function getMetadata(
    programId: anchor.web3.PublicKey, 
    mint: anchor.web3.PublicKey
): anchor.web3.PublicKey {
  return (
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"), programId.toBuffer(), mint.toBuffer()],
        programId,
    )
  )[0];
}

/**
 * Get the master edition of the NFT
 * 
 * @param {anchor.web3.PublicKey} programId The program ID of the NFT metadata program
 * @param {anchor.web3.PublicKey} mint The mint of the NFT 
 * @returns {anchor.web3.PublicKey} The master edition of the NFT
 */
export function getMasterEdition(
    programId: anchor.web3.PublicKey, 
    mint: anchor.web3.PublicKey
): anchor.web3.PublicKey {
  return (
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"), programId.toBuffer(), mint.toBuffer(), Buffer.from("edition")],
        programId,
    )
  )[0];
}