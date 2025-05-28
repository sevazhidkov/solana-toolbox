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
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';

let serializer = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    serialize(self.content, value, data, deserializable);
  },
  pod: (
    self: ToolboxIdlTypeFullPod,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    serialize(self.content, value, data, deserializable);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    if (value === null) {
      data.push(Buffer.from([0]));
      return;
    }
    data.push(Buffer.from([1]));
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
      data.push(self.prefix.toBuffer(array.length));
    }
    array.forEach((item: any) => {
      serialize(self.items, item, data, deserializable);
    });
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    let array = ToolboxUtils.expectArray(value);
    if (array.length != self.length) {
      throw new Error('Expected an array of size: ' + self.length); // TODO - better error handling
    }
    array.forEach((item: any) => {
      serialize(self.items, item, data, deserializable);
    });
  },
  string: (
    self: ToolboxIdlTypeFullString,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {
    let string = ToolboxUtils.expectString(value);
    if (deserializable) {
      data.push(self.prefix.toBuffer(string.length));
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
  enum: (self: ToolboxIdlTypeFullEnum, value: any, data: Buffer[]) => {
    if (ToolboxUtils.isNumber(value)) {
    }
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {},
  const: (
    self: ToolboxIdlTypeFullConst,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {},
  primitive: (
    self: ToolboxIdlTypePrimitive,
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {},
};

let serializerFields = {
  named: (
    self: ToolboxIdlTypeFullFieldNamed[],
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {},
  unnamed: (
    self: ToolboxIdlTypeFullFieldUnnamed[],
    value: any,
    data: Buffer[],
    deserializable: boolean,
  ) => {},
};

function serialize(
  typeFull: ToolboxIdlTypeFull,
  value: any,
  data: Buffer[],
  deserializable: boolean,
) {
  typeFull.traverse(serializer, value, data, deserializable);
}

function serializeFields(
  typeFullFields: ToolboxIdlTypeFullFields,
  value: any,
  data: Buffer[],
  deserializable: boolean,
) {
  typeFullFields.traverse(serializeFields, value, data, deserializable);
}
