# For each row:
# - clone scraping/cloned_repos/<repo>/<sha>/
# - insert the two marker comments around the fn body
# - append path to EXTRACTION_PATHS.txt (repo root)

import csv, re, shutil
from pathlib import Path
from git import Repo, exc as gitexc
from tqdm import tqdm

SELECT_CSV  = "selected_cases.csv" # shortlisted repos
REPOS_TXT   = "scraping/repos.txt"
OUT_ROOT    = Path("scraping/cloned_repos")
DEPTH       = 700
MAX_PER_REPO = 20

# -------- slug map --------
slug_of = {l.strip().split("/")[-1]: l.strip()
           for l in Path(REPOS_TXT).read_text().splitlines()
           if l.strip() and not l.startswith("#")}

# -------- load shortlist & cap --------
rows = list(csv.DictReader(open(SELECT_CSV, encoding="utf-8")))
bucket = {}
for r in rows:
    bucket.setdefault(r["repo"], []).append(r)
for repo in bucket:
    bucket[repo] = bucket[repo][:MAX_PER_REPO]

total = sum(len(v) for v in bucket.values())
print(f"Cloning/annotating {total} cases from {len(bucket)} repos")

# -------- annotator --------
def annotate(path: Path, fn: str):
    txt = path.read_text(encoding="utf-8", errors="ignore")
    m = re.search(rf"^(?P<i>\s*).*?\bfn\s+{re.escape(fn)}\b.*\{{", txt, re.M)
    if not m: return None
    start = txt[:m.end()].count("\n")
    lines = txt.splitlines(keepends=True)
    depth=0; body=False; end=start
    for idx in range(start, len(lines)):
        for ch in lines[idx]:
            if ch=="{": depth+=1; body=True
            elif ch=="}": depth-=1
        if body and depth==0:
            end=idx
            break
    lines.insert(start, "// Extraction Starts Here\n")
    lines.insert(end+2, "// Extraction Ends Here\n")
    path.write_text("".join(lines), encoding="utf-8")
    return start+1, end+2   # 1-based line numbers

# -------- main loop --------
for repo, cases in tqdm(bucket.items(), unit="repo"):
    slug = slug_of.get(repo)
    if not slug:
        print(f"[warn] slug missing for {repo}")
        continue
    for r in cases:
        sha, path, fn = r["sha"], r["file"], r["fn"]
        repo_dir = OUT_ROOT / repo / sha[:10]
        if not repo_dir.exists():
            Repo.clone_from(f"https://github.com/{slug}.git", repo_dir,
                            depth=DEPTH, single_branch=True)
        git = Repo(repo_dir).git
        try: git.fetch("origin", sha, depth=1)
        except gitexc.GitCommandError: pass
        git.checkout(sha)

        file_path = repo_dir / path
        line_span = annotate(file_path, fn)
        if not line_span: continue

        idx = repo_dir / "SELECTED_EXTRACTION.txt"
        with idx.open("a", encoding="utf-8") as f:
            f.write(f"file: {file_path.relative_to(repo_dir)}\n")
            f.write(f"fn:   {fn}\n")
            f.write(f"lines:{line_span[0]}-{line_span[1]}\n\n")

print("Repos cloned & labelled - see", OUT_ROOT)