import {
  AccountInfo,
  Blockhash,
  Connection,
  PublicKey,
  TransactionSignature,
  VersionedTransaction,
} from '@solana/web3.js';
import { ToolboxEndpointExecution } from './ToolboxEndpointExecution';

export class ToolboxEndpoint {
  public static readonly PUBLIC_RPC_URL_MAINNET_BETA =
    'https://api.mainnet-beta.solana.com';
  public static readonly PUBLIC_RPC_URL_TESTNET =
    'https://api.testnet.solana.com';
  public static readonly PUBLIC_RPC_URL_DEVNET =
    'https://api.devnet.solana.com';

  private static urlOrMonikerToUrl = new Map<string, string>([
    ['m', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['mainnet', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['mainnet-beta', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['t', ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
    ['testnet', ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
    ['d', ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET],
    ['devnet', ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET],
  ]);

  private connection: Connection;
  private commitment: 'finalized' | 'confirmed';

  public constructor(
    urlOrMoniker: string,
    commitment: 'finalized' | 'confirmed',
  ) {
    this.connection = new Connection(
      ToolboxEndpoint.getUrlFromUrlOrMoniker(urlOrMoniker),
      commitment,
    );
    this.commitment = commitment;
  }

  public static getUrlFromUrlOrMoniker(urlOrMoniker: string): string {
    return (
      ToolboxEndpoint.urlOrMonikerToUrl.get(urlOrMoniker.toLowerCase()) ??
      urlOrMoniker
    );
  }

  public async getLatestBlockhash(): Promise<Blockhash> {
    return (await this.connection.getLatestBlockhash()).blockhash;
  }

  public async getBalance(address: PublicKey): Promise<number> {
    return await this.connection.getBalance(address);
  }

  public async getAccount(
    address: PublicKey,
  ): Promise<AccountInfo<Buffer> | null> {
    return await this.connection.getAccountInfo(address);
  }

  public async simulateTransaction(
    versionedTransaction: VersionedTransaction,
    verifySignatures: boolean,
  ): Promise<ToolboxEndpointExecution> {
    // TODO - resolved lookup tables
    let response = await this.connection.simulateTransaction(
      versionedTransaction,
      {
        sigVerify: verifySignatures,
        replaceRecentBlockhash: false,
        commitment: this.commitment,
        accounts: undefined,
      },
    );
    console.log('simulateTransaction.response', response);
    // TODO - convert to executution
    return new ToolboxEndpointExecution();
  }

  public async processTransaction(
    versionedTransaction: VersionedTransaction,
    verifyPreflight: boolean,
  ): Promise<[TransactionSignature, ToolboxEndpointExecution]> {
    let signature = await this.connection.sendTransaction(
      versionedTransaction,
      {
        skipPreflight: !verifyPreflight,
        preflightCommitment: this.commitment,
      },
    );
    console.log('processTransaction.signature', signature);
    return [signature, new ToolboxEndpointExecution()];
  }

  public async getExecution(
    signature: TransactionSignature,
  ): Promise<ToolboxEndpointExecution> {
    let response = await this.connection.getTransaction(signature, {
      commitment: this.commitment,
      maxSupportedTransactionVersion: 0,
    });
    console.log('getExecution.response', response);
    return new ToolboxEndpointExecution();
  }

  public async searchAddresses(
    programId: PublicKey,
    dataLength: number,
    dataChunks: [number, Buffer][],
  ): Promise<Set<PublicKey>> {
    let response = await this.connection.getProgramAccounts(programId, {
      commitment: this.commitment,
      dataSlice: {
        offset: 0,
        length: 0,
      },
      filters: [
        {
          dataSize: dataLength,
        },
        // TODO - data chunk memcpy filters
      ],
    });
    console.log('searchAddresses.response', response);
    let addresses = new Set<PublicKey>();
    for (let finding of response) {
      addresses.add(finding.pubkey);
    }
    return addresses;
  }

  public async searchSignatures(
    address: PublicKey,
    limit: number,
    startBefore?: TransactionSignature,
    rewindUntil?: TransactionSignature,
  ): Promise<TransactionSignature[]> {
    let oldestKnownSignature = startBefore;
    let orderedSignatures = [];
    let retries = 0;
    while (true) {
      let batchSize = Math.min(1000, retries == 0 ? 10 : 1000);
      retries++;
      let signatures = await this.connection.getSignaturesForAddress(
        address,
        {
          before: oldestKnownSignature,
          limit: batchSize,
        },
        this.commitment,
      );
      if (signatures.length == 0) {
        return orderedSignatures;
      }
      for (let signature of signatures) {
        let foundSignature = signature.signature;
        orderedSignatures.push(foundSignature);
        if (orderedSignatures.length >= limit) {
          return orderedSignatures;
        }
        if (rewindUntil && foundSignature == rewindUntil) {
          return orderedSignatures;
        }
        oldestKnownSignature = foundSignature;
      }
    }
  }
}
