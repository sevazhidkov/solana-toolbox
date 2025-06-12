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
    space: undefined,
    blobs: [],
    discriminator: Buffer.from([]),
    contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
    contentTypeFull: ToolboxIdlTypeFull.nothing(),
  });

  public readonly name: string;
  public readonly docs: any;
  public readonly space: number | undefined;
  public readonly blobs: { offset: number; value: Buffer }[];
  public readonly discriminator: Buffer;
  public readonly contentTypeFlat: ToolboxIdlTypeFlat;
  public readonly contentTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    docs: any;
    space: number | undefined;
    blobs: { offset: number; value: Buffer }[];
    discriminator: Buffer;
    contentTypeFlat: ToolboxIdlTypeFlat;
    contentTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
    this.docs = value.docs;
    this.space = value.space;
    this.blobs = value.blobs;
    this.discriminator = value.discriminator;
    this.contentTypeFlat = value.contentTypeFlat;
    this.contentTypeFull = value.contentTypeFull;
  }

  public static tryParse(
    idlAccountName: string,
    idlAccount: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlAccount {
    ToolboxUtils.expectObject(idlAccount);
    let docs = idlAccount['docs'];
    let space = undefined;
    if (ToolboxUtils.isNumber(idlAccount['space'])) {
      space = idlAccount['space'];
    }
    let blobs = [];
    if (ToolboxUtils.isArray(idlAccount['blobs'])) {
      for (let blob of idlAccount['blobs']) {
        ToolboxUtils.expectObject(blob);
        blobs.push({
          offset: ToolboxUtils.expectNumber(blob['offset']),
          value: Buffer.from(ToolboxUtils.expectArray(blob['value'])),
        });
      }
    }
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
      space,
      blobs,
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
    if (this.space !== undefined) {
      if (accountData.length !== this.space) {
        throw new Error(
          `Invalid account data length ${accountData.length} for account space ${this.space}`,
        );
      }
    }
    for (let blob of this.blobs) {
      if (
        blob.offset < 0 ||
        blob.offset + blob.value.length > accountData.length
      ) {
        throw new Error(
          `Invalid blob offset ${blob.offset} with length ${blob.value.length} in account data of length ${accountData.length}`,
        );
      }
      for (let i = 0; i < blob.value.length; i++) {
        if (accountData[blob.offset + i] !== blob.value[i]) {
          throw new Error(
            `Invalid blob value at offset ${blob.offset + i} in account data`,
          );
        }
      }
    }
    if (accountData.length < this.discriminator.length) {
      throw new Error(
        `Invalid account data length ${accountData.length} for discriminator length ${this.discriminator.length}`,
      );
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (accountData[i] !== this.discriminator[i]) {
        throw new Error(`Invalid discriminator at index ${i} in account data`);
      }
    }
  }
}
