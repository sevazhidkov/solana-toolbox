import {
  ToolboxIdlTypedef,
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypePrefix,
} from '../src';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create IDLs using different shortened formats
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        variants: [
          { name: '77', fields: [], code: 77 },
          { name: 'Case1', fields: [] },
          { name: 'Case2', fields: [], code: 42 },
          { name: 'Case3', fields: [] },
        ],
      },
    },
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        variants: [
          { name: '77', code: 77 },
          { name: 'Case1' },
          { name: 'Case2', code: 42 },
          { name: 'Case3' },
        ],
      },
    },
  });
  let idlProgram3 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        variants: [77, 'Case1', { name: 'Case2', code: 42 }, 'Case3'],
      },
    },
  });
  let idlProgram4 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        variants: {
          '77': 77,
          Case1: 1,
          Case2: 42,
          Case3: { code: 3, fields: [] },
        },
      },
    },
  });
  // Assert that all are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  expect(idlProgram1).toStrictEqual(idlProgram3);
  expect(idlProgram1).toStrictEqual(idlProgram4);
  // Assert that the content is correct
  expect(idlProgram1.typedefs.get('MyEnum')).toStrictEqual(
    new ToolboxIdlTypedef({
      name: 'MyEnum',
      docs: undefined,
      serialization: undefined,
      repr: undefined,
      generics: [],
      typeFlat: ToolboxIdlTypeFlat.enum({
        prefix: ToolboxIdlTypePrefix.U8,
        variants: [
          {
            name: '77',
            code: 77,
            docs: undefined,
            fields: ToolboxIdlTypeFlatFields.nothing(),
          },
          {
            name: 'Case1',
            code: 1,
            docs: undefined,
            fields: ToolboxIdlTypeFlatFields.nothing(),
          },
          {
            name: 'Case2',
            code: 42,
            docs: undefined,
            fields: ToolboxIdlTypeFlatFields.nothing(),
          },
          {
            name: 'Case3',
            code: 3,
            docs: undefined,
            fields: ToolboxIdlTypeFlatFields.nothing(),
          },
        ],
      }),
    }),
  );
});
