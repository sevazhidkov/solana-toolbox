import {
  AccountInfo,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import { ToolboxIdlProgram } from './ToolboxIdlProgram';
import { ToolboxEndpoint } from './ToolboxEndpoint';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';

export class ToolboxIdlService {
  private cachedPrograms: Map<PublicKey, ToolboxIdlProgram | null>;

  constructor() {
    this.cachedPrograms = new Map<PublicKey, ToolboxIdlProgram | null>();
  }

  public setProgram(
    programId: PublicKey,
    idlProgram: ToolboxIdlProgram | null,
  ) {
    this.cachedPrograms.set(programId, idlProgram);
  }

  public async getOrResolveProgram(
    endpoint: ToolboxEndpoint,
    programId: PublicKey,
  ): Promise<ToolboxIdlProgram | null> {
    let cachedProgram = this.cachedPrograms.get(programId);
    if (cachedProgram !== undefined) {
      return cachedProgram;
    }
    let resolvedProgram = await ToolboxIdlService.resolveProgram(
      endpoint,
      programId,
    );
    this.cachedPrograms.set(programId, resolvedProgram);
    return resolvedProgram;
  }

  static async resolveProgram(
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
      (await this.getOrResolveProgram(endpoint, account.owner)) ??
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

  public async decodeInstruction(
    endpoint: ToolboxEndpoint,
    instruction: TransactionInstruction,
  ) {
    let idlProgram =
      (await this.getOrResolveProgram(endpoint, instruction.programId)) ??
      ToolboxIdlProgram.Unknown;
    let idlInstruction =
      idlProgram.guessInstruction(instruction.data) ??
      ToolboxIdlInstruction.Unknown;
    let { instructionProgramId, instructionAddresses, instructionPayload } =
      idlInstruction.decode(instruction);
    return {
      program: idlProgram,
      instruction: idlInstruction,
      instructionProgramId,
      instructionAddresses,
      instructionPayload,
    };
  }

  // TODO - support finding
}
