import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlAccount {
  public name: string;
  // TODO - support space/chunks/blobs
  public discriminator: Buffer;
  public contentTypeFlat: ToolboxIdlTypeFlat;
  // public contentTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    discriminator: Buffer;
    contentTypeFlat: ToolboxIdlTypeFlat;
    // contentTypeFull: ToolboxIdlTypeFull,
  }) {
    this.name = value.name;
    this.discriminator = value.discriminator;
    this.contentTypeFlat = value.contentTypeFlat;
    // this.contentTypeFull = value.contentTypeFull;
  }

  public static tryParse(
    idlAccountName: string,
    idlAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlAccount {
    let discriminator = Buffer.from(
      idlAccount['discriminator'] ??
        ToolboxUtils.discriminator('account:' + idlAccountName),
    );
    let contentTypeFlat = parseObjectIsPossible(idlAccount)
      ? parse(idlAccount)
      : parse(idlAccountName);
    // let contentTypeFull = contentTypeFlat.tryHydrate(new Map(), typedefs);
    return new ToolboxIdlAccount({
      name: idlAccountName,
      discriminator,
      contentTypeFlat,
    });
  }

  public check(accountData: Buffer): boolean {
    if (accountData.length < this.discriminator.length) {
      return false;
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (accountData[i] !== this.discriminator[i]) {
        return false;
      }
    }
    return true;
  }
}
