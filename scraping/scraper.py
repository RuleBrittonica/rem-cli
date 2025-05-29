"""
Parallel Rust-repo scraper

• Reads repos.txt (one GitHub slug per line)
• Clones each repo (last DEPTH commits, primary branch)
• Detects extract-method refactorings
• Marks if the new fn is async or generic
• Appends results to extracts.csv as soon as each repo finishes
• Shows a tqdm progress bar
"""

import csv, re, shutil, tempfile, textwrap
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime
from pathlib import Path

from git import Repo
from tree_sitter import Parser
from tree_sitter_language_pack import get_language
from tqdm import tqdm

# ───────── configuration ─────────
DEPTH        = 500
MAX_WORKERS  = 8                         # tune to your CPU / bandwidth
CSV_PATH     = "extracts.csv"
FIELDNAMES   = [
    "repo", "sha", "author", "date",
    "file", "fn", "generic", "async"
]

# ───────── tree-sitter setup ─────────
parser = Parser()
parser.set_language(get_language("rust"))

NEW_FN_RE = re.compile(r"^\+\s*(?:pub\s+)?(async\s+)?fn\s+(\w+)", re.M)

def is_extract_method(patch: str, fn_name: str) -> bool:
    """Heuristic: new fn + ≥3 removed lines + a new call to that fn"""
    removed = [l for l in patch.splitlines() if l.startswith("-") and l.strip(" -")]
    added_call = re.search(rf"^\+\s*{re.escape(fn_name)}\s*[(]", patch, re.M)
    return len(removed) >= 3 and added_call is not None


# ───────── CSV helper ─────────
def append_rows(rows):
    new_file = not Path(CSV_PATH).exists()
    mode     = "a" if not new_file else "w"
    with open(CSV_PATH, mode, newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=FIELDNAMES)
        if new_file:
            w.writeheader()
        w.writerows(rows)


# ───────── per-repo worker ─────────
def scrape_repo(full_name: str):
    url  = f"https://github.com/{full_name}.git"
    name = full_name.split("/")[-1]
    tmp  = tempfile.mkdtemp(prefix="scrape-")
    rows = []

    try:
        repo = Repo.clone_from(
            url, tmp, depth=DEPTH, single_branch=True, no_checkout=False
        )

        for commit in repo.iter_commits():
            if not commit.parents:
                continue
            parent = commit.parents[0]

            for d in parent.diff(commit, create_patch=True):
                if not (d.b_path and d.b_path.endswith(".rs")):
                    continue
                patch = d.diff.decode(errors="ignore")
                m = NEW_FN_RE.search(patch)
                if not m:
                    continue

                is_async = bool(m.group(1))
                fn_name  = m.group(2)

                if not is_extract_method(patch, fn_name):
                    continue

                sig_after_fn = patch[m.end() : patch.find("\n", m.end())]
                is_generic   = "<" in sig_after_fn and ">" in sig_after_fn

                rows.append(
                    {
                        "repo":   name,
                        "sha":    commit.hexsha[:10],
                        "author": commit.author.email,
                        "date":   commit.committed_datetime.isoformat(),
                        "file":   d.b_path,
                        "fn":     fn_name,
                        "generic": is_generic,
                        "async":   is_async,
                    }
                )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    return name, len(rows), rows


# ───────── main ─────────
def main():
    with open("repos.txt", encoding="utf-8") as f:
        repos = [line.strip() for line in f if line.strip() and not line.startswith("#")]

    print(f"Scanning {len(repos)} repos - {DEPTH} commits each …")
    with ThreadPoolExecutor(max_workers=min(MAX_WORKERS, len(repos))) as ex:
        futures = {ex.submit(scrape_repo, r): r for r in repos}
        for fut in tqdm(as_completed(futures), total=len(futures), unit="repo"):
            repo, n, rows = fut.result()
            if rows:
                append_rows(rows)
            tqdm.write(f"{repo:25s} → {n:3d} matches")

    print("\n✓ Done — results in", CSV_PATH)


if __name__ == "__main__":
    main()