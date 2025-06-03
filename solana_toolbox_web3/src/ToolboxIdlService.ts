import {
  AccountInfo,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import { ToolboxIdlProgram } from './ToolboxIdlProgram';
import { ToolboxEndpoint } from './ToolboxEndpoint';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';

export class ToolboxIdlService {
  private cachedPrograms: Map<PublicKey, ToolboxIdlProgram | null>;

  constructor() {
    this.cachedPrograms = new Map<PublicKey, ToolboxIdlProgram | null>();
  }

  public preloadProgram(
    programId: PublicKey,
    idlProgram: ToolboxIdlProgram | null,
  ) {
    this.cachedPrograms.set(programId, idlProgram);
  }

  public async resolveProgram(
    endpoint: ToolboxEndpoint,
    programId: PublicKey,
  ): Promise<ToolboxIdlProgram | null> {
    let cachedProgram = this.cachedPrograms.get(programId);
    if (cachedProgram !== undefined) {
      return cachedProgram;
    }
    let resolvedProgram = await ToolboxIdlService.loadProgram(
      endpoint,
      programId,
    );
    this.cachedPrograms.set(programId, resolvedProgram);
    return resolvedProgram;
  }

  static async loadProgram(
    endpoint: ToolboxEndpoint,
    programId: PublicKey,
  ): Promise<ToolboxIdlProgram | null> {
    // TODO - lib idls
    let account = await endpoint.getAccount(
      await ToolboxIdlProgram.findAnchorAddress(programId),
    );
    if (account == null) {
      return null;
    }
    return ToolboxIdlProgram.tryParseFromAccountData(account.data);
  }

  public async getAndDecodeAccount(
    endpoint: ToolboxEndpoint,
    address: PublicKey,
  ) {
    let account = (await endpoint.getAccount(address)) ?? {
      lamports: 0,
      owner: SystemProgram.programId,
      data: Buffer.from([]),
      executable: false,
    };
    return this.decodeAccount(endpoint, account);
  }

  public async decodeAccount(
    endpoint: ToolboxEndpoint,
    account: AccountInfo<Buffer>,
  ) {
    let idlProgram =
      (await this.resolveProgram(endpoint, account.owner)) ??
      ToolboxIdlProgram.Unknown;
    let idlAccount =
      idlProgram.guessAccount(account.data) ?? ToolboxIdlAccount.Unknown;
    let accountState = idlAccount.decode(account.data);
    return {
      lamports: account.lamports,
      owner: account.owner,
      program: idlProgram,
      account: idlAccount,
      state: accountState,
    };
  }
}
