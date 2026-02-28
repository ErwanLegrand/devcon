# Claude Code Instructions

## Git Commits

- **Never use command substitution** (`$(cat <<'EOF'...EOF)`) for commit messages — this pattern requires user validation at every commit.
- Provide commit messages **directly** via a temp file:
  ```bash
  printf 'subject line\n\nbody paragraph' > /tmp/commit_msg && git commit -F /tmp/commit_msg
  ```
- For single-line messages, `-m "message"` is fine.
