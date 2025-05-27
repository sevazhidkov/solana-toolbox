import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse } from './ToolboxIdlTypeFlat.parse';

export class ToolboxIdlTypedef {
  public name: string;
  public serialization?: string;
  public repr?: string;
  public generics: string[];
  public typeFlat: ToolboxIdlTypeFlat;

  private constructor(
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

  public static tryParse(idlTypedef: any): ToolboxIdlTypedef {
    let name = idlTypedef['name'] as string;
    let serialization = idlTypedef['serialization'] as string | undefined;
    let repr = idlTypedef['repr'] as string | undefined;
    let generics = idlTypedef['generics'] as string[] | undefined; // TODO - proper parsing
    let typeFlat = parse(idlTypedef);
    return new ToolboxIdlTypedef(
      name,
      serialization,
      repr,
      generics ?? [],
      typeFlat,
    );
  }
}
