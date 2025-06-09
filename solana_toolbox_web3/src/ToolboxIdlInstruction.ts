import {
  AccountMeta,
  PublicKey,
  TransactionInstruction,
} from '@solana/web3.js';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstructionAccount } from './ToolboxIdlInstructionAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from './ToolboxIdlTypeFlat';
import { hydrate, hydrateFields } from './ToolboxIdlTypeFlat.hydrate';
import { parse, parseFields } from './ToolboxIdlTypeFlat.parse';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from './ToolboxIdlTypeFull';
import { deserializeFields } from './ToolboxIdlTypeFull.deserialize';
import { serializeFields } from './ToolboxIdlTypeFull.serialize';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlInstruction {
  public readonly name: string;
  public readonly docs: any;
  public readonly discriminator: Buffer;
  public readonly accounts: ToolboxIdlInstructionAccount[];
  public readonly argsTypeFlatFields: ToolboxIdlTypeFlatFields;
  public readonly argsTypeFullFields: ToolboxIdlTypeFullFields;
  public readonly returnTypeFlat: ToolboxIdlTypeFlat;
  public readonly returnTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    docs: any;
    discriminator: Buffer;
    accounts: ToolboxIdlInstructionAccount[];
    argsTypeFlatFields: ToolboxIdlTypeFlatFields;
    argsTypeFullFields: ToolboxIdlTypeFullFields;
    returnTypeFlat: ToolboxIdlTypeFlat;
    returnTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.accounts = value.accounts;
    this.argsTypeFlatFields = value.argsTypeFlatFields;
    this.argsTypeFullFields = value.argsTypeFullFields;
    this.returnTypeFlat = value.returnTypeFlat;
    this.returnTypeFull = value.returnTypeFull;
  }

  public static tryParse(
    idlInstructionName: string,
    idlInstruction: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
    accounts: Map<string, ToolboxIdlAccount>,
  ): ToolboxIdlInstruction {
    let docs = idlInstruction['docs'];
    let discriminator = Buffer.from(
      idlInstruction['discriminator'] ??
        ToolboxUtils.discriminator(`global:${idlInstructionName}`),
    );
    let idlInstructionAccounts = ToolboxUtils.expectArray(
      idlInstruction['accounts'] ?? [],
    );
    let instructionAccounts = idlInstructionAccounts.map(
      (idlInstructionAccount: any) => {
        return ToolboxIdlInstructionAccount.tryParse(
          idlInstructionAccount,
          typedefs,
          accounts,
        );
      },
    );
    let argsTypeFlatFields = parseFields(idlInstruction['args'] ?? []);
    let argsTypeFullFields = hydrateFields(
      argsTypeFlatFields,
      new Map(),
      typedefs,
    );
    let returnTypeFlat = parse(idlInstruction['returns'] ?? { fields: [] });
    let returnTypeFull = hydrate(returnTypeFlat, new Map(), typedefs);
    return new ToolboxIdlInstruction({
      name: idlInstructionName,
      docs,
      discriminator,
      accounts: instructionAccounts,
      argsTypeFlatFields,
      argsTypeFullFields,
      returnTypeFlat,
      returnTypeFull,
    });
  }

  public checkPayload(instructionData: Buffer) {
    if (instructionData.length < this.discriminator.length) {
      throw new Error('Invalid discriminator');
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (instructionData[i] !== this.discriminator[i]) {
        throw new Error('Invalid discriminator');
      }
    }
  }

  public encode(
    instructionProgramId: PublicKey,
    instructionPayload: any,
    instructionAddresses: Map<string, PublicKey>,
  ): TransactionInstruction {
    let instructionMetas = this.encodeAddresses(instructionAddresses);
    let instructionData = this.encodePayload(instructionPayload);
    return {
      programId: instructionProgramId,
      keys: instructionMetas,
      data: instructionData,
    };
  }

  public decode(instruction: TransactionInstruction): {
    instructionProgramId: PublicKey;
    instructionPayload: any;
    instructionAddresses: Map<string, PublicKey>;
  } {
    this.checkPayload(instruction.data);
    let instructionAddresses = this.decodeAddresses(instruction.keys);
    let instructionPayload = this.decodePayload(instruction.data);
    return {
      instructionProgramId: instruction.programId,
      instructionAddresses,
      instructionPayload,
    };
  }

  public encodeAddresses(
    instructionAddresses: Map<string, PublicKey>,
  ): AccountMeta[] {
    let instructionMetas = [];
    for (let account of this.accounts) {
      if (account.optional && !instructionAddresses.has(account.name)) {
        continue;
      }
      let instructionAddress = instructionAddresses.get(account.name);
      if (!instructionAddress) {
        throw new Error(`Missing address for account: ${account.name}`);
      }
      instructionMetas.push({
        pubkey: instructionAddress,
        isSigner: account.signer,
        isWritable: account.writable,
      });
    }
    return instructionMetas;
  }

  public decodeAddresses(
    instructionMetas: AccountMeta[],
  ): Map<string, PublicKey> {
    let instructionOptionalsPossible = 0;
    for (let account of this.accounts) {
      if (account.optional) {
        instructionOptionalsPossible++;
      }
    }
    let instructionOptionalsUnuseds =
      this.accounts.length - instructionMetas.length;
    let instructionOptionalsUsed =
      instructionOptionalsPossible - instructionOptionalsUnuseds;
    let instructionAddresses = new Map<string, PublicKey>();
    let instructionMetaIndex = 0;
    let instructionOptionalsCurrent = 0;
    for (let account of this.accounts) {
      if (account.optional) {
        instructionOptionalsCurrent += 1;
        if (instructionOptionalsCurrent > instructionOptionalsUsed) {
          continue;
        }
      }
      if (instructionMetaIndex >= instructionMetas.length) {
        break;
      }
      instructionAddresses.set(
        account.name,
        instructionMetas[instructionMetaIndex].pubkey,
      );
      instructionMetaIndex++;
    }
    return instructionAddresses;
  }

  public encodePayload(instructionPayload: any): Buffer {
    let data: Buffer[] = [];
    data.push(this.discriminator);
    serializeFields(this.argsTypeFullFields, instructionPayload, data, true);
    return Buffer.concat(data);
  }

  public decodePayload(instructionData: Buffer): any {
    this.checkPayload(instructionData);
    let [, instructionPayload] = deserializeFields(
      this.argsTypeFullFields,
      instructionData,
      this.discriminator.length,
    );
    return instructionPayload;
  }
}
