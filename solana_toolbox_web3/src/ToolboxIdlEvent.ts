import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlEvent {
  public name: string;
  public discriminator: Buffer;
  public infoTypeFlat: ToolboxIdlTypeFlat;
  public infoTypeFull: ToolboxIdlTypeFull;

  constructor(value: {
    name: string;
    discriminator: Buffer;
    infoTypeFlat: ToolboxIdlTypeFlat;
    infoTypeFull: ToolboxIdlTypeFull;
  }) {
    this.name = value.name;
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
    let discriminator = Buffer.from(
      ToolboxUtils.expectArray(
        idlEvent['discriminator'] ??
          ToolboxUtils.discriminator('event:' + idlEventName),
      ),
    );
    let infoTypeFlat = parseObjectIsPossible(idlEvent)
      ? parse(idlEvent)
      : parse(idlEventName);
    let infoTypeFull = hydrate(infoTypeFlat, new Map(), typedefs);
    return new ToolboxIdlEvent({
      name: idlEventName,
      discriminator,
      infoTypeFlat,
      infoTypeFull,
    });
  }
}
