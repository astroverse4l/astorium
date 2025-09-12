import { Connection, Keypair } from "@solana/web3.js";
import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";
import * as fs from 'fs';

// This is a dummy keypair. In a real application, you would use a secure way to handle keys.
const payer = Keypair.generate();

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(payer))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: 'https://api.devnet.solana.com',
        timeout: 60000,
    }));

async function createTokenWithMetadata() {
    console.log("Creating token with metadata...");

    const metadata = JSON.parse(fs.readFileSync("./metadata.json", "utf-8"));

    // In a real application, you would upload the metadata to a permanent storage like Arweave.
    // For this example, we will use the placeholder URI from the metadata file.
    const { uri } = await metaplex.nfts().uploadMetadata(metadata);

    const { nft: token } = await metaplex.nfts().create({
        uri: uri,
        name: metadata.name,
        symbol: metadata.symbol,
        sellerFeeBasisPoints: 500, // 5%
        isCollection: false,
    });

    console.log("Token created successfully!");
    console.log("Token address:", token.address.toBase58());
    console.log("Metadata address:", token.metadataAddress.toBase58());
    console.log("You will need to fund the 'payer' account with some SOL first.");
}

createTokenWithMetadata();
