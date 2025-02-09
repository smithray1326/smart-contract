import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Metaplex, keypairIdentity } from '@metaplex-foundation/js';
import { PROGRAM_ID as BUBBLEGUM_PROGRAM_ID } from '@metaplex-foundation/mpl-bubblegum';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import {
  ConfirmOptions,
  Connection,
  Keypair,
  PublicKey,
  TransactionSignature,
  clusterApiUrl,
  sendAndConfirmTransaction,
  Transaction,
} from '@solana/web3.js';

import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from '@solana/spl-account-compression';
import { MintNftSkytrade } from '../target/types/mint_nft_skytrade';

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const wallet = provider.wallet as anchor.Wallet;
const program = anchor.workspace.MintNftSkytrade as Program<MintNftSkytrade>;

// Retry parameters
const maxRetries = 5; // Maximum number of retries
const initialRetryDelayMs = 1000; // Initial delay before first retry in milliseconds
const retryBackoffFactor = 2; // Backoff factor for exponential backoff
const maxRetryDelayMs = 5000; // Maximum delay between retries in milliseconds

async function sendTransactionWithRetry(
  tx: any
): Promise<TransactionSignature> {
  let retries = 0;
  let retryDelayMs = initialRetryDelayMs;

  while (true) {
    try {
      // Send transaction to the Solana cluster and confirm its execution
      const options: ConfirmOptions = {
        commitment: 'confirmed',
        preflightCommitment: 'confirmed',
      };
      const txSignature = await sendAndConfirmTransaction(
        connection,
        tx,
        [wallet.payer],
        options
      );
      return txSignature;
    } catch (error) {
      console.error(
        `Error sending transaction (attempt ${retries + 1}):`,
        error
      );
      if (retries >= maxRetries) {
        console.error('Max retries exceeded, aborting.');
        throw error; // Max retries exceeded, propagate the error further
      }
      // Retry with exponential backoff
      retries++;
      await new Promise((resolve) => setTimeout(resolve, retryDelayMs));
      retryDelayMs = Math.min(
        retryDelayMs * retryBackoffFactor,
        maxRetryDelayMs
      );
    }
  }
}

async function mintNftToCollection(): Promise<void> {
  try {
    const metaplex = Metaplex.make(connection).use(
      keypairIdentity(wallet.payer)
    );

    const merkleTree = Keypair.generate();

    const [treeAuthority] = PublicKey.findProgramAddressSync(
      [merkleTree.publicKey.toBuffer()],
      BUBBLEGUM_PROGRAM_ID
    );

    // pda "tree creator", allows our program to update the tree
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('AUTH')],
      program.programId
    );

    const [bubblegumSigner] = PublicKey.findProgramAddressSync(
      [Buffer.from('collection_cpi', 'utf8')],
      BUBBLEGUM_PROGRAM_ID
    );

    // Create collection metadata
    const metadata = {
      uri: 'https://arweave.net/h19GMcMz7RLDY7kAHGWeWolHTmO83mLLMNPzEkF32BQ',
      name: 'SKY-TRADE',
      symbol: 'SKY',
    };

    // Create collection NFT
    const collectionNft = await metaplex.nfts().create({
      uri: metadata.uri,
      name: metadata.name,
      symbol: metadata.symbol,
      sellerFeeBasisPoints: 0,
      isCollection: true,
    });

    // Transfer collection NFT metadata update authority to PDA
    await metaplex.nfts().update({
      nftOrSft: collectionNft.nft,
      updateAuthority: wallet.payer,
      newUpdateAuthority: pda,
    });

    // Execute the methods builder to get the instruction and accounts
    const instruction = await program.methods
      .mintNft()
      .accounts({
        pda: pda,
        merkleTree: merkleTree.publicKey,
        treeAuthority: treeAuthority,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        bubblegumSigner: bubblegumSigner,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,

        collectionMint: collectionNft.mintAddress,
        collectionMetadata: collectionNft.metadataAddress,
        editionAccount: collectionNft.masterEditionAddress,
      })
      .instruction();

    // Create a new transaction
    const tx = new Transaction();

    tx.add(instruction);

    const txSignature = await sendTransactionWithRetry(tx);
    console.log('Transaction executed successfully:', txSignature);
  } catch (error) {
    console.error('An error occurred:', error);
  }
}

mintNftToCollection();

export { mintNftToCollection };
