## Objective

Create comprehensive documentation for the heredoc detection feature, covering user configuration, pattern authoring, and security considerations.

## Documentation Deliverables

### 1. User Guide Updates

#### README.md Updates
- Add heredoc scanning to feature list
- Document new command-line options (if any)
- Update configuration examples

#### Configuration Guide
- How to enable/disable heredoc scanning
- Language-specific configuration
- Performance tuning options
- Fallback behavior settings

### 2. Pattern Authoring Guide

New document: `docs/patterns.md`

#### Pattern Syntax
- tree-sitter query syntax (if using queries)
- ast-grep pattern syntax (if using ast-grep-core)
- Examples for each supported language

#### Adding New Patterns
- Step-by-step guide
- Testing requirements
- Performance considerations
- Review checklist

#### Pattern Library Reference
- Complete list of all patterns
- What each pattern detects
- Known limitations
- False positive/negative notes

### 3. Security Documentation

New document: `docs/security.md`

#### Threat Model
- Attack vectors heredoc detection addresses
- Attack vectors explicitly out of scope
- Assumptions and limitations

#### Bypass Considerations
- Known potential bypasses
- Why certain bypasses are accepted
- Defense in depth recommendations

#### Incident Response
- What to do if a command is wrongly blocked
- What to do if a dangerous command gets through
- How to report security issues

### 4. Developer Documentation

#### Architecture Overview
- Pipeline flow diagram
- Module responsibilities
- Data flow through heredoc analysis

#### API Documentation
- Internal Rust API documentation (rustdoc)
- Integration points for extending
- Error handling patterns

#### Contributing Guide Updates
- How to add new language support
- How to add new patterns
- Testing requirements for contributions

### 5. AGENTS.md Updates

Update the AI agent guidelines:
- New heredoc detection capabilities
- How to test heredoc patterns
- False positive handling guidance

## Documentation Quality Requirements

- All code examples must be tested
- All configuration examples must be valid
- Mermaid diagrams for complex flows
- Cross-references between related docs
- Version numbers where relevant

## Dependencies

- Feature implementation complete
- Test suite passing
- Performance benchmarks available
- ADR finalized
