#!/usr/bin/python

import os
import pprint
import re
import shutil
import subprocess
import tempfile

import gtk


def tmpmeld(a, b):
    tempdir = tempfile.mkdtemp(prefix="meld_")
    try:
         a_filepath = os.path.join(tempdir, "a")
         b_filepath = os.path.join(tempdir, "b")
         open(a_filepath, "w").write(a)
         open(b_filepath, "w").write(b)
         subprocess.call(["meld", a_filepath, b_filepath])
    finally:
         shutil.rmtree(tempdir)


def main(clipboard, text, data):
    text = text.strip()
    m = re.compile("^AssertionError: (u?'.*') != (u?'.*')$").match(text)
    if m:
        tmpmeld(eval(m.group(1)), eval(m.group(2)))
        
    m = re.compile("^AssertionError: (\[.*\]) != (\[.*\])$").match(text)
    if m:
        tmpmeld(pprint.pformat(eval(m.group(1))),
                pprint.pformat(eval(m.group(2))))
        
    gtk.main_quit()


if __name__ == '__main__':
    gtk.Clipboard().request_text(main)
    gtk.main()

