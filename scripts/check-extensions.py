#!/usr/bin/env python3
"""
Compares the extension directories in an OZ stellar-contracts source tree against
known-extensions.toml. Outputs a GitHub Actions step output variable
`new_extensions` containing any unrecognised extensions (empty string if none).

Usage: python3 scripts/check-extensions.py <oz-source-dir> <known-extensions.toml>
"""
import os
import sys
import re

def list_extensions(oz_root, token_type):
    """Return the set of extension subdirectory names for fungible or non_fungible."""
    path = os.path.join(
        oz_root, "packages", "tokens", "src", token_type, "extensions"
    )
    if not os.path.isdir(path):
        return set()
    return {
        entry for entry in os.listdir(path)
        if os.path.isdir(os.path.join(path, entry))
    }

def parse_known(toml_path, section):
    """Parse the handled list for a section from a simple TOML file."""
    with open(toml_path) as f:
        content = f.read()
    # Find the section and extract the array value
    pattern = rf'\[{section}\][^\[]*handled\s*=\s*\[([^\]]*)\]'
    m = re.search(pattern, content, re.DOTALL)
    if not m:
        return set()
    items = re.findall(r'"([^"]+)"', m.group(1))
    return set(items)

def main():
    if len(sys.argv) != 3:
        print("Usage: check-extensions.py <oz-source-dir> <known-extensions.toml>", file=sys.stderr)
        sys.exit(1)

    oz_root = sys.argv[1]
    toml_path = sys.argv[2]

    new = []

    for token_type, section in [("fungible", "fungible"), ("non_fungible", "non_fungible")]:
        actual = list_extensions(oz_root, token_type)
        known = parse_known(toml_path, section)
        unknown = sorted(actual - known)
        for ext in unknown:
            new.append(f"{token_type}/{ext}")

    if new:
        value = "\n".join(new)
        print(f"new_extensions={value}")
    else:
        print("new_extensions=")

if __name__ == "__main__":
    main()
