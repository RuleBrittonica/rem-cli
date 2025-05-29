"""
fetch_rs_files.py
Read extracts.csv  clone the needed repos (bare+shallow)
                   git-show each detected .rs file at the given commit
                   save under scraped_files/<repo>/<sha>__<path>.rs
"""

import csv, os, re, shutil, tempfile
from pathlib import Path
from subprocess import check_output, CalledProcessError, DEVNULL
from tqdm import tqdm

from git import Repo

#  configuration
CSV_PATH        = "scraping/extracts.csv"
REPOS_TXT       = "scraping/repos.txt"
OUT_DIR         = Path("scraping/scraped_files")
DEPTH_PER_REPO  = 600

#  map short name -> full slug
def build_slug_map():
    m = {}
    with open(REPOS_TXT, encoding="utf-8") as f:
        for line in f:
            slug = line.strip()
            if not slug or slug.startswith("#"):
                continue
            name = slug.split("/")[-1]
            m.setdefault(name, slug)   # keep 1st occurrence if duplicates
    return m

SLUG_OF = build_slug_map()

#  helper to fetch a single file version
def fetch_file(slug, sha, file_path, out_file):
    url = f"https://github.com/{slug}.git"
    tmp = tempfile.mkdtemp(prefix="rs-bare-")

    try:
        repo = Repo.clone_from(
            url,
            tmp,
            bare=True,          # bare repo: only .git directory
            depth=DEPTH_PER_REPO,
            single_branch=True,
        )
        # Ensure the exact (maybe older) commit exists
        repo.git.fetch("origin", sha, depth=1)

        blob_data = repo.git.show(f"{sha}:{file_path}")
        out_file.parent.mkdir(parents=True, exist_ok=True)
        out_file.write_text(blob_data, encoding="utf-8")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


# main
def main():
    OUT_DIR.mkdir(exist_ok=True)

    with open(CSV_PATH, newline="", encoding="utf-8") as f:
        rows = list(csv.DictReader(f))

    # Build a todo list of (slug, sha_full, file_path)
    tasks = []
    for r in rows:
        name       = r["repo"]
        slug       = SLUG_OF.get(name)
        if not slug:
            print(f"[warn] no slug for repo {name!r} – skipping")
            continue

        sha        = r["sha"]
        file_path  = r["file"]
        tasks.append((slug, sha, file_path))

    print(f"Fetching {len(tasks)} Rust files into {OUT_DIR}/ …")

    for slug, sha, file_path in tqdm(tasks, unit="file"):
        repo_name   = slug.split("/")[-1]
        safe_path   = file_path.replace("/", "_")
        out_file    = OUT_DIR / repo_name / f"{sha}__{safe_path}"
        out_file    = out_file if out_file.suffix == ".rs" else out_file.with_suffix(".rs")

        if out_file.exists():
            continue  # already fetched in a previous run

        try:
            fetch_file(slug, sha, file_path, out_file)
        except CalledProcessError as e:
            tqdm.write(f"[error] {slug} {sha} {file_path}: {e}")

    print("✓ Done – see", OUT_DIR)


if __name__ == "__main__":
    main()