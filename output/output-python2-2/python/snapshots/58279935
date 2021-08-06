#!/usr/bin/python

import os
import subprocess
import sys


def unrar_dir(top):
    for root, dirs, files in os.walk(top):
        for name in files:
            if (name.endswith(".part01.rar") or (
                    name.endswith(".rar") and ".part" not in name) or
                    name.endswith(".001")):
                cmd = ["unrar", "x", os.path.join(root, name)]
                subprocess.call(cmd) 
            

if __name__ == "__main__":
    for arg in sys.argv[1:]:
        unrar_dir(arg)
    raw_input()
