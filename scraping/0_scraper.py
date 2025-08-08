# Rust-repo scraper
# Reads scraping/repos.txt
# Clones each repo (depth DEPTH, primary branch)
# Detects extract-method refactorings
# Stores *both* sides:
#   - pre_sha  (parent commit)
#   - post_sha (child commit)
#   - file path
#   - lifted line range in pre-extract file
#   - new fn name (+ async / generic flags)
# Appends ≤ MAX_CASES rows per repo → scraping/extracts.csv

import csv, re, shutil, tempfile, textwrap, time
from datetime import datetime
from pathlib import Path
from git import Repo, exc as gitexc
from tree_sitter import Parser
from tree_sitter_language_pack import get_language

# ───────── configuration ─────────
DEPTH        = 500
MAX_CASES    = 20
CSV_PATH     = "scraping/extracts.csv"
REPOS_TXT    = "scraping/repos.txt"

FIELDS = [
    "repo", "pre_sha", "post_sha",
    "pre_file", "post_file",
    "fn", "start_line", "end_line", "match_pct",
    "generic", "async", "author", "date"
]

# ───────── helpers & regexes ─────────
parser = Parser(get_language("rust"))

NEW_FN_RE = re.compile(r"^\+\s*(?:pub\s+)?(async\s+)?fn\s+(\w+).*?\{", re.M)
HUNK_RE   = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", re.M)

def removed_lines(patch: str):
    return [l[1:].rstrip() for l in patch.splitlines()
            if l.startswith("-") and l.strip(" -")]

def new_fn_body(patch: str, m: re.Match):
    """Return the text of the newly-added fn (diff syntax, no +/-)."""
    start = m.end()
    brace = 0; seen = False
    body = []
    for line in patch[start:].splitlines():
        if line.startswith("-"):          # removed line – ignore
            continue
        clean = line[1:] if line.startswith("+") else line
        body.append(clean.rstrip())
        brace += clean.count("{") - clean.count("}")
        if "{" in clean: seen = True
        if seen and brace == 0:
            break
    return body

def body_contains(body, removed):
    body_set = set(textwrap.dedent("\n".join(body)).splitlines())
    hits = sum(1 for l in removed if l.strip() in body_set)
    return 0 if not removed else hits * 100 // len(removed)

def removed_range(patch: str):
    spans = []
    for h in HUNK_RE.finditer(patch):
        start = int(h.group(1))
        length = int(h.group(2) or 1)
        hunkslice = patch[h.end():].split("\n", 1)[0]
        if "-" in hunkslice:
            spans.append((start, start + length - 1))
    if not spans:
        return None
    return min(s[0] for s in spans), max(s[1] for s in spans)

def write_rows(rows):
    new = not Path(CSV_PATH).exists()
    mode = "a" if not new else "w"
    with open(CSV_PATH, mode, newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        if new:
            w.writeheader()
        w.writerows(rows)

# ───────── core scraper ─────────
def scrape_repo(slug: str):
    repo_name = slug.split("/")[-1]
    tmp = tempfile.mkdtemp(prefix="scrape-")
    rows = []
    print(f"[{datetime.now():%H:%M:%S}] cloning {repo_name} …")

    try:
        repo = Repo.clone_from(f"https://github.com/{slug}.git", tmp,
                               depth=DEPTH, single_branch=True, no_checkout=False)

        for commit in repo.iter_commits():
            if len(rows) >= MAX_CASES:
                break
            if not commit.parents:
                continue
            parent = commit.parents[0]
            try:
                diffs = parent.diff(commit, create_patch=True)
            except gitexc.GitCommandError:
                break  # shallow boundary

            for d in diffs:
                if len(rows) >= MAX_CASES:
                    break
                if not (d.b_path and d.b_path.endswith(".rs")):
                    continue

                patch = d.diff.decode(errors="ignore")
                m = NEW_FN_RE.search(patch)
                if not m:
                    continue

                fn = m.group(2)
                body = new_fn_body(patch, m)
                removed = removed_lines(patch)
                match_pct = body_contains(body, removed)
                if match_pct < 60:        # require ≥60 % literal overlap
                    continue

                rng = removed_range(patch)
                if not rng:
                    continue

                sig_line = patch[m.start():patch.find("\n", m.start())]
                rows.append({
                    "repo":        repo_name,
                    "pre_sha":     parent.hexsha,
                    "post_sha":    commit.hexsha,
                    "pre_file":    d.a_path,
                    "post_file":   d.b_path,
                    "fn":          fn,
                    "start_line":  rng[0],
                    "end_line":    rng[1],
                    "match_pct":   match_pct,
                    "generic":     "<" in sig_line and ">" in sig_line,
                    "async":       bool(m.group(1)),
                    "author":      commit.author.email,
                    "date":        commit.committed_datetime.isoformat(),
                })
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    return rows

# ───────── orchestration ─────────
def main():
    slugs = [l.strip() for l in Path(REPOS_TXT).read_text().splitlines()
             if l.strip() and not l.startswith("#")]
    total = 0; start = time.time()
    for i, slug in enumerate(slugs, 1):
        rows = scrape_repo(slug)
        total += len(rows)
        if rows:
            write_rows(rows)
        print(f"[{i:02}/{len(slugs)}] {slug.split('/')[-1]:<20} → "
              f"{len(rows)} rows (cum {total})")
    print(f"✓ Done – {total} rows → {CSV_PATH} ({time.time()-start:.1f}s)")

if __name__ == "__main__":
    main()
