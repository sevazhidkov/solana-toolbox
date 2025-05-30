import { sha256 } from 'sha.js';

export class ToolboxUtils {
  public static isObject(value: any): boolean {
    return typeof value === 'object' && !Array.isArray(value) && value !== null;
  }

  public static isArray(value: any): boolean {
    return Array.isArray(value);
  }

  public static isString(value: any): boolean {
    return typeof value === 'string' || value instanceof String;
  }

  public static isNumber(value: any): boolean {
    return typeof value === 'number';
  }

  public static isBoolean(value: any): boolean {
    return typeof value === 'boolean';
  }

  public static expectObject(value: any): Record<string, any> {
    if (!ToolboxUtils.isObject(value)) {
      throw new Error('Expected an object');
    }
    return value;
  }

  public static expectArray(value: any): any[] {
    if (!ToolboxUtils.isArray(value)) {
      throw new Error('Expected an array');
    }
    return value;
  }

  public static expectString(value: any): string {
    if (!ToolboxUtils.isString(value)) {
      throw new Error('Expected a string');
    }
    return value;
  }

  public static expectNumber(value: any): number {
    if (!ToolboxUtils.isNumber(value)) {
      throw new Error('Expected a number');
    }
    return value;
  }

  public static expectBoolean(value: any): boolean {
    if (!ToolboxUtils.isBoolean(value)) {
      throw new Error('Expected a boolean');
    }
    return value;
  }

  public static convertToSnakeCase(value: string) {
    return value
      .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
      .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
      .toLowerCase();
  }

  public static discriminator(value: string) {
    return Array.from(new sha256().update(value).digest()).slice(0, 8);
  }
}
