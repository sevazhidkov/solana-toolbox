import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { deserialize } from './ToolboxIdlTypeFull.deserialize';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlAccount {
  public static readonly Unknown = new ToolboxIdlAccount({
    name: 'unknown',
    docs: undefined,
    discriminator: Buffer.from([]),
    contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
    contentTypeFull: ToolboxIdlTypeFull.nothing(),
  });

  public name: string;
  public docs: any;
  // TODO - support space/chunks/blobs
  public discriminator: Buffer;
  public contentTypeFlat: ToolboxIdlTypeFlat;
  public contentTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    docs: any;
    discriminator: Buffer;
    contentTypeFlat: ToolboxIdlTypeFlat;
    contentTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.contentTypeFlat = value.contentTypeFlat;
    this.contentTypeFull = value.contentTypeFull;
  }

  public static tryParse(
    idlAccountName: string,
    idlAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlAccount {
    let docs = idlAccount['docs'];
    let discriminator = Buffer.from(
      idlAccount['discriminator'] ??
        ToolboxUtils.discriminator(`account:${idlAccountName}`),
    );
    let contentTypeFlat = parseObjectIsPossible(idlAccount)
      ? parse(idlAccount)
      : parse(idlAccountName);
    let contentTypeFull = hydrate(contentTypeFlat, new Map(), typedefs);
    return new ToolboxIdlAccount({
      name: idlAccountName,
      docs,
      discriminator,
      contentTypeFlat,
      contentTypeFull,
    });
  }

  public encode(accountState: any): Buffer {
    let data: Buffer[] = [];
    data.push(this.discriminator);
    serialize(this.contentTypeFull, accountState, data, true);
    return Buffer.concat(data);
  }

  public decode(accountData: Buffer): any {
    this.check(accountData);
    let [, accountState] = deserialize(
      this.contentTypeFull,
      accountData,
      this.discriminator.length,
    );
    return accountState;
  }

  public check(accountData: Buffer) {
    if (accountData.length < this.discriminator.length) {
      throw new Error('Invalid discriminator');
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (accountData[i] !== this.discriminator[i]) {
        throw new Error('Invalid discriminator');
      }
    }
  }
}
