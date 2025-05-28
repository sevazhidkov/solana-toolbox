export class ToolboxIdlTypePrefix {
  public static readonly U8 = new ToolboxIdlTypePrefix(1);
  public static readonly U16 = new ToolboxIdlTypePrefix(2);
  public static readonly U32 = new ToolboxIdlTypePrefix(4);
  public static readonly U64 = new ToolboxIdlTypePrefix(8);

  private size: number;

  private constructor(size: number) {
    this.size = size;
  }

  public toBuffer(value: number): Buffer {
    let buffer = Buffer.alloc(this.size);
    if (this.size === 1) {
      buffer.writeUInt8(value);
    }
    if (this.size === 2) {
      buffer.writeUInt16LE(value);
    }
    if (this.size === 4) {
      buffer.writeUInt32LE(value);
    }
    if (this.size === 8) {
      buffer.writeBigUint64LE(BigInt(value));
    }
    return buffer;
  }
}
