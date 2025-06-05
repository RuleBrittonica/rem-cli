"""
Rust-repo scraper

• Reads scraping/repos.txt
• Clones each repo (depth DEPTH, primary branch)
• Detects extract-method refactorings
• Flags async / generic new functions
• Appends to scraping/extracts.csv after every repo
• Prints one-line progress updates instead of a progress bar
"""
import csv
import re
import shutil
import tempfile
import time
from datetime import datetime
from pathlib import Path

from git import Repo
from git.exc import GitCommandError
from tree_sitter import Parser
from tree_sitter_language_pack import get_language
#  configuration
DEPTH       = 500            # commits per repo
CSV_PATH    = "scraping/extracts.csv"
REPOS_TXT   = "scraping/repos.txt"

FIELDNAMES  = [
    "repo", "sha", "author", "date",
    "file", "fn", "generic", "async"
]

#  tree-sitter setup
RUST   = get_language("rust")
parser = Parser(RUST)

NEW_FN_RE = re.compile(r"^\+\s*(?:pub\s+)?(async\s+)?fn\s+(\w+)", re.M)

def is_extract_method(patch: str, fn_name: str) -> bool:
    """Heuristic:   new fn  +  ≥3 removed lines  +  a new call to that fn"""
    removed     = [l for l in patch.splitlines()
                   if l.startswith("-") and l.strip(" -")]
    added_call  = re.search(rf"^\+\s*{re.escape(fn_name)}\s*[(]", patch, re.M)
    return len(removed) >= 3 and added_call is not None


def append_rows(rows):
    new_file = not Path(CSV_PATH).exists()
    mode     = "a" if not new_file else "w"
    with open(CSV_PATH, mode, newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDNAMES)
        if new_file:
            writer.writeheader()
        writer.writerows(rows)


def scrape_repo(full_name: str):
    """Clone one repo (shallow) and return (name, n_matches, rows)."""
    url   = f"https://github.com/{full_name}.git"
    name  = full_name.split("/")[-1]
    tmp   = tempfile.mkdtemp(prefix="scrape-")
    rows  = []

    print(f"[{datetime.now().isoformat(timespec='seconds')}] ")
    print(f"Cloning {name} from {url} (depth {DEPTH})")

    try:
        repo = Repo.clone_from(
            url, tmp, depth=DEPTH, single_branch=True, no_checkout=False
        )

        for commit in repo.iter_commits():
            if not commit.parents:
                continue
            parent = commit.parents[0]

            # Shallow edge?  Skip & stop if diff fails.
            try:
                diffs = parent.diff(commit, create_patch=True)
            except GitCommandError:
                break

            for d in diffs:
                if not (d.b_path and d.b_path.endswith(".rs")):
                    continue

                patch = d.diff.decode(errors="ignore")
                m     = NEW_FN_RE.search(patch)
                if not m:
                    continue

                fn_name  = m.group(2)
                if not is_extract_method(patch, fn_name):
                    continue

                sig_after_fn = patch[m.end(): patch.find("\n", m.end())]
                rows.append(
                    {
                        "repo":    name,
                        "sha":     commit.hexsha,
                        "author":  commit.author.email,

                          
                        "date":    commit.committed_datetime.isoformat(),
                        "file":    d.b_path,
                        "fn":      fn_name,
                        "generic": "<" in sig_after_fn and ">" in sig_after_fn,
                        "async":   bool(m.group(1)),
                    }
                )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    return name, len(rows), rows


def main():
    with open(REPOS_TXT, encoding="utf-8") as f:
        repos = [r.strip() for r in f
                 if r.strip() and not r.startswith("#")]

    print(f"[{datetime.now().isoformat(timespec='seconds')}] "
          f"Scanning {len(repos)} repos (depth {DEPTH})")

    start_all = time.time()
    total_matches = 0

    for idx, full_name in enumerate(repos, 1):
        start_repo = time.time()
        repo_name, n_matches, rows = scrape_repo(full_name)
        if rows:
            append_rows(rows)
            total_matches += n_matches

        elapsed_repo = time.time() - start_repo
        print(f"[{idx:02}/{len(repos)}] {repo_name:<25} "
              f"→ {n_matches:3d} matches  "
              f"({elapsed_repo:5.1f}s)")

    print(f"\n !!! Finished.  {total_matches} matches "
          f"written to {CSV_PATH}  "
          f"in {time.time() - start_all:0.1f}s")


if __name__ == "__main__":
    main()