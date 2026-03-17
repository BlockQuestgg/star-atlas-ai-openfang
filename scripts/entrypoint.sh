#!/bin/bash
# Container entrypoint — sets up config, installs agents, starts daemon,
# installs Skills and Hands.
#
# Sequence:
#   1. Copy config template
#   2. Install custom agent templates
#   3. Start openfang in the background
#   4. Wait for daemon health
#   5. Install custom Skills
#   6. Install custom Hands
#   7. List installed Hands (manual activation)
#   8. Wait on daemon PID (keeps container alive)

set -euo pipefail

OPENFANG_HOME="/home/openfang/.openfang"
CONFIG_TEMPLATE="/home/openfang/project/.config-template/config.toml"
HANDS_DIR="/home/openfang/project/hands"
SKILLS_DIR="/home/openfang/project/skills"
AGENTS_DIR="/home/openfang/project/agents/custom"

# --- Step 1: Copy config ---
mkdir -p "$OPENFANG_HOME"

if [ -f "$CONFIG_TEMPLATE" ]; then
  cp "$CONFIG_TEMPLATE" "$OPENFANG_HOME/config.toml"
  chmod 600 "$OPENFANG_HOME/config.toml"
  echo "Config copied from template."
else
  echo "ERROR: Config template not found at $CONFIG_TEMPLATE"
  exit 1
fi

# --- Step 2: Install custom agent templates ---
# OpenFang looks for agents at ~/.openfang/agents/{name}/agent.toml
if [ -d "$AGENTS_DIR" ]; then
  for agent_file in "$AGENTS_DIR"/*.toml; do
    [ -f "$agent_file" ] || continue
    agent_name=$(basename "$agent_file" .toml)
    agent_dest="$OPENFANG_HOME/agents/$agent_name"
    mkdir -p "$agent_dest"
    cp "$agent_file" "$agent_dest/agent.toml"
    echo "Installed agent template: $agent_name"
  done
fi

# --- Step 3: Start daemon in background ---
openfang start &
DAEMON_PID=$!

# --- Step 4: Wait for daemon to be ready ---
echo "Waiting for OpenFang daemon..."
HEALTHY=false
for i in $(seq 1 30); do
  if openfang health 2>/dev/null; then
    echo "Daemon ready."
    HEALTHY=true
    break
  fi
  if ! kill -0 "$DAEMON_PID" 2>/dev/null; then
    echo "ERROR: Daemon exited before becoming healthy."
    exit 1
  fi
  sleep 1
done

if [ "$HEALTHY" = false ]; then
  echo "ERROR: Daemon did not become healthy within 30 seconds."
  exit 1
fi

# --- Step 5: Install custom Skills ---
if [ -d "$SKILLS_DIR" ]; then
  for skill_dir in "$SKILLS_DIR"/*/; do
    [ -f "$skill_dir/skill.toml" ] || continue
    skill_name=$(basename "$skill_dir")
    echo "Installing skill: $skill_name"
    openfang skill install "$skill_dir" 2>&1 || echo "WARNING: skill install failed for $skill_name"
  done
fi

# --- Step 6: Install custom Hands ---
INSTALL_FAILURES=0
if [ -d "$HANDS_DIR" ]; then
  for hand_dir in "$HANDS_DIR"/*/; do
    [ -f "$hand_dir/HAND.toml" ] || continue
    hand_name=$(basename "$hand_dir")
    echo "Installing hand: $hand_name"
    if ! openfang hand install "$hand_dir" 2>&1; then
      echo "ERROR: hand install failed for $hand_name"
      INSTALL_FAILURES=$((INSTALL_FAILURES + 1))
    fi
  done
  echo "Hand installation complete. Failures: $INSTALL_FAILURES"
else
  echo "No hands directory found at $HANDS_DIR"
fi

# --- Step 7: List installed hands (activate manually via `just hand-activate-*`) ---
# Hands are installed but NOT auto-activated to avoid burning LLM tokens
# on autonomous ticks. Activate on demand:
#   just hand-activate-researcher
#   just hand-activate-brainstorm
#   just hand-activate-pip-advisor
#   just hand-activate-knowledge-keeper
echo "Installed hands (activate manually):"
openfang hand list 2>&1 || true

# --- Step 8: Keep container alive on daemon ---
wait $DAEMON_PID
