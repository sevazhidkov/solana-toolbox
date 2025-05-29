import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { deserialize } from './ToolboxIdlTypeFull.deserialize';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlAccount {
  public name: string;
  // TODO - support space/chunks/blobs
  public discriminator: Buffer;
  public contentTypeFlat: ToolboxIdlTypeFlat;
  public contentTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    discriminator: Buffer;
    contentTypeFlat: ToolboxIdlTypeFlat;
    contentTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
    this.discriminator = value.discriminator;
    this.contentTypeFlat = value.contentTypeFlat;
    this.contentTypeFull = value.contentTypeFull;
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
    let contentTypeFull = hydrate(contentTypeFlat, new Map(), typedefs);
    return new ToolboxIdlAccount({
      name: idlAccountName,
      discriminator,
      contentTypeFlat,
      contentTypeFull,
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

  public encode(accountState: any): Buffer {
    let data: Buffer[] = [];
    data.push(this.discriminator);
    serialize(this.contentTypeFull, accountState, data, true);
    return Buffer.concat(data);
  }

  public decode(accountData: Buffer): any {
    if (!this.check(accountData)) {
      throw new Error('Invalid account type'); // TODO - better error handling
    }
    let [accountSize, accountState] = deserialize(
      this.contentTypeFull,
      accountData,
      this.discriminator.length,
    );
    return accountState;
  }
}
