export class ToolboxIdlTypePrimitive {
  public static readonly U8 = new ToolboxIdlTypePrimitive('u8');
  public static readonly U16 = new ToolboxIdlTypePrimitive('u16');
  public static readonly U32 = new ToolboxIdlTypePrimitive('u32');
  public static readonly U64 = new ToolboxIdlTypePrimitive('u64');
  public static readonly U128 = new ToolboxIdlTypePrimitive('u128');
  public static readonly I8 = new ToolboxIdlTypePrimitive('i8');
  public static readonly I16 = new ToolboxIdlTypePrimitive('i16');
  public static readonly I32 = new ToolboxIdlTypePrimitive('i32');
  public static readonly I64 = new ToolboxIdlTypePrimitive('i64');
  public static readonly I128 = new ToolboxIdlTypePrimitive('i128');
  public static readonly F32 = new ToolboxIdlTypePrimitive('f32');
  public static readonly F64 = new ToolboxIdlTypePrimitive('f64');
  public static readonly Boolean = new ToolboxIdlTypePrimitive('bool');
  public static readonly PublicKey = new ToolboxIdlTypePrimitive('pubkey');

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
      ToolboxIdlTypePrimitive.Boolean,
      ToolboxIdlTypePrimitive.PublicKey,
    ];
    let primitivesByName = new Map<string, ToolboxIdlTypePrimitive>();
    primitives.forEach((primitive) => {
      primitivesByName.set(primitive.name, primitive);
    });
    return primitivesByName;
  })();

  private name: string;

  private constructor(name: string) {
    this.name = name;
  }
}
