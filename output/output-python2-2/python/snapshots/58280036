#!/usr/bin/python

import os


moves = []
for root, dirs, files in os.walk(os.getcwd()):
    for name in files:
        src = os.path.join(root, name)
        root_basename = os.path.basename(root)
        if not name.startswith(root_basename):
            name = root_basename + name
        dst = os.path.join(os.path.dirname(root), name)
        dst = dst.replace(" ", "_")
        moves.append((src, dst))


for src, dst in moves:
    os.rename(src, dst)

