"""Compile copied source files without Cargo or sibling implementation files."""
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CASES = {
    "sorted_set": "let mut s = SortedSet::new(); assert!(s.add(2)); assert!(!s.add(2)); assert_eq!(s.pop(-1), Some(2));",
    "sorted_multiset": "let mut m = SortedMultiset::new(); m.add(2); m.add(2); assert_eq!(m.count(&2), 2); assert!(m.discard(&2));",
    "bucket_list": "let mut b = BucketList::new(); b.append(2); b.insert(0, 3).unwrap(); assert_eq!(b.pop(-1), Some(2));",
}
for names in [(name,) for name in CASES] + [tuple(CASES)]:
    with tempfile.TemporaryDirectory() as directory:
        folder = Path(directory)
        source = "\n".join((ROOT / "src" / f"{name}.rs").read_text() for name in names)
        source += "\nfn main() {\n" + "\n".join(CASES[name] for name in names) + "\n}\n"
        (folder / "main.rs").write_text(source)
        subprocess.run(["rustc", "--edition=2021", "-D", "warnings", "main.rs", "-o", "check"], cwd=folder, check=True)
        subprocess.run([str(folder / "check")], check=True)
        print("Standalone copy-paste OK:", ", ".join(names))
