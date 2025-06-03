import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullArray,
  ToolboxIdlTypeFullConst,
  ToolboxIdlTypeFullEnum,
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
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';

export function deserialize(
  typeFull: ToolboxIdlTypeFull,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return typeFull.traverse(deserializeVisitor, data, dataOffset, undefined);
}

let deserializeVisitor = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return ToolboxUtils.withContext(() => {
      return deserialize(self.content, data, dataOffset);
    }, `Deserialize: Typedef: ${self.name} (offset: ${dataOffset})`);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    if ((dataPrefix & 1) == 0) {
      return [dataSize, null];
    }
    let dataContentOffset = dataOffset + dataSize;
    let [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset,
    );
    dataSize += dataContentSize;
    return [dataSize, dataContent];
  },
  vec: (
    self: ToolboxIdlTypeFullVec,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    let dataItems = [];
    for (let i = 0; i < dataPrefix; i++) {
      let dataItemOffset = dataOffset + dataSize;
      let [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset,
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = 0;
    let dataItems = [];
    for (let i = 0; i < self.length; i++) {
      let dataItemOffset = dataOffset + dataSize;
      let [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset,
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  string: (
    self: ToolboxIdlTypeFullString,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    let dataCharsOffset = dataOffset + dataSize;
    let dataString = data.toString(
      'utf8',
      dataCharsOffset,
      dataCharsOffset + dataPrefix,
    );
    dataSize += dataPrefix;
    return [dataSize, dataString];
  },
  struct: (
    self: ToolboxIdlTypeFullStruct,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserializeFields(self.fields, data, dataOffset);
  },
  enum: (
    self: ToolboxIdlTypeFullEnum,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let enumMask = 0;
    for (let variant of self.variants) {
      enumMask |= variant.code;
    }
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    let dataVariantOffset = dataOffset + dataSize;
    for (let variant of self.variants) {
      if (variant.code === (dataPrefix & enumMask)) {
        let [dataVariantSize, dataVariant] = ToolboxUtils.withContext(() => {
          return deserializeFields(variant.fields, data, dataVariantOffset);
        }, `Deserialize: Enum Variant: ${variant.name} (offset: ${dataVariantOffset})`);
        dataSize += dataVariantSize;
        if (dataVariant === null) {
          return [dataSize, variant.name];
        }
        return [dataSize, { [variant.name]: dataVariant }];
      }
    }
    throw new Error(
      `Deserialize: Unknown enum code: ${dataPrefix} (offset: ${dataOffset})`,
    );
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = self.before;
    let dataContentOffset = dataOffset + dataSize;
    let [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset,
    );
    dataSize += Math.max(dataContentSize, self.minSize);
    dataSize += self.after;
    return [dataSize, dataContent];
  },
  const: (
    _self: ToolboxIdlTypeFullConst,
    _data: Buffer,
    _dataOffset: number,
  ): [number, any] => {
    throw new Error('Cannot deserialize a const type');
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserializePrimitive(self, data, dataOffset);
  },
};

export function deserializeFields(
  fields: ToolboxIdlTypeFullFields,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return fields.traverse(deserializeFieldsVisitor, data, dataOffset, undefined);
}

let deserializeFieldsVisitor = {
  named: (
    self: ToolboxIdlTypeFullFieldNamed[],
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    if (self.length <= 0) {
      return [0, null];
    }
    let dataSize = 0;
    let dataFields: Record<string, any> = {};
    for (let field of self) {
      let dataFieldOffset = dataOffset + dataSize;
      let [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.name} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields[field.name] = dataField;
    }
    return [dataSize, dataFields];
  },
  unnamed: (
    self: ToolboxIdlTypeFullFieldUnnamed[],
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    if (self.length <= 0) {
      return [0, null];
    }
    let dataSize = 0;
    let dataFields = [];
    for (let field of self) {
      let dataFieldOffset = dataOffset + dataSize;
      let [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.position} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields.push(dataField);
    }
    return [dataSize, dataFields];
  },
};

export function deserializePrefix(
  prefix: ToolboxIdlTypePrefix,
  data: Buffer,
  dataOffset: number,
): [number, number] {
  return [
    prefix.size,
    prefix.traverse(deserializePrefixVisitor, data, dataOffset),
  ];
}

let deserializePrefixVisitor = {
  u8: (data: Buffer, dataOffset: number): number => {
    return data.readUInt8(dataOffset);
  },
  u16: (data: Buffer, dataOffset: number): number => {
    return data.readUInt16LE(dataOffset);
  },
  u32: (data: Buffer, dataOffset: number): number => {
    return data.readUInt32LE(dataOffset);
  },
  u64: (data: Buffer, dataOffset: number): number => {
    return Number(data.readBigUInt64LE(dataOffset) & 0xffffffffffffn);
  },
  u128: (data: Buffer, dataOffset: number): number => {
    return Number(data.readBigUInt64LE(dataOffset) & 0xffffffffffffn);
  },
};

export function deserializePrimitive(
  primitive: ToolboxIdlTypePrimitive,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return [
    primitive.size,
    primitive.traverse(deserializePrimitiveVisitor, data, dataOffset),
  ];
}

let deserializePrimitiveVisitor = {
  u8: (data: Buffer, dataOffset: number): any => {
    return data.readUInt8(dataOffset);
  },
  u16: (data: Buffer, dataOffset: number): any => {
    return data.readUInt16LE(dataOffset);
  },
  u32: (data: Buffer, dataOffset: number): any => {
    return data.readUInt32LE(dataOffset);
  },
  u64: (data: Buffer, dataOffset: number): any => {
    return data.readBigUInt64LE(dataOffset);
  },
  u128: (data: Buffer, dataOffset: number): any => {
    let low = data.readBigUInt64LE(dataOffset);
    let high = data.readBigUInt64LE(dataOffset + 8);
    return low | (high << 64n);
  },
  i8: (data: Buffer, dataOffset: number): any => {
    return data.readInt8(dataOffset);
  },
  i16: (data: Buffer, dataOffset: number): any => {
    return data.readInt16LE(dataOffset);
  },
  i32: (data: Buffer, dataOffset: number): any => {
    return data.readInt32LE(dataOffset);
  },
  i64: (data: Buffer, dataOffset: number): any => {
    return data.readBigInt64LE(dataOffset);
  },
  i128: (data: Buffer, dataOffset: number): any => {
    let low = data.readBigUInt64LE(dataOffset);
    let high = data.readBigInt64LE(dataOffset + 8);
    return low | (high << 64n);
  },
  f32: (data: Buffer, dataOffset: number): any => {
    return data.readFloatLE(dataOffset);
  },
  f64: (data: Buffer, dataOffset: number): any => {
    return data.readDoubleLE(dataOffset);
  },
  bool: (data: Buffer, dataOffset: number): any => {
    return data.readUInt8(dataOffset) != 0;
  },
  pubkey: (data: Buffer, dataOffset: number): any => {
    return new PublicKey(data.subarray(dataOffset, dataOffset + 32)).toBase58();
  },
};
