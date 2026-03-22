#!/usr/bin/env python3
"""
Example script demonstrating undocx Python bindings.
Install: pip install undocx
"""

import sys


def main():
    try:
        import undocx
    except ImportError:
        print("❌ undocx not installed. Install with: pip install undocx")
        sys.exit(1)

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input.docx> [output.md]")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else None

    try:
        # Request: "open with open and pass only data"
        print(f"Reading '{input_file}' as bytes...")
        with open(input_file, "rb") as f:
            data = f.read()

        markdown = undocx.convert_docx(data)

        if output_file:
            with open(output_file, "w", encoding="utf-8") as f:
                f.write(markdown)
            print(f"✅ Converted '{input_file}' to '{output_file}'")
        else:
            print(markdown)

    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
