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
  ToolboxIdlTypeFullPod,
  ToolboxIdlTypeFullString,
  ToolboxIdlTypeFullStruct,
  ToolboxIdlTypeFullTypedef,
  ToolboxIdlTypeFullVec,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

export function deserialize(
  typeFull: ToolboxIdlTypeFull,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return typeFull.traverse(deserializer, data, dataOffset, undefined);
}

let deserializer = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserialize(self.content, data, dataOffset);
  },
  pod: (
    self: ToolboxIdlTypeFullPod,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserialize(self.content, data, dataOffset);
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
    if (dataPrefix % 0xff == 0) {
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
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    let dataVariantOffset = dataOffset + dataSize;
    for (let variant of self.variants) {
      if (variant.code == dataPrefix) {
        let [dataVariantSize, dataVariant] = deserializeFields(
          variant.fields,
          data,
          dataVariantOffset,
        );
        dataSize += dataVariantSize;
        if (dataVariant === null) {
          return [dataSize, variant.name];
        }
        return [
          dataSize,
          {
            [variant.name]: dataVariant,
          },
        ];
      }
    }
    throw new Error('Could not deserialize enum with code: ' + dataPrefix);
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
    self: ToolboxIdlTypeFullConst,
    data: Buffer,
    dataOffset: number,
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
  return fields.traverse(deserializerFields, data, dataOffset, undefined);
}

let deserializerFields = {
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
      let [dataFieldSize, dataField] = deserialize(
        field.content,
        data,
        dataFieldOffset,
      );
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
      let [dataFieldSize, dataField] = deserialize(
        field.content,
        data,
        dataFieldOffset,
      );
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
    prefix.traverse(deserializerPrefix, data, dataOffset, undefined),
  ];
}

let deserializerPrefix = {
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
    return Number(data.readBigUInt64LE(dataOffset));
  },
};

export function deserializePrimitive(
  primitive: ToolboxIdlTypePrimitive,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return [
    primitive.size,
    primitive.traverse(deserializerPrimitive, data, dataOffset, undefined),
  ];
}

let deserializerPrimitive = {
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
    return Number(data.readBigUInt64LE(dataOffset));
  },
  u128: (data: Buffer, dataOffset: number): any => {
    return Number(data.readBigUInt64LE(dataOffset));
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
    return Number(data.readBigInt64LE(dataOffset));
  },
  i128: (data: Buffer, dataOffset: number): any => {
    return Number(data.readBigInt64LE(dataOffset));
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
