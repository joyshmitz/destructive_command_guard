## Objective

Create comprehensive test coverage for heredoc detection and AST-based pattern matching, including unit tests, integration tests, and bypass attempt tests.

## Test Categories

### 1. Unit Tests

#### Heredoc Detection Tests
- Bash heredoc variants: <<EOF, <<-EOF, <<<, <<'EOF', <<"EOF"
- Python multiline strings as heredocs: python3 -c '''...'''
- Node.js: node -e '...', node <<EOF
- Perl: perl -e '...', perl <<'END'
- Ruby: ruby -e '...', ruby <<~RUBY

#### Language Detection Tests
- Detection from command prefix (python3, node, ruby, perl, bash)
- Detection from shebang (#!/usr/bin/env python3)
- Detection from content heuristics (import statements, require(), etc.)
- Fallback behavior for unknown languages

#### Pattern Matching Tests
- Each destructive pattern per language
- Each safe pattern (things that look dangerous but aren't)
- Edge cases in quoting and escaping
- Variable interpolation handling

### 2. Integration Tests

#### Pipeline Tests
- Full flow from JSON input to block/allow decision
- Heredoc commands correctly trigger deep analysis
- Non-heredoc commands still use fast path
- Performance within latency budget

#### Error Handling Tests
- Malformed heredoc syntax
- Unparseable content (binary, corrupted)
- Unknown languages fall back gracefully
- Timeout handling

### 3. Bypass Attempt Tests (Security Focus)

These are CRITICAL - they test the actual attack vectors we're trying to block.

#### Encoding Bypasses
- Base64 encoded commands in heredocs
- Hex encoding
- Unicode obfuscation
- String concatenation to build dangerous strings

#### Indirection Bypasses
- Variable expansion: CMD="rm"; ${CMD} -rf
- Array expansion: arr=(rm -rf /); "${arr[@]}"
- Command substitution: $(echo rm) -rf
- Eval-based: eval "dangerous command"

#### Language-Specific Bypasses
- Python: __import__, exec(), compile()
- JavaScript: eval(), Function(), require()
- Ruby: send(), instance_eval(), Kernel.`
- Perl: eval, do EXPR, qx//

#### Heredoc Nesting
- Heredoc containing another heredoc
- Heredoc with escaped delimiters
- Heredoc across multiple commands (pipelines)

### 4. False Positive Tests

Ensure we DON'T block legitimate uses:
- Documentation containing command examples
- Grep patterns searching for dangerous commands
- Test files containing example commands
- Config files with commented dangerous commands
- bd create with descriptions about dangerous patterns

### 5. Performance Tests

- Latency benchmarks for various command types
- Memory usage under load
- Concurrent command checking
- Large heredoc handling

## Test Infrastructure

### Test Data Directory
```
tests/
  fixtures/
    heredocs/
      bash/
      python/
      javascript/
      ruby/
      perl/
    bypass_attempts/
    false_positives/
```

### Test Macros
```rust
// Test that a command is blocked
assert_blocked!("python3 << 'EOF'\nimport os; os.system('rm -rf /')\nEOF");

// Test that a command is allowed
assert_allowed!("python3 << 'EOF'\nprint('hello')\nEOF");

// Test specific pattern match
assert_pattern_matches!("python", "os.system($CMD)", "os.system('rm -rf /')");
```

## Success Criteria

- 100% coverage of documented attack vectors
- Zero false positives in false_positive test suite
- All performance tests pass within budget
- Clear documentation of what each test validates

## Dependencies

- Integration pipeline must be complete
- All language patterns must be defined
- Performance benchmarking infrastructure
