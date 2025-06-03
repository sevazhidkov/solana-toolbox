import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullArray,
  ToolboxIdlTypeFullConst,
  ToolboxIdlTypeFullEnum,
  ToolboxIdlTypeFullEnumVariant,
  ToolboxIdlTypeFullFieldNamed,
  ToolboxIdlTypeFullFields,
  ToolboxIdlTypeFullFieldUnnamed,
  ToolboxIdlTypeFullOption,
  ToolboxIdlTypeFullPadded,
  ToolboxIdlTypeFullString,
  ToolboxIdlTypeFullStruct,
  ToolboxIdlTypeFullTypedef,
  ToolboxIdlTypeFullVec,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';

export function serialize(
  typeFull: ToolboxIdlTypeFull,
  value: any,
  data: Buffer[],
  deserializable: boolean,
) {
  typeFull.traverse(serializeVisitor, value, data, deserializable);
}

let serializeVisitor = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    ToolboxUtils.withContext(() => {
      return serialize(self.content, value, data, deserializable);
    }, `Serialize: Typedef: ${self.name}`);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    if (value === null) {
      serializePrefix(self.prefix, 0, data);
      return;
    }
    serializePrefix(self.prefix, 1, data);
    serialize(self.content, value, data, deserializable);
  },
  vec: (
    self: ToolboxIdlTypeFullVec,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    let array = ToolboxUtils.expectArray(value);
    if (deserializable) {
      serializePrefix(self.prefix, array.length, data);
    }
    for (let item of array) {
      serialize(self.items, item, data, deserializable);
    }
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    let array = ToolboxUtils.expectArray(value);
    if (array.length != self.length) {
      throw new Error(`Expected an array of size: ${self.length}`);
    }
    for (let item of array) {
      serialize(self.items, item, data, deserializable);
    }
  },
  string: (
    self: ToolboxIdlTypeFullString,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    let string = ToolboxUtils.expectString(value);
    if (deserializable) {
      serializePrefix(self.prefix, string.length, data);
    }
    data.push(Buffer.from(string, 'utf8'));
  },
  struct: (
    self: ToolboxIdlTypeFullStruct,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    serializeFields(self.fields, value, data, deserializable);
  },
  enum: (
    self: ToolboxIdlTypeFullEnum,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    function serializeEnumVariant(
      variant: ToolboxIdlTypeFullEnumVariant,
      value: any,
    ) {
      ToolboxUtils.withContext(() => {
        serializePrefix(self.prefix, variant.code, data);
        serializeFields(variant.fields, value, data, deserializable);
      }, `Serialize: Enum Variant: ${variant.name}`);
    }
    if (ToolboxUtils.isNumber(value)) {
      for (let variant of self.variants) {
        if (variant.code == value) {
          return serializeEnumVariant(variant, null);
        }
      }
      throw new Error(`Could not find enum variant with code: ${value}`);
    }
    if (ToolboxUtils.isString(value)) {
      for (let variant of self.variants) {
        if (variant.name == value) {
          return serializeEnumVariant(variant, null);
        }
      }
      throw new Error(`Could not find enum variant with name: ${value}`);
    }
    if (ToolboxUtils.isObject(value)) {
      for (let variant of self.variants) {
        if (value.hasOwnProperty(variant.name)) {
          return serializeEnumVariant(variant, value[variant.name]);
        }
      }
      throw new Error('Could not guess enum variant from object key');
    }
    throw new Error('Expected enum value to be: number/string or object');
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    if (self.before) {
      data.push(Buffer.alloc(self.before));
    }
    let contentData: Buffer[] = [];
    serialize(self.content, value, contentData, deserializable);
    for (let contentBuffer of contentData) {
      data.push(contentBuffer);
    }
    let contentSize = contentData.reduce((size, contentBuffer) => {
      return size + contentBuffer.length;
    }, 0);
    if (self.minSize > contentSize) {
      data.push(Buffer.alloc(self.minSize - contentSize));
    }
    if (self.after) {
      data.push(Buffer.alloc(self.after));
    }
  },
  const: (
    _self: ToolboxIdlTypeFullConst,
    _value: any,
    _data: Buffer[],
    _deserializable: boolean,
  ) => {
    throw new Error('Cannot serialize a const type');
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    value: any,
    data: Buffer[],
    _deserializable: boolean,
  ) => {
    serializePrimitive(self, value, data);
  },
};

export function serializeFields(
  typeFullFields: ToolboxIdlTypeFullFields,
  value: any,
  data: Buffer[],
  deserializable: boolean,
) {
  typeFullFields.traverse(serializeFieldsVisitor, value, data, deserializable);
}

let serializeFieldsVisitor = {
  named: (
    self: ToolboxIdlTypeFullFieldNamed[],
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectObject(value);
    for (let field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.name], data, deserializable);
      }, `Serialize: Field: ${field.name}`);
    }
  },
  unnamed: (
    self: ToolboxIdlTypeFullFieldUnnamed[],
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectArray(value);
    for (let field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.position], data, deserializable);
      }, `Serialize: Field: ${field.position}`);
    }
  },
};

export function serializePrefix(
  prefix: ToolboxIdlTypePrefix,
  value: number,
  data: Buffer[],
) {
  let buffer = Buffer.alloc(prefix.size);
  prefix.traverse(serializePrefixVisitor, buffer, value);
  data.push(buffer);
}

let serializePrefixVisitor = {
  u8: (buffer: Buffer, value: any) => {
    buffer.writeUInt8(ToolboxUtils.expectNumber(value));
  },
  u16: (buffer: Buffer, value: any) => {
    buffer.writeUInt16LE(ToolboxUtils.expectNumber(value));
  },
  u32: (buffer: Buffer, value: any) => {
    buffer.writeUInt32LE(ToolboxUtils.expectNumber(value));
  },
  u64: (buffer: Buffer, value: any) => {
    buffer.writeBigUInt64LE(BigInt(ToolboxUtils.expectNumber(value)));
  },
  u128: (buffer: Buffer, value: any) => {
    buffer.writeBigUInt64LE(BigInt(ToolboxUtils.expectNumber(value)));
  },
};

export function serializePrimitive(
  primitive: ToolboxIdlTypePrimitive,
  value: any,
  data: Buffer[],
) {
  let buffer = Buffer.alloc(primitive.size);
  primitive.traverse(serializePrimitiveVisitor, buffer, value);
  data.push(buffer);
}

let serializePrimitiveVisitor = {
  u8: (buffer: Buffer, value: any) => {
    buffer.writeUInt8(ToolboxUtils.expectNumber(value));
  },
  u16: (buffer: Buffer, value: any) => {
    buffer.writeUInt16LE(ToolboxUtils.expectNumber(value));
  },
  u32: (buffer: Buffer, value: any) => {
    buffer.writeUInt32LE(ToolboxUtils.expectNumber(value));
  },
  u64: (buffer: Buffer, value: any) => {
    buffer.writeBigUInt64LE(ToolboxUtils.expectBigInt(value));
  },
  u128: (buffer: Buffer, value: any) => {
    let num = ToolboxUtils.expectBigInt(value);
    let low = num & 0xffffffffffffffffn;
    let high = (num >> 64n) & 0xffffffffffffffffn;
    buffer.writeBigUInt64LE(low, 0);
    buffer.writeBigUInt64LE(high, 8);
  },
  i8: (buffer: Buffer, value: any) => {
    buffer.writeInt8(ToolboxUtils.expectNumber(value));
  },
  i16: (buffer: Buffer, value: any) => {
    buffer.writeInt16LE(ToolboxUtils.expectNumber(value));
  },
  i32: (buffer: Buffer, value: any) => {
    buffer.writeInt32LE(ToolboxUtils.expectNumber(value));
  },
  i64: (buffer: Buffer, value: any) => {
    buffer.writeBigInt64LE(ToolboxUtils.expectBigInt(value));
  },
  i128: (buffer: Buffer, value: any) => {
    let num = ToolboxUtils.expectBigInt(value);
    let low = BigInt.asIntN(64, num);
    let high = BigInt.asIntN(64, num >> 64n);
    buffer.writeBigInt64LE(low, 0);
    buffer.writeBigInt64LE(high, 8);
  },
  f32: (buffer: Buffer, value: any) => {
    buffer.writeFloatLE(ToolboxUtils.expectNumber(value));
  },
  f64: (buffer: Buffer, value: any) => {
    buffer.writeDoubleLE(ToolboxUtils.expectNumber(value));
  },
  bool: (buffer: Buffer, value: any) => {
    if (ToolboxUtils.expectBoolean(value)) {
      buffer.writeUInt8(1);
    } else {
      buffer.writeUInt8(0);
    }
  },
  pubkey: (buffer: Buffer, value: any) => {
    buffer.set(new PublicKey(ToolboxUtils.expectString(value)).toBuffer());
  },
};
