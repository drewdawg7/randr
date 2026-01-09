#!/usr/bin/env python3
"""
Generate markdown report from session stats JSON.

Renders the JSON data through the markdown template.
"""
import json
import re
import sys
from pathlib import Path
from typing import Union


SCRIPT_DIR = Path(__file__).parent
TEMPLATE_DIR = SCRIPT_DIR.parent / "templates"
DEFAULT_TEMPLATE = TEMPLATE_DIR / "session-report.md"


def format_number(value: Union[int, float]) -> str:
    """Format number with thousands separators."""
    if isinstance(value, float):
        return f"{value:,.2f}"
    return f"{value:,}"


def format_percent(value: float) -> str:
    """Format decimal as percentage."""
    return f"{value * 100:.1f}%"


def render_template(template: str, data: dict) -> str:
    """
    Render template with data.

    Supports:
    - {{key}} - Simple replacement
    - {{key.nested}} - Nested key access
    - {{key|number}} - Format as number
    - {{key|percent}} - Format as percentage
    - {{#each key}}...{{/each}} - Loop over list/dict
    - {{#if key}}...{{/if}} - Conditional
    - {{@key}} - Current key in each loop
    - {{this}} - Current value in each loop
    """
    result = template

    # Handle {{#each key}}...{{/each}} blocks
    each_pattern = re.compile(r'\{\{#each\s+(\w+(?:\.\w+)*)\}\}(.*?)\{\{/each\}\}', re.DOTALL)

    def replace_each(match):
        key_path = match.group(1)
        block_content = match.group(2)

        # Get the value
        value = data
        for key in key_path.split('.'):
            if isinstance(value, dict):
                value = value.get(key, {})
            else:
                value = {}

        if isinstance(value, dict):
            # Iterate over dict items
            lines = []
            for k, v in value.items():
                line = block_content
                line = line.replace("{{@key}}", str(k))
                line = line.replace("{{this}}", str(v))
                lines.append(line)
            return "".join(lines)
        elif isinstance(value, list):
            # Iterate over list items
            lines = []
            for item in value:
                line = block_content
                line = line.replace("{{this}}", str(item))
                # Handle nested access in list items
                if isinstance(item, dict):
                    for k, v in item.items():
                        line = line.replace(f"{{{{{k}}}}}", str(v) if v is not None else "")
                lines.append(line)
            return "".join(lines)
        return ""

    result = each_pattern.sub(replace_each, result)

    # Handle {{#if key}}...{{/if}} blocks
    if_pattern = re.compile(r'\{\{#if\s+(\w+(?:\.\w+)*)\}\}(.*?)\{\{/if\}\}', re.DOTALL)

    def replace_if(match):
        key_path = match.group(1)
        block_content = match.group(2)

        # Get the value
        value = data
        for key in key_path.split('.'):
            if isinstance(value, dict):
                value = value.get(key)
            else:
                value = None

        # Check truthiness
        if value and (not isinstance(value, (list, dict)) or len(value) > 0):
            return block_content
        return ""

    result = if_pattern.sub(replace_if, result)

    # Handle simple {{key}} and {{key.nested}} replacements with filters
    var_pattern = re.compile(r'\{\{(\w+(?:\.\w+)*)(?:\|(\w+))?\}\}')

    def replace_var(match):
        key_path = match.group(1)
        filter_name = match.group(2)

        # Get the value
        value = data
        for key in key_path.split('.'):
            if isinstance(value, dict):
                value = value.get(key)
            else:
                value = None

        if value is None:
            return ""

        # Apply filter
        if filter_name == "number":
            return format_number(value)
        elif filter_name == "percent":
            return format_percent(value)
        elif filter_name == "length":
            return str(len(value)) if hasattr(value, '__len__') else "0"

        return str(value)

    result = var_pattern.sub(replace_var, result)

    return result


def generate_report(stats: dict, output_dir: Path, template_path: Path = None) -> Path:
    """
    Generate markdown report from session stats.

    Args:
        stats: Session stats dict
        output_dir: Directory to write report
        template_path: Path to template file (default: templates/session-report.md)

    Returns:
        Path to generated report file
    """
    if template_path is None:
        template_path = DEFAULT_TEMPLATE

    # Read template
    with open(template_path, "r") as f:
        template = f.read()

    # Render template
    report = render_template(template, stats)

    # Write report
    output_dir.mkdir(parents=True, exist_ok=True)
    output_file = output_dir / f"{stats['session_id']}.md"

    with open(output_file, "w") as f:
        f.write(report)

    return output_file


def main():
    """CLI entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Generate markdown report from session stats JSON"
    )
    parser.add_argument("json_file", help="Path to session stats JSON file")
    parser.add_argument("--template", help="Path to template file")
    parser.add_argument(
        "--output-dir",
        default=".claude/session_stats",
        help="Output directory (default: .claude/session_stats)"
    )

    args = parser.parse_args()

    # Load JSON
    with open(args.json_file, "r") as f:
        stats = json.load(f)

    # Generate report
    template_path = Path(args.template) if args.template else None
    output_dir = Path(args.output_dir)

    report_file = generate_report(stats, output_dir, template_path)
    print(f"Generated: {report_file}")


if __name__ == "__main__":
    main()
