import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { inflate } from 'deflate-js';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlError } from './ToolboxIdlError';

export class ToolboxIdlProgram {
  public static readonly DISCRIMINATOR = Buffer.from([
    0x18, 0x46, 0x62, 0xbf, 0x3a, 0x90, 0x7b, 0x9e,
  ]);

  public typedefs: Map<string, ToolboxIdlTypedef>;
  public accounts: Map<string, ToolboxIdlAccount>;
  public instructions: Map<string, ToolboxIdlInstruction>;
  public errors: Map<string, ToolboxIdlError>;

  constructor(value: {
    typedefs: Map<string, ToolboxIdlTypedef>;
    accounts: Map<string, ToolboxIdlAccount>;
    instructions: Map<string, ToolboxIdlInstruction>;
    errors: Map<string, ToolboxIdlError>;
  }) {
    this.typedefs = value.typedefs;
    this.accounts = value.accounts;
    this.instructions = value.instructions;
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
    let length = accountData.readUInt32LE(40);
    console.log('length', length);
    let content = accountData.subarray(44, 44 + length);
    console.log('content', content);
    let contentEncoded = inflate(content);
    console.log('contentEncoded', contentEncoded);
    let contentDecoded = contentEncoded.toString();
    console.log('contentDecoded', contentDecoded);
    return ToolboxIdlProgram.tryParseFromString(contentDecoded);
  }

  public static tryParseFromString(idlString: string): ToolboxIdlProgram {
    return ToolboxIdlProgram.tryParse(JSON.stringify(idlString));
  }

  public static tryParse(idlRoot: any): ToolboxIdlProgram {
    let typedefs = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'types',
      undefined,
      undefined,
      ToolboxIdlTypedef.tryParse,
    );
    let accounts = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'accounts',
      typedefs,
      undefined,
      ToolboxIdlAccount.tryParse,
    );
    let instructions = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'instructions',
      typedefs,
      accounts,
      ToolboxIdlInstruction.tryParse,
    );
    let errors = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'errors',
      undefined,
      undefined,
      ToolboxIdlError.tryParse,
    );
    return new ToolboxIdlProgram({ typedefs, accounts, instructions, errors });
  }

  static tryParseScopedNamedValues<T, P1, P2>(
    idlRoot: any,
    collectionKey: string,
    param1: P1,
    param2: P2,
    parsingFunction: (name: string, value: any, param1: P1, param2: P2) => T,
  ): Map<string, T> {
    let values = new Map();
    let collection = idlRoot[collectionKey];
    if (ToolboxUtils.isArray(collection)) {
      collection.forEach((item: any) => {
        let name = ToolboxUtils.expectString(item['name']);
        values.set(name, parsingFunction(name, item, param1, param2));
      });
    }
    if (ToolboxUtils.isObject(collection)) {
      Object.entries(collection).forEach(([key, value]) => {
        values.set(key, parsingFunction(key, value, param1, param2));
      });
    }
    return values;
  }

  public guessAccount(accountData: Buffer): ToolboxIdlAccount | null {
    for (let account of this.accounts.values()) {
      try {
        if (account.check(accountData)) {
          return account;
        }
      } catch {}
    }
    return null;
  }
}
