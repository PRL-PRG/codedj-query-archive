#!/usr/bin/python

from PIL import Image
import sys, os

fn = sys.argv[1]

im = Image.open(fn)

def conv(level):
    # 255 in L mode is white
    #   1 in 1 mode is white
    if level > 175:
        return 1
    else:
        return 0

newfn = os.path.splitext(fn)[0] + ".pbm"

# explicit load to work around 1.0.15 bug
im.load()
im.point(conv, "1").save(newfn)

os.system("scan-script %s" % newfn)
