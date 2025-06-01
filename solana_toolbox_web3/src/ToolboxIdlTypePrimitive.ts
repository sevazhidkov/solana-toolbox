export class ToolboxIdlTypePrimitive {
  public static readonly U8 = new ToolboxIdlTypePrimitive('u8', 1, 1);
  public static readonly U16 = new ToolboxIdlTypePrimitive('u16', 2, 2);
  public static readonly U32 = new ToolboxIdlTypePrimitive('u32', 4, 4);
  public static readonly U64 = new ToolboxIdlTypePrimitive('u64', 8, 8);
  public static readonly U128 = new ToolboxIdlTypePrimitive('u128', 16, 16);
  public static readonly I8 = new ToolboxIdlTypePrimitive('i8', 1, 1);
  public static readonly I16 = new ToolboxIdlTypePrimitive('i16', 2, 2);
  public static readonly I32 = new ToolboxIdlTypePrimitive('i32', 4, 4);
  public static readonly I64 = new ToolboxIdlTypePrimitive('i64', 8, 8);
  public static readonly I128 = new ToolboxIdlTypePrimitive('i128', 16, 16);
  public static readonly F32 = new ToolboxIdlTypePrimitive('f32', 4, 4);
  public static readonly F64 = new ToolboxIdlTypePrimitive('f64', 8, 8);
  public static readonly Bool = new ToolboxIdlTypePrimitive('bool', 1, 1);
  public static readonly Pubkey = new ToolboxIdlTypePrimitive('pubkey', 32, 1);

  public static readonly primitiveByName = (() => {
    let primitives = [
      ToolboxIdlTypePrimitive.U8,
      ToolboxIdlTypePrimitive.U16,
      ToolboxIdlTypePrimitive.U32,
      ToolboxIdlTypePrimitive.U64,
      ToolboxIdlTypePrimitive.U128,
      ToolboxIdlTypePrimitive.I8,
      ToolboxIdlTypePrimitive.I16,
      ToolboxIdlTypePrimitive.I32,
      ToolboxIdlTypePrimitive.I64,
      ToolboxIdlTypePrimitive.I128,
      ToolboxIdlTypePrimitive.F32,
      ToolboxIdlTypePrimitive.F64,
      ToolboxIdlTypePrimitive.Bool,
      ToolboxIdlTypePrimitive.Pubkey,
    ];
    let primitivesByName = new Map<string, ToolboxIdlTypePrimitive>();
    for (let primitive of primitives) {
      primitivesByName.set(primitive.name, primitive);
    }
    return primitivesByName;
  })();

  public name: string;
  public size: number;
  public alignment: number;

  private constructor(name: string, size: number, alignment: number) {
    this.name = name;
    this.size = size;
    this.alignment = alignment;
  }

  public traverse<P1, P2, T>(
    visitor: {
      u8: (param1: P1, param2: P2) => T;
      u16: (param1: P1, param2: P2) => T;
      u32: (param1: P1, param2: P2) => T;
      u64: (param1: P1, param2: P2) => T;
      u128: (param1: P1, param2: P2) => T;
      i8: (param1: P1, param2: P2) => T;
      i16: (param1: P1, param2: P2) => T;
      i32: (param1: P1, param2: P2) => T;
      i64: (param1: P1, param2: P2) => T;
      i128: (param1: P1, param2: P2) => T;
      f32: (param1: P1, param2: P2) => T;
      f64: (param1: P1, param2: P2) => T;
      bool: (param1: P1, param2: P2) => T;
      pubkey: (param1: P1, param2: P2) => T;
    },
    param1: P1,
    param2: P2,
  ): T {
    return visitor[this.name as keyof typeof visitor](param1, param2);
  }
}
