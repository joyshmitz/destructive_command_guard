## Objective

Define ast-grep/tree-sitter patterns to detect dangerous Bash/Shell constructs within heredoc bodies.

## Why This Matters

Bash heredocs are the most common attack vector because:
1. bash -c, sh -c are extremely common command forms
2. Shell scripts can invoke ANY system command
3. Existing dcg regex patterns already target many shell commands - we need equivalent AST patterns

## Pattern Categories to Define

### File/Directory Destruction
- recursive forced removal patterns
- rmdir on critical directories
- find with delete patterns
- Wildcards in destructive contexts

### Git Destructive Operations
- hard reset patterns
- forced clean patterns
- force push to protected branches
- force branch deletion

### Permission/Ownership Changes
- chmod 777 on system paths
- chown root or changing critical file ownership
- setfacl manipulations

### System Administration
- mkfs, fdisk, dd commands on devices
- service/systemctl stop on critical services
- kill -9 on system processes
- shutdown, reboot, halt

### Data Exfiltration Indicators
- curl/wget piped to shell
- base64 encoding of sensitive files
- tar/zip of home directories or /etc

## Implementation Notes

Shell parsing is complex due to:
- Quoting rules (single, double, command substitution)
- Variable expansion
- Command substitution
- Heredocs within heredocs
- Arrays and special variables

Start with tree-sitter-bash and test each pattern against real shell scripts to validate accuracy.

## Test Cases

Each pattern needs positive tests (should match) and negative tests (should not match) covering:
- Common benign uses that look similar
- Quoting variations
- Variable indirection
- Command aliasing

## Dependencies

- Pattern library structure design (how patterns are organized)
- ast-grep invocation layer (how to run ast-grep on content)
