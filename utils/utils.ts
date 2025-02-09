import {
  findLeafAssetIdPda,
  getAssetWithProof,
} from '@metaplex-foundation/mpl-bubblegum';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { PublicKey } from '@metaplex-foundation/umi';
import dotenv from 'dotenv';
import { dasApi } from '@metaplex-foundation/digital-asset-standard-api';
import { PublicKey as PublicK } from '@solana/web3.js';
const bs58 = require('bs58').default;

dotenv.config();

export class AssetExtractor {
  // Method to extract the asset ID
  public async extractAssetId(leafIndex: number, merkleTree: PublicKey) {
    try {
      const umi = createUmi(
        `https://devnet.helius-rpc.com/?api-key=${process.env.RPC_URL}`
      );
      umi.use(dasApi());

      const [assetId, bump] = findLeafAssetIdPda(umi, {
        merkleTree,
        leafIndex,
      });

      const rpcAsset = await umi.rpc.getAsset(assetId);

      if (!rpcAsset) {
        throw new Error('Asset not found');
      }

      return { assetId, rpcAsset };
    } catch (error) {
      console.error('Error extracting asset ID:', error);
      throw new Error(
        `Failed to extract asset ID: ${
          error instanceof Error ? error.message : 'Unknown error'
        }`
      );
    }
  }

  public async getAssetProof(assetId: PublicKey) {
    const umi = createUmi(
      `https://devnet.helius-rpc.com/?api-key=${process.env.RPC_URL}`
    );
    umi.use(dasApi());

    const assetwithProof = getAssetWithProof(umi, assetId);
    return assetwithProof;
  }

  public mapProof(assetProof: { proof: string[] }) {
    if (!assetProof.proof || assetProof.proof.length === 0) {
      throw new Error('Proof is empty');
    }
    // Ensure proof is in correct order (from leaf to root)
    const orderedProof = [...assetProof.proof];

    return orderedProof.map((node) => ({
      pubkey: new PublicK(node),
      isSigner: false,
      isWritable: false,
    }));
  }
  public decode(stuff: string) {
    return this.bufferToArray(bs58.decode(stuff));
  }
  bufferToArray(buffer: Buffer): number[] {
    const nums: number[] = [];
    for (let i = 0; i < buffer.length; i++) {
      nums.push(buffer[i]);
    }
    return nums;
  }
}
