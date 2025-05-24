import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';

export class ToolboxIdlAccount {
  public name: string;
  public docs: any;
  // TODO - support space/chunks/blobs
  public discriminator: Buffer;
  public contentTypeFlat: ToolboxIdlTypeFlat;
  public contentTypeFull: ToolboxIdlTypeFull;

  constructor(
    name: string,
    docs: any,
    discriminator: Buffer,
    contentTypeFlat: ToolboxIdlTypeFlat,
    contentTypeFull: ToolboxIdlTypeFull,
  ) {
    this.name = name;
    this.docs = docs;
    this.discriminator = discriminator;
    this.contentTypeFlat = contentTypeFlat;
    this.contentTypeFull = contentTypeFull;
  }

  public static tryParse(
    idlAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlAccount {
    let name = idlAccount['name'] as string;
    let docs = idlAccount['docs'];
    let discriminator = Buffer.from(idlAccount['discriminator']) as Buffer;
    let contentTypeFlat = ToolboxIdlTypeFlat.tryParseObjectIsPossible(
      idlAccount,
    )
      ? ToolboxIdlTypeFlat.tryParse(idlAccount)
      : ToolboxIdlTypeFlat.tryParse(name);
    let contentTypeFull = contentTypeFlat.tryHydrate(new Map(), typedefs);
    return new ToolboxIdlAccount(
      name,
      docs,
      discriminator,
      contentTypeFlat,
      contentTypeFull,
    );
  }
}
