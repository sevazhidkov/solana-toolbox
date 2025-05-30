import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlError } from './ToolboxIdlError';
import { inflate } from 'pako';
import { ToolboxIdlEvent } from './ToolboxIdlEvent';

export type ToolboxIdlProgramMetadata = {
  name?: string;
  docs?: any;
  description?: string;
  address?: PublicKey;
  version?: string;
  spec?: string;
};

export class ToolboxIdlProgram {
  public static readonly DISCRIMINATOR = Buffer.from([
    0x18, 0x46, 0x62, 0xbf, 0x3a, 0x90, 0x7b, 0x9e,
  ]);

  public static readonly Unknown = new ToolboxIdlProgram({
    metadata: {},
    typedefs: new Map(),
    accounts: new Map(),
    instructions: new Map(),
    events: new Map(),
    errors: new Map(),
  });

  public metadata: ToolboxIdlProgramMetadata;
  public typedefs: Map<string, ToolboxIdlTypedef>;
  public accounts: Map<string, ToolboxIdlAccount>;
  public instructions: Map<string, ToolboxIdlInstruction>;
  public events: Map<string, ToolboxIdlEvent>;
  public errors: Map<string, ToolboxIdlError>;

  constructor(value: {
    metadata: ToolboxIdlProgramMetadata;
    typedefs: Map<string, ToolboxIdlTypedef>;
    accounts: Map<string, ToolboxIdlAccount>;
    instructions: Map<string, ToolboxIdlInstruction>;
    events: Map<string, ToolboxIdlEvent>;
    errors: Map<string, ToolboxIdlError>;
  }) {
    this.metadata = value.metadata;
    this.typedefs = value.typedefs;
    this.accounts = value.accounts;
    this.instructions = value.instructions;
    this.events = value.events;
    this.errors = value.errors;
  }

  public static async findAnchorAddress(
    programId: PublicKey,
  ): Promise<PublicKey> {
    let base = PublicKey.findProgramAddressSync([], programId)[0];
    return await PublicKey.createWithSeed(base, 'anchor:idl', programId);
  }

  public static tryParseFromAccountData(
    accountData: Buffer,
  ): ToolboxIdlProgram {
    let discriminator = accountData.subarray(0, 8);
    if (!discriminator.equals(ToolboxIdlProgram.DISCRIMINATOR)) {
      throw new Error('Invalid IDL program discriminator');
    }
    let contentLength = accountData.readUInt32LE(40);
    let contentRaw = accountData.subarray(44, 44 + contentLength);
    let contentEncoded = inflate(contentRaw);
    let contentDecoded = new TextDecoder('utf8').decode(contentEncoded);
    return ToolboxIdlProgram.tryParseFromString(contentDecoded);
  }

  public static tryParseFromString(idlString: string): ToolboxIdlProgram {
    let idlRoot = JSON.parse(idlString);
    return ToolboxIdlProgram.tryParse(idlRoot);
  }

  public static tryParse(idlRoot: any): ToolboxIdlProgram {
    let metadata = {
      ...ToolboxIdlProgram.tryParseMetadata(idlRoot),
      ...ToolboxIdlProgram.tryParseMetadata(idlRoot['metadata']),
    };
    let typedefs = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'types',
      false,
      undefined,
      undefined,
      ToolboxIdlTypedef.tryParse,
    );
    let accounts = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'accounts',
      false,
      typedefs,
      undefined,
      ToolboxIdlAccount.tryParse,
    );
    let instructions = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'instructions',
      true,
      typedefs,
      accounts,
      ToolboxIdlInstruction.tryParse,
    );
    let events = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'events',
      false,
      typedefs,
      undefined,
      ToolboxIdlEvent.tryParse,
    );
    let errors = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'errors',
      false,
      undefined,
      undefined,
      ToolboxIdlError.tryParse,
    );
    return new ToolboxIdlProgram({
      metadata,
      typedefs,
      accounts,
      instructions,
      events,
      errors,
    });
  }

  static tryParseMetadata(idlMetadata: any): ToolboxIdlProgramMetadata {
    if (!idlMetadata) {
      return {};
    }
    let rawName = idlMetadata['name'];
    let rawDocs = idlMetadata['docs'];
    let rawDescription = idlMetadata['description'];
    let rawAddress = idlMetadata['address'];
    let rawVersion = idlMetadata['version'];
    let rawSpec = idlMetadata['spec'];
    return {
      name: rawName ? ToolboxUtils.expectString(rawName) : undefined,
      docs: rawDocs,
      description: rawDescription
        ? ToolboxUtils.expectString(rawDescription)
        : undefined,
      address: rawAddress
        ? new PublicKey(ToolboxUtils.expectString(rawAddress))
        : undefined,
      version: rawVersion ? ToolboxUtils.expectString(rawVersion) : undefined,
      spec: rawSpec ? ToolboxUtils.expectString(rawSpec) : undefined,
    };
  }

  static tryParseScopedNamedValues<T, P1, P2>(
    idlRoot: any,
    collectionKey: string,
    nameToSnakeCase: boolean,
    param1: P1,
    param2: P2,
    parsingFunction: (name: string, value: any, param1: P1, param2: P2) => T,
  ): Map<string, T> {
    let values = new Map();
    let collection = idlRoot[collectionKey];
    if (ToolboxUtils.isArray(collection)) {
      for (let item of collection) {
        let name = ToolboxUtils.expectString(item['name']);
        if (nameToSnakeCase) {
          name = ToolboxUtils.convertToSnakeCase(name);
        }
        values.set(name, parsingFunction(name, item, param1, param2));
      }
    }
    if (ToolboxUtils.isObject(collection)) {
      Object.entries(collection).forEach(([key, value]) => {
        if (nameToSnakeCase) {
          key = ToolboxUtils.convertToSnakeCase(key);
        }
        values.set(key, parsingFunction(key, value, param1, param2));
      });
    }
    return values;
  }

  public guessAccount(accountData: Buffer): ToolboxIdlAccount | null {
    for (let account of this.accounts.values()) {
      try {
        account.check(accountData);
        return account;
      } catch {}
    }
    return null;
  }

  public guessInstruction(
    instructionData: Buffer,
  ): ToolboxIdlInstruction | null {
    for (let instruction of this.instructions.values()) {
      try {
        instruction.check(instructionData);
        return instruction;
      } catch {}
    }
    return null;
  }

  public guessEvent(eventData: Buffer): ToolboxIdlEvent | null {
    for (let event of this.events.values()) {
      try {
        event.check(eventData);
        return event;
      } catch {}
    }
    return null;
  }

  public guessError(errorCode: number): ToolboxIdlError | null {
    for (let error of this.errors.values()) {
      if (error.code === errorCode) {
        return error;
      }
    }
    return null;
  }
}
