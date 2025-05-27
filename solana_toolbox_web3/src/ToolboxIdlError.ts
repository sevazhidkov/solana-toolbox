import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlTypeFlat } from './ToolboxIdlTypeFlat';
import { parse, parseObjectIsPossible } from './ToolboxIdlTypeFlat.parse';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlError {
  public name: string;
  public code: number;
  public msg: string;

  constructor(value: { name: string; code: number; msg: string }) {
    this.name = value.name;
    this.code = value.code;
    this.msg = value.msg;
  }

  public static tryParse(idlErrorName: string, idlError: any): ToolboxIdlError {
    if (ToolboxUtils.isNumber(idlError)) {
      return new ToolboxIdlError({
        name: idlErrorName,
        code: idlError,
        msg: '',
      });
    }
    if (ToolboxUtils.isObject(idlError)) {
      let code = ToolboxUtils.expectNumber(idlError['code']);
      let msg = ToolboxUtils.expectString(idlError['msg'] ?? '');
      return new ToolboxIdlError({
        name: idlErrorName,
        code,
        msg,
      });
    }
    throw new Error('Unparsable error (expected an object or number)');
  }
}
