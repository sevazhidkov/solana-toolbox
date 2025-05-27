import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from './ToolboxIdlTypeFlat';
import { parse, parseFields } from './ToolboxIdlTypeFlat.parse';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlInstructionAccount {
  public name: string;
  public writable: boolean;
  public signer: boolean;
  public optional: boolean;
  public address: PublicKey | undefined;
  public pda: any[] | undefined;

  constructor(value: {
    name: string;
    writable: boolean;
    signer: boolean;
    optional: boolean;
    address?: PublicKey;
    pda?: any[];
  }) {
    this.name = value.name;
    this.writable = value.writable;
    this.signer = value.signer;
    this.optional = value.optional;
    this.address = value.address;
    this.pda = value.pda;
  }

  public static tryParse(
    idlInstructionAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
    accounts: Map<string, ToolboxIdlAccount>,
  ): ToolboxIdlInstructionAccount {
    ToolboxUtils.expectObject(idlInstructionAccount);
    let name = ToolboxUtils.expectString(idlInstructionAccount['name']);
    let writable = ToolboxUtils.expectBoolean(
      idlInstructionAccount['writable'] ??
        idlInstructionAccount['isMut'] ??
        false,
    );
    let signer = ToolboxUtils.expectBoolean(
      idlInstructionAccount['signer'] ??
        idlInstructionAccount['isSigner'] ??
        false,
    );
    let optional = ToolboxUtils.expectBoolean(
      idlInstructionAccount['optional'] ??
        idlInstructionAccount['isOptional'] ??
        false,
    );
    let address = undefined;
    if (idlInstructionAccount['address']) {
      address = new PublicKey(
        ToolboxUtils.expectString(idlInstructionAccount['address']),
      );
    }
    return new ToolboxIdlInstructionAccount({
      name,
      writable,
      signer,
      optional,
      address,
    });
  }
}
