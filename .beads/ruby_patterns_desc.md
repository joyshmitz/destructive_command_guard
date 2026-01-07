## Objective

Define ast-grep/tree-sitter patterns to detect dangerous Ruby constructs within heredoc bodies.

## Why This Matters

Ruby heredocs are dangerous because:
1. Ruby has powerful metaprogramming (eval, define_method, method_missing)
2. Backticks and system() provide easy shell access
3. FileUtils module has destructive filesystem operations
4. Ruby's flexibility makes static analysis challenging

## Pattern Categories to Define

### Shell Execution
- Backtick commands: `dangerous command`
- system(), exec(), spawn() calls
- %x{} syntax for shell commands
- Open3 module usage
- IO.popen for process execution

### Filesystem Operations
- FileUtils.rm_rf, FileUtils.remove_dir
- File.delete, File.unlink
- Dir.rmdir, Dir.delete
- Pathname#rmtree

### Metaprogramming Dangers
- eval() and instance_eval
- send() and public_send() with dynamic methods
- define_method with external input
- const_get with dynamic names
- method_missing abuse

### Kernel Methods
- Kernel.exit!, Kernel.abort
- Kernel.load, Kernel.require with dynamic paths
- Kernel.fork and process manipulation

### Data Exfiltration
- Net::HTTP requests
- open-uri with external URLs
- Socket operations

## Implementation Notes

Ruby parsing challenges:
- Multiple string syntaxes (', ", %, heredocs)
- Blocks and procs as arguments
- Method calls without parentheses
- Symbol to proc (&:method_name)
- Duck typing makes type inference hard

Use tree-sitter-ruby for parsing.

## Test Cases

Ruby-specific test cases:
- Block syntax variations
- Method chaining
- Metaprogramming patterns
- Different string quoting styles

## Dependencies

- Pattern library structure design
- ast-grep invocation layer
