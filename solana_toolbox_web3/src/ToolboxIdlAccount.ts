import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';

export class ToolboxIdlAccount {
  public name: string;
  // TODO - support space/chunks/blobs
  public discriminator: Buffer;
  public contentTypeFlat: ToolboxIdlTypeFlat;
  // public contentTypeFull: ToolboxIdlTypeFull;

  constructor(
    name: string,
    discriminator: Buffer,
    contentTypeFlat: ToolboxIdlTypeFlat,
    // contentTypeFull: ToolboxIdlTypeFull,
  ) {
    this.name = name;
    this.discriminator = discriminator;
    this.contentTypeFlat = contentTypeFlat;
    // this.contentTypeFull = contentTypeFull;
  }

  public static tryParse(
    idlAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlAccount {
    let name = idlAccount['name'] as string;
    let discriminator = Buffer.from(idlAccount['discriminator']) as Buffer;
    let contentTypeFlat = parseObjectIsPossible(idlAccount)
      ? parse(idlAccount)
      : parse(name);
    // let contentTypeFull = contentTypeFlat.tryHydrate(new Map(), typedefs);
    return new ToolboxIdlAccount(
      name,
      discriminator,
      contentTypeFlat,
      // contentTypeFull,
    );
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
