import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse } from './ToolboxIdlTypeFlat.parse';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlTypedef {
  public name: string;
  public docs: any;
  public serialization?: string;
  public repr?: string;
  public generics: string[];
  public typeFlat: ToolboxIdlTypeFlat;

  constructor(
    name: string,
    docs: any,
    serialization: string | undefined,
    repr: string | undefined,
    generics: string[],
    typeFlat: ToolboxIdlTypeFlat,
  ) {
    this.name = name;
    this.docs = docs;
    this.serialization = serialization;
    this.repr = repr;
    this.generics = generics;
    this.typeFlat = typeFlat;
  }

  public static tryParse(
    idlTypedefName: string,
    idlTypedef: any,
  ): ToolboxIdlTypedef {
    ToolboxUtils.expectObject(idlTypedef);
    let docs = idlTypedef['docs'];
    let serialization = undefined;
    if (ToolboxUtils.isString(idlTypedef['serialization'])) {
      serialization = idlTypedef['serialization'];
    }
    let repr = undefined;
    if (ToolboxUtils.isString(idlTypedef['repr'])) {
      repr = idlTypedef['repr'];
    }
    if (ToolboxUtils.isObject(idlTypedef['repr'])) {
      repr = ToolboxUtils.expectString(idlTypedef['repr']['kind']);
    }
    let generics = [];
    if (ToolboxUtils.isArray(idlTypedef['generics'])) {
      for (let idlGeneric of idlTypedef['generics']) {
        if (ToolboxUtils.isString(idlGeneric)) {
          generics.push(idlGeneric);
        } else {
          ToolboxUtils.expectObject(idlGeneric);
          generics.push(ToolboxUtils.expectString(idlGeneric['name']));
        }
      }
    }
    let typeFlat = parse(idlTypedef);
    return new ToolboxIdlTypedef(
      idlTypedefName,
      docs,
      serialization,
      repr,
      generics,
      typeFlat,
    );
  }
}
