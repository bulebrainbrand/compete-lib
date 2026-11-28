#!/usr/bin/env python3
"""List the library files a Rust source file depends on, for competitive-verifier.

Usage: python3 .verify-helper/deps.py <path>

Prints one path per line (relative to the crate root). A file depends on:
  * modules it names through `crate::`, `compete_lib::`, `self::` or `super::` paths
  * files defining `#[macro_export]` macros it calls
and, transitively, on whatever those files depend on.
`mod foo;` declarations are not followed, so lib.rs and the module files
do not make every file depend on every other file.
"""

import pathlib
import re
import sys

CRATE_NAMES = {"crate", "compete_lib"}


def find_root(path: pathlib.Path) -> pathlib.Path:
    for parent in [path.parent, *path.parents]:
        if (parent / "Cargo.toml").exists():
            return parent
    raise SystemExit(f"Cargo.toml not found for {path}")


def strip_comments(text: str) -> str:
    out = []
    i, n = 0, len(text)
    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
        elif text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append('""')
            i = j + 1
        else:
            out.append(text[i])
            i += 1
    return "".join(out)


def module_path_of(file: pathlib.Path, src: pathlib.Path) -> list[str]:
    rel = file.relative_to(src).with_suffix("")
    parts = list(rel.parts)
    if parts in (["lib"], ["main"]):
        return []
    if parts and parts[-1] == "mod":
        parts = parts[:-1]
    return parts


def file_of(segments: list[str], src: pathlib.Path) -> pathlib.Path:
    """The file of the longest module prefix of `segments`."""
    for k in range(len(segments), 0, -1):
        base = src.joinpath(*segments[:k])
        for cand in (base.with_suffix(".rs"), base / "mod.rs"):
            if cand.is_file():
                return cand
    return src / "lib.rs"


def expand_use_tree(tree: str) -> list[str]:
    """`a::{b, c::{d, e}}` -> ["a::b", "a::c::d", "a::c::e"]."""
    tree = re.sub(r"\s+", "", tree)
    m = re.match(r"^(.*?)\{(.*)\}$", tree)
    if not m:
        return [re.sub(r"\bas\w+$", "", tree)]
    prefix, inner = m.groups()
    items, depth, cur = [], 0, ""
    for ch in inner:
        if ch == "," and depth == 0:
            items.append(cur)
            cur = ""
            continue
        depth += ch == "{"
        depth -= ch == "}"
        cur += ch
    items.append(cur)
    return [p for it in items if it for p in expand_use_tree(prefix + it)]


def referenced_paths(code: str) -> list[str]:
    paths = []
    for m in re.finditer(r"\buse\s+([^;]+);", code):
        paths.extend(expand_use_tree(m.group(1)))
    paths.extend(m.group(0) for m in re.finditer(r"\b\w+(?:\s*::\s*\w+)+", code))
    return [re.sub(r"\s+", "", p).lstrip(":") for p in paths]


def resolve(path: str, here: list[str], src: pathlib.Path) -> pathlib.Path | None:
    segs = [s for s in path.split("::") if s and s != "*"]
    if not segs:
        return None
    if segs[0] in CRATE_NAMES:
        base, rest = [], segs[1:]
    elif segs[0] in ("self", "super"):
        base = list(here)
        while segs and segs[0] in ("self", "super"):
            if segs.pop(0) == "super" and base:
                base.pop()
        rest = segs
    else:
        return None
    return file_of(base + rest, src) if rest else None


def main() -> None:
    target = pathlib.Path(sys.argv[1]).resolve()
    root = find_root(target)
    src = root / "src"

    macros: dict[str, pathlib.Path] = {}
    for f in src.rglob("*.rs"):
        code = strip_comments(f.read_text(encoding="utf-8"))
        for m in re.finditer(r"#\[macro_export\]\s*macro_rules!\s*(\w+)", code):
            macros[m.group(1)] = f

    seen: set[pathlib.Path] = set()
    stack = [target]
    while stack:
        f = stack.pop()
        if f in seen:
            continue
        seen.add(f)
        code = strip_comments(f.read_text(encoding="utf-8"))
        here = module_path_of(f, src) if src in f.parents else []
        found = set()
        for p in referenced_paths(code):
            dep = resolve(p, here, src)
            if dep:
                found.add(dep)
        for name, dep in macros.items():
            if re.search(rf"(?<![\w!]){re.escape(name)}\s*!", code) and dep != f:
                if not re.search(rf"macro_rules!\s*{re.escape(name)}\b", code):
                    found.add(dep)
        stack.extend(found - seen)

    seen.discard(target)
    for f in sorted(seen):
        print(f.relative_to(root).as_posix())


if __name__ == "__main__":
    main()
