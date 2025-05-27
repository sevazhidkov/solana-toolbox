import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstructionAccount } from './ToolboxIdlInstructionAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from './ToolboxIdlTypeFlat';
import { parse, parseFields } from './ToolboxIdlTypeFlat.parse';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlInstruction {
  public name: string;
  public discriminator: Buffer;
  public accounts: ToolboxIdlInstructionAccount[];
  public argsTypeFlatFields: ToolboxIdlTypeFlatFields;
  public returnTypeFlat: ToolboxIdlTypeFlat;

  constructor(value: {
    name: string;
    discriminator: Buffer;
    accounts: ToolboxIdlInstructionAccount[];
    argsTypeFlatFields: ToolboxIdlTypeFlatFields;
    returnTypeFlat: ToolboxIdlTypeFlat;
  }) {
    this.name = value.name;
    this.discriminator = value.discriminator;
    this.accounts = value.accounts;
    this.argsTypeFlatFields = value.argsTypeFlatFields;
    this.returnTypeFlat = value.returnTypeFlat;
  }

  public static tryParse(
    idlInstructionName: string,
    idlInstruction: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
    accounts: Map<string, ToolboxIdlAccount>,
  ): ToolboxIdlInstruction {
    let discriminator = Buffer.from(
      idlInstruction['discriminator'] ??
        ToolboxUtils.discriminator('global:' + idlInstructionName),
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
    // TODO - implement parsing logic and case sensitive discriminator
    let argsTypeFlatFields = parseFields(idlInstruction['args'] ?? []);
    let returnTypeFlat = parse(idlInstruction['returns'] ?? { fields: [] });
    return new ToolboxIdlInstruction({
      name: idlInstructionName,
      discriminator,
      accounts: instructionAccounts,
      argsTypeFlatFields,
      returnTypeFlat,
    });
  }
}
