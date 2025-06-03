import {
  ToolboxIdlTypedef,
  ToolboxIdlTypeFlat,
  ToolboxIdlTypePrefix,
} from '../src';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create IDLs using different shortened formats
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    types: [
      {
        name: 'MyEnum',
        generics: [{ kind: 'type', name: 'A' }, { name: 'B' }],
        type: { variants: [] },
      },
    ],
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    types: [
      {
        name: 'MyEnum',
        generics: [{ kind: 'type', name: 'A' }, { name: 'B' }],
        variants: [],
      },
    ],
  });
  let idlProgram3 = ToolboxIdlProgram.tryParse({
    types: [
      {
        name: 'MyEnum',
        generics: ['A', 'B'],
        variants: [],
      },
    ],
  });
  let idlProgram4 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        generics: [{ kind: 'type', name: 'A' }, { name: 'B' }],
        type: { variants: [] },
      },
    },
  });
  let idlProgram5 = ToolboxIdlProgram.tryParse({
    types: {
      MyEnum: {
        generics: ['A', 'B'],
        variants: [],
      },
    },
  });
  // Assert that all are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  expect(idlProgram1).toStrictEqual(idlProgram3);
  expect(idlProgram1).toStrictEqual(idlProgram4);
  expect(idlProgram1).toStrictEqual(idlProgram5);
  // Assert that the content is correct
  expect(idlProgram1.typedefs.get('MyEnum')).toStrictEqual(
    new ToolboxIdlTypedef({
      name: 'MyEnum',
      docs: undefined,
      serialization: undefined,
      repr: undefined,
      generics: ['A', 'B'],
      typeFlat: ToolboxIdlTypeFlat.enum({
        prefix: ToolboxIdlTypePrefix.U8,
        variants: [],
      }),
    }),
  );
});
