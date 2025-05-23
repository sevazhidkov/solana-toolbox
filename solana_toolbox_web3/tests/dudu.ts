import { greet } from '../src/index';

describe('greet', () => {
  it('returns a greeting', () => {
    expect(greet('Vincent')).toBe('Hello, Vincent!');
  });
});
