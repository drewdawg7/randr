#!/usr/bin/env python3
"""Analyze Claude session data for cost and usage insights."""

import json
from pathlib import Path
from datetime import datetime
from typing import Optional

# Pricing per million tokens (Opus 4.5)
PRICING = {
    'claude-opus-4-5-20251101': {
        'input': 5.0,
        'output': 25.0,
        'cache_read': 0.5,
        'cache_write': 6.25
    },
    'claude-sonnet-4-5-20250929': {
        'input': 3.0,
        'output': 15.0,
        'cache_read': 0.3,
        'cache_write': 3.75
    },
    'default': {
        'input': 5.0,
        'output': 25.0,
        'cache_read': 0.5,
        'cache_write': 6.25
    }
}


def get_pricing(model: str) -> dict:
    """Get pricing for a model."""
    for key in PRICING:
        if key in model:
            return PRICING[key]
    return PRICING['default']


def get_current_session_id() -> Optional[str]:
    """Get the current session ID from history."""
    history_file = Path.home() / ".claude" / "history.jsonl"
    if not history_file.exists():
        return None
    with open(history_file, 'r') as f:
        lines = f.readlines()
        if lines:
            last_entry = json.loads(lines[-1])
            return last_entry.get('sessionId')
    return None


def get_project_path() -> Path:
    """Get the Claude project path for current directory."""
    import os
    cwd = os.getcwd()
    project_name = cwd.replace('/', '-')
    return Path.home() / ".claude" / "projects" / project_name


def analyze_session(session_id: Optional[str] = None, project_path: Optional[Path] = None) -> dict:
    """Analyze a session for cost and usage insights."""
    if session_id is None:
        session_id = get_current_session_id()
    if project_path is None:
        project_path = get_project_path()

    session_file = project_path / f"{session_id}.jsonl"

    if not session_file.exists():
        return {"error": f"Session file not found: {session_file}"}

    # Data structures
    tool_stats = {}
    model_stats = {}
    timestamps = []
    api_calls = []

    with open(session_file, 'r') as f:
        for line in f:
            try:
                entry = json.loads(line)

                if 'timestamp' in entry:
                    timestamps.append(entry['timestamp'])

                if 'message' in entry and entry.get('type') == 'assistant':
                    msg = entry['message']
                    usage = msg.get('usage', {})
                    content = msg.get('content', [])
                    model = msg.get('model', 'unknown')

                    if not usage:
                        continue

                    # Extract metrics
                    input_t = usage.get('input_tokens', 0)
                    output_t = usage.get('output_tokens', 0)
                    cache_read = usage.get('cache_read_input_tokens', 0)
                    cache_write = usage.get('cache_creation_input_tokens', 0)

                    # Calculate cost
                    pricing = get_pricing(model)
                    cost = (
                        input_t * pricing['input'] +
                        output_t * pricing['output'] +
                        cache_read * pricing['cache_read'] +
                        cache_write * pricing['cache_write']
                    ) / 1_000_000

                    # Find tool name if any
                    tool_name = None
                    if isinstance(content, list):
                        for item in content:
                            if isinstance(item, dict) and item.get('type') == 'tool_use':
                                tool_name = item.get('name')
                                break

                    tool_name = tool_name or 'text_response'

                    # Update tool stats
                    if tool_name not in tool_stats:
                        tool_stats[tool_name] = {
                            'calls': 0,
                            'input_tokens': 0,
                            'output_tokens': 0,
                            'cache_read': 0,
                            'cache_write': 0,
                            'cost': 0.0
                        }

                    tool_stats[tool_name]['calls'] += 1
                    tool_stats[tool_name]['input_tokens'] += input_t
                    tool_stats[tool_name]['output_tokens'] += output_t
                    tool_stats[tool_name]['cache_read'] += cache_read
                    tool_stats[tool_name]['cache_write'] += cache_write
                    tool_stats[tool_name]['cost'] += cost

                    # Update model stats
                    if model not in model_stats:
                        model_stats[model] = {
                            'calls': 0,
                            'input_tokens': 0,
                            'output_tokens': 0,
                            'cache_read': 0,
                            'cache_write': 0,
                            'cost': 0.0
                        }

                    model_stats[model]['calls'] += 1
                    model_stats[model]['input_tokens'] += input_t
                    model_stats[model]['output_tokens'] += output_t
                    model_stats[model]['cache_read'] += cache_read
                    model_stats[model]['cache_write'] += cache_write
                    model_stats[model]['cost'] += cost

                    api_calls.append({
                        'tool': tool_name,
                        'cost': cost,
                        'cache_write': cache_write
                    })

            except json.JSONDecodeError:
                continue

    # Calculate duration
    duration_seconds = 0
    if len(timestamps) >= 2:
        try:
            start = datetime.fromisoformat(timestamps[0].replace('Z', '+00:00'))
            end = datetime.fromisoformat(timestamps[-1].replace('Z', '+00:00'))
            duration_seconds = (end - start).total_seconds()
        except:
            pass

    # Calculate totals
    total_cost = sum(t['cost'] for t in tool_stats.values())
    total_calls = sum(t['calls'] for t in tool_stats.values())
    total_input = sum(t['input_tokens'] for t in tool_stats.values())
    total_output = sum(t['output_tokens'] for t in tool_stats.values())
    total_cache_read = sum(t['cache_read'] for t in tool_stats.values())
    total_cache_write = sum(t['cache_write'] for t in tool_stats.values())

    # Sort tools by cost
    sorted_tools = sorted(tool_stats.items(), key=lambda x: -x[1]['cost'])

    # Top 5 most expensive tools
    top_tools = [
        {
            'name': name,
            'calls': data['calls'],
            'cost': round(data['cost'], 4),
            'avg_cost': round(data['cost'] / data['calls'], 4) if data['calls'] > 0 else 0
        }
        for name, data in sorted_tools[:5]
    ]

    # Cache efficiency ratio
    cache_efficiency = total_cache_read / total_input if total_input > 0 else 0

    return {
        "success": True,
        "session_id": session_id,
        "duration_seconds": int(duration_seconds),
        "duration_formatted": f"{int(duration_seconds // 3600)}h {int((duration_seconds % 3600) // 60)}m",
        "api_calls": total_calls,
        "total_cost": round(total_cost, 2),
        "avg_cost_per_call": round(total_cost / total_calls, 4) if total_calls > 0 else 0,
        "tokens": {
            "input": total_input,
            "output": total_output,
            "message": total_input + total_output,
            "cache_read": total_cache_read,
            "cache_write": total_cache_write
        },
        "cache_efficiency_ratio": round(cache_efficiency, 1),
        "top_tools_by_cost": top_tools,
        "model_breakdown": {
            model: {
                "calls": data['calls'],
                "cost": round(data['cost'], 2)
            }
            for model, data in model_stats.items()
            if model != '<synthetic>'
        }
    }


if __name__ == "__main__":
    import sys
    session_id = sys.argv[1] if len(sys.argv) > 1 else None
    result = analyze_session(session_id)
    print(json.dumps(result, indent=2))
