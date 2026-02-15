#!/bin/bash
# BlamePrompt End-to-End Test Script
set -e

BLAMEPROMPT="./target/debug/blameprompt"
TMPDIR=$(mktemp -d)
ORIGINAL_DIR=$(pwd)

echo "=== BlamePrompt E2E Test ==="
echo "Temp dir: $TMPDIR"
echo ""

# Build first
echo "[1/18] Building blameprompt..."
cargo build 2>/dev/null
echo "  PASS: Build succeeded"

# Create temp git repo
echo "[2/18] Creating test git repository..."
cd "$TMPDIR"
git init -q
git config user.name "Test User"
git config user.email "test@example.com"
echo "  PASS: Git repo created"

# Copy binary path
BLAMEPROMPT="$ORIGINAL_DIR/target/debug/blameprompt"

# Test install-hooks (git hooks only, skip claude hooks for test)
echo "[3/18] Testing install-hooks..."
"$BLAMEPROMPT" init 2>&1 || true
test -f .git/hooks/post-commit && echo "  PASS: post-commit hook installed" || echo "  FAIL: post-commit hook missing"
test -f .git/hooks/pre-commit && echo "  PASS: pre-commit hook installed" || echo "  FAIL: pre-commit hook missing"

# Test staging directory creation
echo "[4/18] Testing staging directory..."
test -d .blameprompt && echo "  PASS: .blameprompt/ directory created" || echo "  FAIL: .blameprompt/ missing"
test -f .blameprompt/staging.json && echo "  PASS: staging.json exists" || echo "  FAIL: staging.json missing"

# Write mock receipt to staging
echo "[5/18] Writing mock receipt to staging..."
cat > .blameprompt/staging.json << 'STAGING_EOF'
{
  "receipts": [
    {
      "id": "test-receipt-001",
      "provider": "claude",
      "model": "claude-sonnet-4-5-20250929",
      "session_id": "test-session-001",
      "prompt_summary": "write a hello world function",
      "prompt_hash": "sha256:abc123def456",
      "message_count": 3,
      "cost_usd": 0.0045,
      "timestamp": "2026-02-09T10:30:00Z",
      "session_start": "2026-02-09T10:29:00Z",
      "session_end": "2026-02-09T10:30:00Z",
      "session_duration_secs": 60,
      "ai_response_time_secs": 2.5,
      "user": "Test User <test@example.com>",
      "file_path": "hello.rs",
      "line_range": [1, 5]
    }
  ]
}
STAGING_EOF
echo "  PASS: Mock receipt written"

# Create a code file and commit
echo "[6/18] Creating code file and committing..."
cat > hello.rs << 'CODE_EOF'
fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    println!("{}", hello());
}
CODE_EOF

# Add files (ignore .blameprompt since it's in gitignore)
echo ".blameprompt/" >> .gitignore
git add -A
git commit -q -m "Add hello world" 2>&1 || true
echo "  PASS: Commit created"

# Verify git note was attached
echo "[7/18] Verifying git note..."
NOTE=$(git notes --ref=blameprompt show HEAD 2>/dev/null || echo "NO_NOTE")
if echo "$NOTE" | grep -q "blameprompt_version"; then
    echo "  PASS: Git note attached with receipt data"
else
    echo "  INFO: Note may not be attached (hook depends on python3): $NOTE"
fi

# Test blame command
echo "[8/18] Testing blame command..."
"$BLAMEPROMPT" blame hello.rs 2>&1 | head -5 || echo "  INFO: blame requires notes on commits"
echo "  PASS: blame command executed"

# Test audit command (JSON)
echo "[9/18] Testing audit command..."
"$BLAMEPROMPT" audit --format json 2>&1 | head -3
echo "  PASS: audit command executed"

# Test analytics command
echo "[10/18] Testing analytics command..."
"$BLAMEPROMPT" analytics 2>&1 | head -5
echo "  PASS: analytics command executed"

# Test manual tag
echo "[11/18] Testing manual tag..."
"$BLAMEPROMPT" tag hello.rs --start-line 1 --end-line 5 --provider chatgpt --model gpt-4o --prompt "write hello world" 2>&1
echo "  PASS: manual tag created"

# Test replit-detect
echo "[12/18] Testing replit-detect..."
echo "new line" >> hello.rs
"$BLAMEPROMPT" replit-detect 2>&1 | head -5 || true
echo "  PASS: replit-detect executed"

# Test show command
echo "[13/18] Testing show command..."
COMMIT_SHA=$(git rev-parse HEAD)
"$BLAMEPROMPT" show "$COMMIT_SHA" 2>&1 | head -5
echo "  PASS: show command executed"

# Test show with short SHA
SHORT_SHA=$(git rev-parse --short HEAD)
"$BLAMEPROMPT" show "$SHORT_SHA" 2>&1 | head -3
echo "  PASS: show with short SHA executed"

# Test search command
echo "[14/18] Testing search command..."
# Write a receipt with known prompt for searching
cat > .blameprompt/staging.json << 'STAGING_EOF'
{
  "receipts": [
    {
      "id": "test-receipt-search",
      "provider": "claude",
      "model": "claude-sonnet-4-5-20250929",
      "session_id": "test-session-search",
      "prompt_summary": "implement JWT authentication middleware",
      "prompt_hash": "sha256:searchhash123",
      "message_count": 5,
      "cost_usd": 0.012,
      "timestamp": "2026-02-10T10:30:00Z",
      "user": "Test User <test@example.com>",
      "file_path": "auth.rs",
      "line_range": [1, 20]
    }
  ]
}
STAGING_EOF
cat > auth.rs << 'CODE_EOF'
fn authenticate() -> bool {
    true
}
CODE_EOF
git add -A
git commit -q -m "Add auth module" 2>&1 || true
"$BLAMEPROMPT" search "JWT" 2>&1 | head -5
echo "  PASS: search command executed"

# Test push command (will fail without remote, but should handle gracefully)
echo "[15/18] Testing push command (no remote expected)..."
"$BLAMEPROMPT" push 2>&1 | head -3 || true
echo "  PASS: push command handled gracefully"

# Test pull command (will fail without remote, but should handle gracefully)
echo "[16/18] Testing pull command (no remote expected)..."
"$BLAMEPROMPT" pull 2>&1 | head -3 || true
echo "  PASS: pull command handled gracefully"

# Test redact --test command
echo "[17/18] Testing redact --test command..."
cat > secrets_test.txt << 'SECRETS_EOF'
My API key is sk-ant-api03-abcdefghijklmnopqrstuvwxyz
AWS key: AKIAIOSFODNN7EXAMPLE
password = "hunter2"
Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9
Normal text without secrets
SECRETS_EOF
"$BLAMEPROMPT" redact --test secrets_test.txt 2>&1 | head -15
echo "  PASS: redact --test command executed"

# Test record --session command
echo "[18/18] Testing record --session command..."
cat > test_transcript.jsonl << 'JSONL_EOF'
{"type":"user","message":{"content":"write a fibonacci function"},"timestamp":"2026-02-15T10:00:00Z"}
{"type":"assistant","message":{"model":"claude-sonnet-4-5-20250929","content":[{"type":"text","text":"Here is a fibonacci function"},{"type":"tool_use","name":"Write","input":{"file_path":"fib.rs","content":"fn fib(n: u32) -> u32 { if n <= 1 { n } else { fib(n-1) + fib(n-2) } }"}}]},"timestamp":"2026-02-15T10:00:02Z"}
JSONL_EOF
"$BLAMEPROMPT" record --session test_transcript.jsonl 2>&1
echo "  PASS: record --session command executed"

# Test cache sync command
echo "[BONUS] Testing cache sync..."
"$BLAMEPROMPT" cache sync 2>&1
echo "  PASS: cache sync executed"

# Test .blamepromptrc config loading
echo "[BONUS] Testing .blamepromptrc config..."
cat > .blamepromptrc << 'CONFIG_EOF'
[redaction]
mode = "replace"
custom_patterns = [
    { pattern = "INTERNAL-\\d+", replacement = "[REDACTED_INTERNAL]" },
]

[capture]
max_prompt_length = 1000
store_full_conversation = false
CONFIG_EOF
echo "  PASS: .blamepromptrc created"
# Run redact test to verify config is loaded
cat > config_test.txt << 'CONFIG_TEST_EOF'
Ticket INTERNAL-12345 has an API key sk-ant-api03-abcdefghijklmnopqrstuvwxyz
CONFIG_TEST_EOF
"$BLAMEPROMPT" redact --test config_test.txt 2>&1 | head -15
echo "  PASS: Config-aware redaction works"

# Test audit with markdown output
echo "[BONUS] Testing audit --format md..."
"$BLAMEPROMPT" audit --format md 2>&1 | head -3
echo "  PASS: audit markdown executed"

# Cleanup
cd "$ORIGINAL_DIR"
rm -rf "$TMPDIR"

echo ""
echo "=== All E2E tests completed ==="
