import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse } from './ToolboxIdlTypeFlat.parse';

export class ToolboxIdlTypedef {
  public name: string;
  public serialization?: string;
  public repr?: string;
  public generics: string[];
  public typeFlat: ToolboxIdlTypeFlat;

  constructor(
    name: string,
    serialization: string | undefined,
    repr: string | undefined,
    generics: string[],
    typeFlat: ToolboxIdlTypeFlat,
  ) {
    this.name = name;
    this.serialization = serialization;
    this.repr = repr;
    this.generics = generics;
    this.typeFlat = typeFlat;
  }

  public static tryParse(
    idlTypedefName: string,
    idlTypedef: any,
  ): ToolboxIdlTypedef {
    // TODO - proper parsing
    let serialization = idlTypedef['serialization'] as string | undefined;
    let repr = idlTypedef['repr'] as string | undefined;
    let generics = idlTypedef['generics'] as string[] | undefined;
    let typeFlat = parse(idlTypedef);
    return new ToolboxIdlTypedef(
      idlTypedefName,
      serialization,
      repr,
      generics ?? [],
      typeFlat,
    );
  }
}
