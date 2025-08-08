# Read scraping/extracts.csv, git-show each detected .rs file,
# label it with two marker comments, and save under
# scraping/scraped_files/<repo>/<sha>__<path>.rs

import csv, re, shutil, tempfile
from pathlib import Path
from subprocess import CalledProcessError
from tqdm import tqdm
from git import Repo

CSV_PATH   = "scraping/extracts.csv"
REPOS_TXT  = "scraping/repos.txt"
OUT_DIR    = Path("scraping/scraped_files")
DEPTH      = 600 # go back further than original scraper incase of new commits

# slug map
slug_of = {l.strip().split("/")[-1]: l.strip()
           for l in Path(REPOS_TXT).read_text().splitlines()
           if l.strip() and not l.startswith("#")}

# marker helpers
def add_markers_post(code: str, fn: str) -> str:
    """Mark the new function in POST-extract file."""
    m = re.search(rf"^(?P<i>\s*).*?\bfn\s+{re.escape(fn)}\b.*\{{", code, re.M)
    if not m:
        return code
    start_ln = code[:m.end()].count("\n")
    lines = code.splitlines(keepends=True)
    depth = 0; seen_body = False; end_ln = start_ln
    for idx in range(start_ln, len(lines)):
        for ch in lines[idx]:
            if ch == "{": depth += 1; seen_body = True
            elif ch == "}": depth -= 1
        if seen_body and depth == 0:
            end_ln = idx
            break
    lines.insert(start_ln, "// Extraction Starts Here\n")
    lines.insert(end_ln + 2, "// Extraction Ends Here\n")
    return "".join(lines)

def add_markers_pre(code: str, start: int, end: int) -> str:
    """
    Mark the removed block in PRE-extract file.
    start / end are 1-based inclusive line numbers from CSV.
    """
    lines = code.splitlines(keepends=True)
    if start > len(lines):               # out of range → leave untouched
        return code
    end = min(end, len(lines))
    # insert END marker first (index unaffected by earlier insert)
    lines.insert(end, "// Extraction Ends Here\n")
    lines.insert(start - 1, "// Extraction Starts Here\n")
    return "".join(lines)

# fetch helper
def git_show(slug: str, sha: str, file_path: str) -> str:
    tmp = tempfile.mkdtemp(prefix="bare-")
    try:
        repo = Repo.clone_from(f"https://github.com/{slug}.git", tmp,
                               bare=True, depth=DEPTH, single_branch=True)
        try: repo.git.fetch("origin", sha, depth=1)
        except: pass
        return repo.git.show(f"{sha}:{file_path}")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

# main
def main():
    OUT_DIR.mkdir(exist_ok=True)
    rows = list(csv.DictReader(open(CSV_PATH, encoding="utf-8")))

    print(f"Fetching {len(rows)} pre/post file pairs …")
    for r in tqdm(rows, unit="case"):
        repo_short = r["repo"]
        slug = slug_of.get(repo_short)
        if not slug:
            tqdm.write(f"[warn] slug missing for {repo_short}")
            continue

        pre_sha  = r["pre_sha"]
        post_sha = r["post_sha"]
        path     = r["file"]
        fn       = r["fn"]
        start_ln = int(r["start_line"])
        end_ln   = int(r["end_line"])

        safe     = path.replace("/", "_")
        repo_dir = OUT_DIR / repo_short
        repo_dir.mkdir(parents=True, exist_ok=True)

        # PRE version
        pre_dst = repo_dir / f"pre_{pre_sha[:10]}__{safe}"
        if not pre_dst.exists():
            try:
                code_pre = git_show(slug, pre_sha, path)
                pre_dst.write_text(add_markers_pre(code_pre, start_ln, end_ln),
                                   encoding="utf-8")
            except CalledProcessError as e:
                tqdm.write(f"[err] PRE {slug} {pre_sha[:7]} {path}: {e}")

        # POST version
        post_dst = repo_dir / f"post_{post_sha[:10]}__{safe}"
        if not post_dst.exists():
            try:
                code_post = git_show(slug, post_sha, path)
                post_dst.write_text(add_markers_post(code_post, fn),
                                    encoding="utf-8")
            except CalledProcessError as e:
                tqdm.write(f"[err] POST {slug} {post_sha[:7]} {path}: {e}")

    print("✓ All files written to", OUT_DIR)

if __name__ == "__main__":
    main()