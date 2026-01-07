## Objective

Embed structural analysis directly in the dcg binary using Rust crates rather than shelling out to external tools.

## Architecture Decision

**DECISION: Embed tree-sitter and ast-grep-core in Rust binary**

### Why Not External Binaries?

External process invocation (ripgrep CLI, ast-grep CLI) has unacceptable overhead:
- Process spawn: 5-20ms
- IPC serialization: 1-5ms
- Output parsing: 1-2ms
- Total: 10-50ms per command

Since dcg runs on EVERY bash command, this latency is unacceptable. Users would notice slowdown.

### Why Embedded Rust Crates?

1. **tree-sitter** is available as a Rust crate with excellent performance
2. **ast-grep is written in Rust** - we may be able to use ast-grep-core directly
3. **Language grammars** are available as Rust crates (tree-sitter-bash, etc.)
4. **Sub-millisecond latency** when everything is compiled in
5. **Single binary** - no "install ast-grep" requirement

### Implementation Strategy

#### Phase 1: tree-sitter Integration
```toml
[dependencies]
tree-sitter = "0.22"
tree-sitter-bash = "0.21"
tree-sitter-python = "0.21"
tree-sitter-javascript = "0.21"
# ... etc
```

#### Phase 2: Pattern Matching

Two options:

**Option A: tree-sitter Queries**
Use tree-sitter's built-in query language (S-expressions):
```scheme
(command
  name: (command_name) @cmd
  argument: (word) @arg
  (#eq? @cmd "rm")
  (#match? @arg "-rf"))
```

Pros: Built into tree-sitter, well-documented
Cons: Verbose, less expressive than ast-grep patterns

**Option B: ast-grep-core Crate**
If ast-grep publishes a library crate, use it:
```rust
use ast_grep_core::{Pattern, Matcher};

let pattern = Pattern::new("os.system($CMD)")?;
let matches = pattern.find_all(&python_ast);
```

Pros: More expressive patterns, proven matching logic
Cons: May not be published as separate crate, may need vendoring

**Option C: Vendor ast-grep Code**
If ast-grep-core isn't available as a crate:
1. Clone ast-grep repo
2. Extract relevant modules (pattern matching, not CLI)
3. Vendor into our codebase
4. Maintain minimal fork

Pros: Full control
Cons: Maintenance burden

#### Recommendation: Try in order A → B → C

1. Start with tree-sitter queries - simplest, no extra deps
2. If queries are too limiting, try ast-grep-core crate
3. If not available, vendor minimal ast-grep code

### Performance Budget

Target latency for heredoc analysis:
- Heredoc detection (regex): <0.5ms
- Content extraction: <0.1ms
- tree-sitter parse: <2ms (for typical heredoc size)
- Pattern matching: <1ms
- **Total: <5ms worst case**

### Binary Size Considerations

Each tree-sitter grammar adds ~500KB-2MB to binary size.
6 languages × 1MB average = ~6MB added

Mitigation:
- Compile grammars with size optimization
- Consider optional features (--features python,bash,js)
- Default: most common languages (bash, python, js)
- Extended: all languages

### Research Tasks Update

This decision affects:
- git_safety_guard-b45: Research should focus on ast-grep's Rust internals
- git_safety_guard-2j3: This becomes primary focus - tree-sitter Rust bindings
- git_safety_guard-5ib: ADR should document this embedded approach

## Success Criteria

- All parsing embedded in single binary
- No external tool dependencies
- Heredoc analysis completes in <5ms
- Binary size increase <10MB
- Pattern matching expressiveness sufficient for all use cases
