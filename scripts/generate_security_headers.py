#!/usr/bin/env python3
"""Generate Cloudflare Pages headers that authorize Trunk's inline loader."""

import base64
import hashlib
import os
import re
from pathlib import Path

PLACEHOLDER = "{{TRUNK_INLINE_SCRIPT_HASHES}}"
SCRIPT_PATTERN = re.compile(
    r"<script\b(?P<attributes>[^>]*)>(?P<body>.*?)</script>",
    re.IGNORECASE | re.DOTALL,
)


def inline_script_hashes(html: str) -> str:
    script_bodies = [
        match.group("body")
        for match in SCRIPT_PATTERN.finditer(html)
        if not re.search(r"(?:^|\s)src\s*=", match.group("attributes"), re.IGNORECASE)
    ]
    if not script_bodies:
        raise RuntimeError("Trunk output contains no inline scripts to authorize")

    hashes = []
    for body in script_bodies:
        digest = hashlib.sha256(body.encode("utf-8")).digest()
        source = f"sha256-{base64.b64encode(digest).decode('ascii')}"
        if source not in hashes:
            hashes.append(source)
    return " ".join(f"'{source}'" for source in hashes)


def main() -> None:
    staging_dir_value = os.environ.get("TRUNK_STAGING_DIR")
    if not staging_dir_value:
        raise RuntimeError("TRUNK_STAGING_DIR is not set")

    staging_dir = Path(staging_dir_value)
    generated_html = (staging_dir / "index.html").read_text(encoding="utf-8")
    template_path = Path(__file__).resolve().parents[1] / "assets" / "_headers.template"
    template = template_path.read_text(encoding="utf-8")
    if template.count(PLACEHOLDER) != 1:
        raise RuntimeError(f"Expected exactly one {PLACEHOLDER} placeholder")

    headers = template.replace(PLACEHOLDER, inline_script_hashes(generated_html))
    if any(len(line) > 2_000 for line in headers.splitlines()):
        raise RuntimeError("Generated headers exceed Cloudflare's 2,000-character line limit")

    output_path = staging_dir / "_headers"
    output_path.write_text(headers, encoding="utf-8")
    print(f"Generated {output_path} with CSP hashes for Trunk inline scripts")


if __name__ == "__main__":
    main()
