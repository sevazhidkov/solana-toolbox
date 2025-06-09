import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { deserialize } from './ToolboxIdlTypeFull.deserialize';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlEvent {
  public readonly name: string;
  public readonly docs: any;
  public readonly discriminator: Buffer;
  public readonly infoTypeFlat: ToolboxIdlTypeFlat;
  public readonly infoTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    docs: any;
    discriminator: Buffer;
    infoTypeFlat: ToolboxIdlTypeFlat;
    infoTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.infoTypeFlat = value.infoTypeFlat;
    this.infoTypeFull = value.infoTypeFull;
  }

  public static tryParse(
    idlEventName: string,
    idlEvent: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlEvent {
    ToolboxUtils.expectObject(idlEvent);
    let docs = idlEvent['docs'];
    let discriminator = Buffer.from(
      ToolboxUtils.expectArray(
        idlEvent['discriminator'] ??
          ToolboxUtils.discriminator(`event:${idlEventName}`),
      ),
    );
    let infoTypeFlat = parseObjectIsPossible(idlEvent)
      ? parse(idlEvent)
      : parse(idlEventName);
    let infoTypeFull = hydrate(infoTypeFlat, new Map(), typedefs);
    return new ToolboxIdlEvent({
      name: idlEventName,
      docs,
      discriminator,
      infoTypeFlat,
      infoTypeFull,
    });
  }

  public encode(eventState: any): Buffer {
    let data: Buffer[] = [];
    data.push(this.discriminator);
    serialize(this.infoTypeFull, eventState, data, true);
    return Buffer.concat(data);
  }

  public decode(eventData: Buffer): any {
    this.check(eventData);
    let [, eventState] = deserialize(
      this.infoTypeFull,
      eventData,
      this.discriminator.length,
    );
    return eventState;
  }

  public check(eventData: Buffer) {
    if (eventData.length < this.discriminator.length) {
      throw new Error('Invalid discriminator');
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (eventData[i] !== this.discriminator[i]) {
        throw new Error('Invalid discriminator');
      }
    }
  }
}
