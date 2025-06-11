import {
  PublicKey,
  TransactionError,
  TransactionInstruction,
} from '@solana/web3.js';

export class ToolboxEndpointExecution {
  public readonly processedTime: Date | null;
  public readonly slot: number;
  public readonly payer: PublicKey;
  public readonly instructions: TransactionInstruction[];
  public readonly logs: string[] | null;
  public readonly error: TransactionError | null;
  public readonly unitsConsumed: number | null;

  constructor(value: {
    processedTime: Date | null;
    slot: number;
    payer: PublicKey;
    instructions: TransactionInstruction[];
    logs: string[] | null;
    error: TransactionError | null;
    unitsConsumed: number | null;
  }) {
    this.processedTime = value.processedTime;
    this.slot = value.slot;
    this.payer = value.payer;
    this.instructions = value.instructions;
    this.logs = value.logs;
    this.error = value.error;
    this.unitsConsumed = value.unitsConsumed;
  }
}
