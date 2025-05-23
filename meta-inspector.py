import sys
import metadata_extractor
import os

if len(sys.argv) != 2:
    print("Usage: python3 meta-inspector.py <file/source>")
    sys.exit(1)

file_path = sys.argv[1]

print("[Metadata]")
meta = metadata_extractor.extract_metadata(file_path)
print(meta)

md5, sha256 = metadata_extractor.hash_file(file_path)
print(f"\n[Hashes]\nMD5: {md5}\nSHA256: {sha256}")

created, modified, accessed = metadata_extractor.file_timestamps(file_path)
print(f"\n[Timestamps]\nCreated: {created}\nModified: {modified}\nAccessed: {accessed}")

# Export to Desktop as meta_<file/source name>.txt
file_stem = os.path.splitext(os.path.basename(file_path))[0]
export_path = os.path.expanduser(f"~/Desktop/meta_{file_stem}.txt")
with open(export_path, "w") as f:
    f.write("[Metadata]\n")
    f.write(meta + "\n\n")
    f.write(f"[Hashes]\nMD5: {md5}\nSHA256: {sha256}\n\n")
    f.write(f"[Timestamps]\nCreated: {created}\nModified: {modified}\nAccessed: {accessed}\n")
print(f"Exported metadata to {export_path}")
