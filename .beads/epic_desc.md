# Epic: ast-grep Integration for Heredoc Detection

## Problem Statement

AI coding agents sometimes attempt to bypass destructive command guards by embedding dangerous commands inside heredoc scripts. The current dcg implementation only examines the top-level command, missing destructive patterns hidden within inline scripts.

### Example Attack Vectors

An attacker might use Python, Bash, Node.js, or other language heredocs to embed dangerous operations that slip past the quick-reject filter. The top-level command appears benign (e.g., "python3 << EOF") while the heredoc body contains destructive operations.

## Solution Overview

Integrate ast-grep (or tree-sitter directly) to:
1. Detect heredoc patterns in commands
2. Extract embedded script content
3. Parse the script according to its detected language
4. Check for destructive patterns within the parsed AST
5. Block if dangerous patterns are found

## Why ast-grep?

ast-grep uses tree-sitter for parsing, providing:
- **Structural awareness**: Understands code structure, not just text patterns
- **Language support**: Handles Python, Bash, JavaScript, TypeScript, Ruby, Perl, and many more
- **Pattern matching**: Powerful AST pattern matching syntax
- **Battle-tested**: Used in production for large-scale code search and refactoring

## Key Technical Challenges

1. **Heredoc Syntax Variants**: Many forms exist (<<, <<-, <<<, <<~, quoted vs unquoted delimiters)
2. **Language Detection**: Must infer language from command prefix, shebang, or heuristics
3. **Performance**: Every command passes through dcg; parsing must be fast
4. **Pattern Library**: Need comprehensive patterns per language for destructive operations
5. **Obfuscation**: Attackers might use encoding, string concatenation, or indirect execution

## Success Criteria

- Detect and block heredoc-embedded destructive commands
- Minimal latency impact (under 10ms for heredoc detection, under 50ms for full parsing)
- Support Python, Bash, JavaScript/TypeScript, Ruby, Perl at minimum
- Configurable via pack system (heredoc scanning can be enabled/disabled)
- Comprehensive test coverage for bypass attempts

## Architecture Decision

We will evaluate two integration approaches:
1. **External binary**: Call ast-grep CLI and parse JSON output
2. **Library integration**: Use tree-sitter Rust bindings directly

The decision will be made in the research phase based on:
- Performance benchmarks
- Dependency complexity
- Maintenance burden
- Pattern expressiveness

## Out of Scope (for initial implementation)

- Scanning files referenced in commands (e.g., "bash script.sh")
- Deep obfuscation detection (base64, rot13, etc.)
- Network-based command retrieval ("curl ... | bash" style)
- Recursive heredoc nesting
