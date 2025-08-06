import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";

// This is a dummy keypair. In a real application, you would use a secure way to handle keys.
const payer = Keypair.generate();
const mintAuthority = Keypair.generate();
const freezeAuthority = Keypair.generate();

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function createToken() {
  const lamports = await getMinimumBalanceForRentExemptMint(connection);

  const mint = Keypair.generate();

  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: MINT_SIZE,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mint.publicKey,
      2, // 2 decimal places
      mintAuthority.publicKey,
      freezeAuthority.publicKey,
      TOKEN_PROGRAM_ID
    )
  );

  console.log("Token created with public key:", mint.publicKey.toBase58());
  console.log("Next steps: run this transaction to create the token on-chain.");
  console.log("You will need to fund the 'payer' account with some SOL first.");
}

createToken();
