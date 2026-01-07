## Objective

Design and implement strategies to dramatically reduce false positives in command blocking, especially for documentation and string arguments.

## The Core Problem

The current regex-based guard suffers from context blindness. It sees:
```
bd create --description="Pattern blocks rm -rf commands"
```

And matches "rm -rf" without understanding it's:
1. Inside a quoted string
2. An argument to a documentation tool
3. NOT actually being executed

This creates severe workflow disruption when trying to document the very patterns we're blocking.

## Why This Is Critical

False positives are arguably WORSE than false negatives:
- False negative: A dangerous command might slip through (rare, recoverable with backups)
- False positive: Blocks legitimate work, forces workarounds, erodes trust in the tool
- Repeated false positives lead users to disable the guard entirely

The guard must be TRUSTED to make intelligent decisions, not blindly pattern-match text.

## Solution Approaches

### 1. Command Structure Analysis (Primary Approach)

Parse the top-level command with tree-sitter-bash to understand structure:
- Identify command name (bd, git, echo, etc.)
- Identify argument positions (which are options vs values)
- Identify quoting context (single, double, unquoted)
- Only apply destructive patterns to EXECUTABLE positions

Example analysis:
```
bd create --title="..." --description="rm -rf pattern docs"
         ^command       ^option         ^string value (NOT executed)
```

### 2. Safe Command Registry

Maintain a list of commands that take non-executable string arguments:
- bd create, bd update (--description, --title)
- git commit (-m), git tag (-m)
- echo, printf (arguments are printed, not executed)
- grep, rg (pattern arguments)

For these commands, don't apply destructive patterns to their string arguments.

### 3. Execution Context Detection

Distinguish between:
- **Direct execution**: The string IS the command (`bash dangerous_cmd`)
- **String literal**: The string is DATA passed to a command (`bd --desc="..."`)
- **Heredoc body**: Requires language-specific analysis (the whole point of ast-grep)

### 4. Two-Phase Analysis

1. **Quick structural check**: Parse command structure, identify context
2. **Pattern matching**: Only apply patterns to executable contexts
3. **Deep analysis**: For heredocs/complex cases, use ast-grep

### 5. Confidence Scoring

Instead of binary block/allow:
- High confidence dangerous: Block immediately
- Medium confidence: More thorough analysis
- Low confidence (looks like documentation): Allow with optional warning

## Implementation Strategy

1. Add tree-sitter-bash as a dependency (if not using ast-grep CLI)
2. Parse incoming commands to identify structure
3. Create ExecutionContext enum: Direct, StringArg, Heredoc, PipeTarget
4. Only apply destructive patterns when context is Direct, Heredoc, or PipeTarget
5. For StringArg context with safe parent commands, skip pattern matching

## Test Cases

Essential false positive tests:
- `bd create --description="This blocks rm -rf"` → ALLOW
- `git commit -m "Fix rm -rf pattern matching"` → ALLOW
- `echo "example: git reset --hard"` → ALLOW
- `grep "rm -rf" patterns.txt` → ALLOW

Essential true positive tests (should still block):
- `rm -rf /tmp/*` → BLOCK
- `bash -c "rm -rf /"` → BLOCK
- `python3 << 'EOF'
  import os; os.system("rm -rf /")
  EOF` → BLOCK (via heredoc analysis)

## Success Criteria

- Zero false positives for documentation workflows (bd, git commit -m)
- Zero false positives for string pattern searches (grep, rg)
- Maintain blocking of actual dangerous commands
- Sub-5ms overhead for structural analysis

## Dependencies

- Heredoc detection strategy (shares parsing infrastructure)
- May influence choice of ast-grep vs tree-sitter-rust
