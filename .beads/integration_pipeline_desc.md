## Objective

Integrate all heredoc detection components into the main dcg check pipeline, creating a seamless flow from command input to block/allow decision.

## Pipeline Architecture

```
JSON Input → Parse → Quick Reject → Normalize →
  → Heredoc Detection → [if heredoc found] →
    → Extract Content → Detect Language → Parse with ast-grep →
    → Apply Language Patterns → Block Decision
  → [no heredoc] → Existing Pattern Matching → Block Decision
```

## Integration Points

### 1. Entry Point Hook

Modify main.rs check flow to:
1. After quick reject passes, check for heredoc indicators
2. If heredoc found, branch to heredoc analysis path
3. If no heredoc, continue with existing pattern matching
4. Merge results from both paths

### 2. Heredoc Detection Integration

Insert heredoc detection before main pattern matching:
```rust
fn check_command(cmd: &str) -> CheckResult {
    // Quick reject (existing)
    if global_quick_reject(cmd) {
        return CheckResult::allowed();
    }

    // NEW: Heredoc detection
    if let Some(heredoc_result) = check_heredoc(cmd) {
        if heredoc_result.blocked {
            return heredoc_result;
        }
    }

    // Existing pattern matching
    REGISTRY.check_command(cmd, &enabled_packs)
}
```

### 3. Performance Budget

Total additional latency budget: 50ms worst case
- Heredoc detection regex: <2ms
- Content extraction: <1ms
- Language detection: <1ms
- ast-grep invocation: <40ms (external process)
- Pattern matching: <5ms

If ast-grep is too slow, fall back to regex patterns for the heredoc body.

### 4. Feature Flag

Add heredoc scanning to pack system:
- New pack: "heredoc" or integrate into "core"
- Can be enabled/disabled via configuration
- Default: enabled for new installations

### 5. Error Handling

Graceful degradation:
- ast-grep not installed → warn, fall back to regex
- ast-grep timeout → allow command, log warning
- Parse error → allow command, log for debugging
- Unknown language → use generic patterns or allow

## Configuration

New config options:
```toml
[heredoc]
enabled = true
timeout_ms = 50
fallback_on_error = true
languages = ["python", "bash", "javascript", "typescript", "ruby", "perl"]
```

## Testing Integration

Integration tests should cover:
- Normal commands (no heredoc) still work fast
- Heredoc commands get analyzed
- Pattern matches in heredocs block correctly
- Safe heredocs pass through
- Error paths handle gracefully

## Dependencies

- Heredoc syntax detection
- Content extraction
- Language detection
- ast-grep invocation layer
- All language patterns
