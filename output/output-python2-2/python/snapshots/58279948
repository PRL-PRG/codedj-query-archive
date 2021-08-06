#!/usr/bin/python

import tempfile
import subprocess
import os
import shutil


def tmpmeld():
    tempdir = tempfile.mkdtemp(prefix="meld_")
    try:
         a = os.path.join(tempdir, "a")
         b = os.path.join(tempdir, "b")
         open(a, "w").close()
         open(b, "w").close()
         subprocess.call(["meld", a, b])
    finally:
         shutil.rmtree(tempdir)


if __name__ == "__main__":
    tmpmeld()

