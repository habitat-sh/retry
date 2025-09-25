# Copilot Instructions for retry Repository

## Repository Overview

This is a Rust crate called `retry` that provides utilities for retrying operations that can fail. The crate supports both synchronous and asynchronous retry mechanisms with various delay strategies including fixed delays, exponential backoff, fibonacci sequences, and random jitter.

### Project Structure

```
retry/
├── .github/                    # GitHub-related configuration files
├── .git/                       # Git repository metadata
├── .gitignore                  # Git ignore patterns
├── .travis.yml                 # Travis CI configuration
├── .vscode/                    # VS Code settings
├── Cargo.toml                  # Rust package manifest
├── Cargo.lock                  # Dependency lock file
├── LICENSE                     # MIT license file
├── README.md                   # Project documentation
├── src/                        # Source code directory
│   ├── lib.rs                  # Main library file with retry logic
│   ├── asynchronous.rs         # Async retry implementations
│   ├── delay.rs                # Delay strategy implementations
│   └── opresult.rs             # OperationResult type definitions
└── target/                     # Rust build artifacts (generated)
    ├── CACHEDIR.TAG
    └── debug/                  # Debug build artifacts
        ├── build/              # Build script outputs
        ├── deps/               # Compiled dependencies
        ├── examples/           # Example binaries
        └── incremental/        # Incremental compilation cache
```

## Critical Instructions

### File Modification Restrictions
- **DO NOT MODIFY** any `*.codegen.go` files if present in the repository
- Always preserve existing functionality when making changes
- Follow Rust coding conventions and best practices
- Maintain backward compatibility unless explicitly requested to make breaking changes

### MCP Server Integration
When a Jira ID is provided:
1. Use the **atlassian-mcp-server** MCP server to fetch Jira issue details
2. Read the story description and acceptance criteria thoroughly
3. Understand the requirements before implementing the task
4. Ask clarifying questions if the Jira story is unclear or incomplete

### Testing Requirements
- **MANDATORY**: Create comprehensive unit test cases for all new implementations
- **MANDATORY**: Ensure code coverage remains above **80%** at all times
- Run `cargo test` to verify all tests pass
- Use `cargo tarpaulin` or similar tools to measure code coverage
- Include both positive and negative test scenarios
- Test edge cases and error conditions

### Pull Request Creation Process
When prompted to create a PR:
1. Use GitHub CLI (gh) to create a new branch with the Jira ID as the branch name
2. Commit all changes to the feature branch
3. Push changes to the remote repository
4. Create a PR using GitHub CLI
5. Add the label **"runtest:all:stable"** to the PR
6. Use HTML tags in PR description for better formatting

**GitHub CLI Commands Structure:**
```bash
# Create and switch to new branch (use Jira ID as branch name)
git checkout -b JIRA-123

# Stage and commit changes
git add .
git commit -m "feat: implement feature from JIRA-123"

# Push branch to remote
git push origin JIRA-123

# Create PR with HTML-formatted description and label
gh pr create --title "feat: implement feature from JIRA-123" \
  --body "<h3>Summary</h3><p>Brief description of changes</p><h3>Changes Made</h3><ul><li>Change 1</li><li>Change 2</li></ul>" \
  --label "runtest:all:stable"
```

### Prompt-Based Workflow
- **ALL tasks MUST be prompt-based**
- After each step, provide a clear summary of what was accomplished
- Always indicate what the next step will be
- List remaining steps to complete the task
- Ask for confirmation before proceeding to the next step: "Do you want to continue with the next step?"

## Complete Implementation Workflow

Follow this comprehensive workflow for all task implementations:

### Phase 1: Analysis and Planning
1. **Requirement Analysis**
   - If Jira ID provided, fetch issue details using atlassian-mcp-server
   - Analyze requirements and acceptance criteria
   - Identify affected components and files
   - **Prompt**: "Analyzed requirements. Next step: Design implementation approach. Continue?"

2. **Impact Assessment**
   - Review current codebase structure
   - Identify files that need modification
   - Check for potential breaking changes
   - Ensure no prohibited files (*.codegen.go) are affected
   - **Prompt**: "Completed impact assessment. Next step: Create implementation plan. Continue?"

3. **Implementation Planning**
   - Design the solution architecture
   - Plan test cases and coverage strategy
   - Identify dependencies and integration points
   - **Prompt**: "Created implementation plan. Next step: Begin coding. Continue?"

### Phase 2: Implementation
4. **Core Implementation**
   - Implement the main functionality
   - Follow Rust best practices and conventions
   - Maintain code quality and documentation
   - **Prompt**: "Completed core implementation. Next step: Add comprehensive tests. Continue?"

5. **Test Implementation**
   - Create unit tests with >80% coverage
   - Include integration tests where applicable
   - Test edge cases and error scenarios
   - Run `cargo test` to verify all tests pass
   - **Prompt**: "Implemented tests with >80% coverage. Next step: Code review and validation. Continue?"

### Phase 3: Validation and Documentation
6. **Code Review and Validation**
   - Review code for quality and adherence to standards
   - Verify test coverage requirements
   - Check for any regressions
   - **Prompt**: "Completed code review. Next step: Update documentation. Continue?"

7. **Documentation Updates**
   - Update README.md if needed
   - Add/update code comments and documentation
   - Update examples if functionality changed
   - **Prompt**: "Updated documentation. Next step: Create pull request. Continue?"

### Phase 4: Pull Request Creation
8. **Pull Request Preparation**
   - Create feature branch using Jira ID as branch name
   - Commit changes with descriptive messages
   - Push to remote repository
   - **Prompt**: "Prepared branch and commits. Next step: Create PR with GitHub CLI. Continue?"

9. **Pull Request Creation**
   - Use GitHub CLI to create PR with HTML-formatted description
   - Add "runtest:all:stable" label
   - Include comprehensive summary of changes
   - **Prompt**: "Created pull request successfully. Task completed!"

## Best Practices

### Code Quality
- Write idiomatic Rust code
- Use appropriate error handling patterns
- Follow the existing code style and conventions
- Add comprehensive documentation comments
- Ensure thread safety for concurrent operations

### Testing Strategy
- Unit tests for individual functions and methods
- Integration tests for complete workflows
- Property-based testing for complex algorithms
- Performance tests for critical paths
- Error condition testing

### Git Workflow
- Use meaningful commit messages following conventional commits
- Keep commits focused and atomic
- Rebase feature branches before creating PRs
- Squash commits when appropriate

### Communication
- Always ask for confirmation before major changes
- Provide clear explanations of technical decisions
- Highlight any potential risks or breaking changes
- Suggest alternative approaches when applicable

---

*This instruction file ensures comprehensive, high-quality development practices while maintaining the integrity and reliability of the retry crate.*