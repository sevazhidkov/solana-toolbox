import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from './ToolboxIdlProgram';
import { ToolboxEndpoint } from './ToolboxEndpoint';

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

  public async loadProgram(
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
}
