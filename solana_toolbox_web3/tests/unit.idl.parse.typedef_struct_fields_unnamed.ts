import {
  ToolboxIdlTypedef,
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypePrefix,
  ToolboxIdlTypePrimitive,
} from '../src';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create IDLs using different shortened formats
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    types: {
      MyStruct: {
        fields: [
          { type: 'u8' },
          { type: 'u64' },
          { type: 'string' },
          { type: ['u8'] },
          { type: { vec: 'u8' } },
          { type: ['u32', 4] },
          { type: { array: ['u32', 4] } },
          { type: { fields: [] } },
          { type: { variants: [] } },
          { type: 'Other' },
          { type: { defined: 'Other' } },
          { type: { defined: { name: 'Other' } } },
          { type: { generic: 'G' } },
          { type: { option: 'u8' } },
          { type: { option32: 'u8' } },
          { type: { fields: [] }, docs: ['Hello'] },
        ],
      },
    },
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    types: {
      MyStruct: {
        fields: [
          'u8',
          'u64',
          'string',
          ['u8'],
          { vec: 'u8' },
          ['u32', 4],
          { array: ['u32', 4] },
          { fields: [] },
          { variants: [] },
          'Other',
          { defined: 'Other' },
          { defined: { name: 'Other' } },
          { generic: 'G' },
          { option: 'u8' },
          { option32: 'u8' },
          { docs: ['Hello'], fields: [] },
        ],
      },
    },
  });
  // Asser that the two notations are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  // Assert that the content is correct
  expect(idlProgram1.typedefs.get('MyStruct')).toStrictEqual(
    new ToolboxIdlTypedef({
      name: 'MyStruct',
      docs: undefined,
      serialization: undefined,
      repr: undefined,
      generics: [],
      typeFlat: ToolboxIdlTypeFlat.struct({
        fields: ToolboxIdlTypeFlatFields.unnamed([
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U64),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.string({
              prefix: ToolboxIdlTypePrefix.U32,
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.array({
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
              length: ToolboxIdlTypeFlat.const({ literal: 4 }),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.array({
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
              length: ToolboxIdlTypeFlat.const({ literal: 4 }),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.struct({
              fields: ToolboxIdlTypeFlatFields.nothing(),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.enum({
              prefix: ToolboxIdlTypePrefix.U8,
              variants: [],
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.generic({
              symbol: 'G',
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.option({
              prefix: ToolboxIdlTypePrefix.U8,
              content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            docs: undefined,
            content: ToolboxIdlTypeFlat.option({
              prefix: ToolboxIdlTypePrefix.U32,
              content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            docs: ['Hello'],
            content: ToolboxIdlTypeFlat.nothing(),
          },
        ]),
      }),
    }),
  );
});
