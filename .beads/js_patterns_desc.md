## Objective

Define ast-grep/tree-sitter patterns to detect dangerous JavaScript/Node.js constructs within heredoc bodies.

## Why This Matters

Node.js heredocs (node -e, node <<EOF) are a significant attack vector because:
1. Node has full filesystem and process access via built-in modules
2. npm/npx can execute arbitrary packages
3. child_process module provides shell execution capabilities

## Pattern Categories to Define

### Filesystem Operations
- fs.rmSync, fs.rmdirSync with recursive option
- fs.unlinkSync on critical paths
- fs.writeFileSync overwriting system files
- rimraf and similar destructive packages

### Process Execution
- child_process.exec, execSync, spawn, spawnSync
- Commands piped to shell interpreters
- process.kill on system processes

### Dangerous Requires/Imports
- require('child_process')
- Dynamic requires with user input
- import() with untrusted paths

### Network Exfiltration
- http/https requests to unknown endpoints
- fs.readFileSync followed by network calls
- Buffer manipulations for data encoding

### Package Manager Abuse
- Requiring packages that execute on install
- npx with arbitrary package names
- Global installs of untrusted packages

## Implementation Notes

JavaScript patterns need to handle:
- CommonJS vs ES modules syntax
- async/await patterns
- Promise chains
- Destructuring in imports
- Template literals
- eval() and Function() constructors

Use tree-sitter-javascript for parsing. Consider typescript patterns separately.

## Test Cases

Each pattern needs tests for:
- Various import styles (require, import, dynamic import)
- Async vs sync API variants
- Method chaining patterns
- Callback vs Promise vs async/await styles

## Dependencies

- Pattern library structure design
- ast-grep invocation layer
