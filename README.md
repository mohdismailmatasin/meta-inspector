Meta Inspector
==============

A fast, cross-platform forensic tool for extracting metadata, hashes, and timestamps from files (images, audio, video, etc.) using Rust for performance and Python for easy scripting.

Features
--------

- Extracts metadata from images, audio, and video files
- Calculates MD5 and SHA256 hashes
- Gets file creation, modification, and access timestamps
- Batch processing support (via Rust API)
- Exports results to your Desktop as meta_<filename>.txt

Quick Start
-----------

1. **Build and install the Python bindings:**
   (Already done if you see this file)

2. **Usage:**

   ```sh
   python3 meta-inspector.py /path/to/your/file
   ```

   - Example:

     ```sh
     python3 meta-inspector.py /home/mohdismailmatasin/Desktop/test.jpg
     ```

3. **Output:**
   - Results are printed in the terminal
   - A file named `meta_<filename>.txt` is created on your Desktop with all details

Requirements
------------

- Python 3.8–3.13
- Rust toolchain (for development/building)
- Linux (tested), should work on macOS/Windows with minor changes

How it Works
------------

- All core logic (metadata extraction, hashing, timestamps) is written in Rust for speed and reliability
- Python is used only for a simple, user-friendly interface
- No sensitive data is sent anywhere; all processing is local

For Developers
--------------

- Rust code: see `src/`
- Python interface: see `meta-inspector.py`
- To rebuild the Python bindings after code changes:

  ```sh
  PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 python3 -m maturin develop --release
  ```

License
-------

MIT
