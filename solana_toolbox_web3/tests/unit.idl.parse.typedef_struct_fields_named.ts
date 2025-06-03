import {
  ToolboxIdlTypedef,
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypePrefix,
  ToolboxIdlTypePrimitive,
} from '../src';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    types: {
      MyStruct: {
        fields: [
          { name: 'u8', type: 'u8' },
          { name: 'u64', type: 'u64' },
          { name: 'string', type: 'string' },
          { name: 'vec1_u8', type: ['u8'] },
          { name: 'vec2_u8', type: { vec: 'u8' } },
          { name: 'vec1_vec_u8', type: [['u8']] },
          { name: 'vec2_vec_u8', type: [{ vec: 'u8' }] },
          { name: 'array1_u32_4', type: ['u32', 4] },
          { name: 'array2_u32_4', type: { array: ['u32', 4] } },
          { name: 'struct1', type: { fields: [] } },
          { name: 'struct2', fields: [] },
          { name: 'enum1', type: { variants: [] } },
          { name: 'enum2', variants: [] },
          { name: 'defined1', defined: 'Other' },
          { name: 'defined2', defined: { name: 'Other' } },
          { name: 'defined3', type: { defined: 'Other' } },
          { name: 'defined4', type: { defined: { name: 'Other' } } },
          { name: 'option1_f32', option: 'f32' },
          { name: 'option2_f32', type: { option: 'f32' } },
          { name: 'generic1', generic: 'G' },
          { name: 'generic2', type: { generic: 'G' } },
          { name: 'docs', type: 'u8', docs: ['Hello'] },
        ],
      },
    },
  });
  // Assert that the content is correct
  expect(idlProgram.typedefs.get('MyStruct')).toStrictEqual(
    new ToolboxIdlTypedef({
      name: 'MyStruct',
      docs: undefined,
      serialization: undefined,
      repr: undefined,
      generics: [],
      typeFlat: ToolboxIdlTypeFlat.struct({
        fields: ToolboxIdlTypeFlatFields.named([
          {
            name: 'u8',
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
          },
          {
            name: 'u64',
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U64),
          },
          {
            name: 'string',
            docs: undefined,
            content: ToolboxIdlTypeFlat.string({
              prefix: ToolboxIdlTypePrefix.U32,
            }),
          },
          {
            name: 'vec1_u8',
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            name: 'vec2_u8',
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
            }),
          },
          {
            name: 'vec1_vec_u8',
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.vec({
                prefix: ToolboxIdlTypePrefix.U32,
                items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
              }),
            }),
          },
          {
            name: 'vec2_vec_u8',
            docs: undefined,
            content: ToolboxIdlTypeFlat.vec({
              prefix: ToolboxIdlTypePrefix.U32,
              items: ToolboxIdlTypeFlat.vec({
                prefix: ToolboxIdlTypePrefix.U32,
                items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
              }),
            }),
          },
          {
            name: 'array1_u32_4',
            docs: undefined,
            content: ToolboxIdlTypeFlat.array({
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
              length: ToolboxIdlTypeFlat.const({ literal: 4 }),
            }),
          },
          {
            name: 'array2_u32_4',
            docs: undefined,
            content: ToolboxIdlTypeFlat.array({
              items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
              length: ToolboxIdlTypeFlat.const({ literal: 4 }),
            }),
          },
          {
            name: 'struct1',
            docs: undefined,
            content: ToolboxIdlTypeFlat.struct({
              fields: ToolboxIdlTypeFlatFields.nothing(),
            }),
          },
          {
            name: 'struct2',
            docs: undefined,
            content: ToolboxIdlTypeFlat.struct({
              fields: ToolboxIdlTypeFlatFields.nothing(),
            }),
          },
          {
            name: 'enum1',
            docs: undefined,
            content: ToolboxIdlTypeFlat.enum({
              prefix: ToolboxIdlTypePrefix.U8,
              variants: [],
            }),
          },
          {
            name: 'enum2',
            docs: undefined,
            content: ToolboxIdlTypeFlat.enum({
              prefix: ToolboxIdlTypePrefix.U8,
              variants: [],
            }),
          },
          {
            name: 'defined1',
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            name: 'defined2',
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            name: 'defined3',
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            name: 'defined4',
            docs: undefined,
            content: ToolboxIdlTypeFlat.defined({
              name: 'Other',
              generics: [],
            }),
          },
          {
            name: 'option1_f32',
            docs: undefined,
            content: ToolboxIdlTypeFlat.option({
              prefix: ToolboxIdlTypePrefix.U8,
              content: ToolboxIdlTypeFlat.primitive(
                ToolboxIdlTypePrimitive.F32,
              ),
            }),
          },
          {
            name: 'option2_f32',
            docs: undefined,
            content: ToolboxIdlTypeFlat.option({
              prefix: ToolboxIdlTypePrefix.U8,
              content: ToolboxIdlTypeFlat.primitive(
                ToolboxIdlTypePrimitive.F32,
              ),
            }),
          },
          {
            name: 'generic1',
            docs: undefined,
            content: ToolboxIdlTypeFlat.generic({
              symbol: 'G',
            }),
          },
          {
            name: 'generic2',
            docs: undefined,
            content: ToolboxIdlTypeFlat.generic({
              symbol: 'G',
            }),
          },
          {
            name: 'docs',
            docs: ['Hello'],
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
          },
        ]),
      }),
    }),
  );
});
