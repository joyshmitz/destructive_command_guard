## Objective

Define ast-grep/tree-sitter patterns to detect dangerous Perl constructs within heredoc bodies.

## Why This Matters

Perl heredocs are a classic attack vector because:
1. Perl is often used for system administration scripts
2. system(), exec(), backticks are core features
3. Perl's "TIMTOWTDI" philosophy means many ways to do dangerous things
4. Perl heredocs have complex quoting semantics

## Pattern Categories to Define

### Shell Execution
- system() and exec() calls
- Backtick commands `cmd`
- qx// operator (equivalent to backticks)
- open() with pipe syntax (open FH, "|cmd" or "cmd|")
- IPC::Open2, IPC::Open3

### Filesystem Operations
- unlink() for file deletion
- rmdir() for directory removal
- File::Path::rmtree
- rename() and link() for file manipulation

### Dangerous Built-ins
- eval() for code execution
- do EXPR for executing files
- require/use with dynamic paths
- AUTOLOAD abuse

### Regular Expression Dangers
- /e modifier (eval in regex replacement)
- Regex denial of service patterns
- (?{code}) embedded code in regex

### Process Control
- kill() on processes
- fork() and wait()
- alarm() and signal handlers

### Data Handling
- Two-argument open() (security risk)
- Reading from tainted input
- LWP/HTTP::Tiny for network access

## Implementation Notes

Perl is notoriously hard to parse correctly. Challenges include:
- Context-dependent syntax
- Sigils ($, @, %, *)
- Barewords vs strings vs subroutines
- Regular expression complexity
- Here-doc quoting variations

tree-sitter-perl exists but may have limitations for complex Perl.

## Test Cases

Perl-specific tests:
- Various quoting mechanisms
- Regex with embedded code
- open() variants
- Sigil interpolation

## Dependencies

- Pattern library structure design
- ast-grep invocation layer
