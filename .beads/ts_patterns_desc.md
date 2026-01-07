## Objective

Define ast-grep/tree-sitter patterns to detect dangerous TypeScript constructs within heredoc bodies.

## Why This Matters

TypeScript heredocs (ts-node, tsx, npx ts-node) share JavaScript's risks plus:
1. Type erasure can hide dangerous operations behind clean interfaces
2. Decorators and metaprogramming can obscure behavior
3. Many TypeScript users assume type safety means runtime safety (it doesn't)

## Pattern Categories to Define

### All JavaScript Patterns (Inherited)
All patterns from the JavaScript task apply here since TypeScript compiles to JavaScript.

### TypeScript-Specific Patterns
- any type casts that hide dangerous operations
- Type assertions (as unknown as DangerousType)
- @ts-ignore comments preceding dangerous code
- Non-null assertions (!) on potentially null file handles

### Decorator Abuse
- Decorators that execute arbitrary code
- Metadata reflection for dynamic execution
- Class decorator patterns that modify behavior

### Module System
- Triple-slash directives loading external code
- Type-only imports that get erased (import type)
- Namespace merging that hides implementations

## Implementation Notes

TypeScript parsing requires tree-sitter-typescript which handles:
- Generic type syntax
- Type annotations
- Decorators
- JSX/TSX syntax variants

Consider whether to parse TypeScript directly or check the JavaScript output.

## Test Cases

Test TypeScript-specific constructs:
- Type assertions around dangerous calls
- Decorator execution order
- Generic type inference edge cases
- Module augmentation

## Dependencies

- JavaScript patterns (shared base)
- Pattern library structure design
- ast-grep invocation layer
