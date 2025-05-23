import { PublicKey } from '@solana/web3.js';

class ToolboxIdlService {
  private cachedPrograms: Map<PublicKey, ToolboxIdlProgram | null>;

  constructor() {
    this.cachedPrograms = new Map<PublicKey, ToolboxIdlProgram | null>();
  }
}
