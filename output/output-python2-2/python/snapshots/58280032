#!/usr/bin/python

"""
enable back and forward keys in firefox

    http://snarfed.org/space/thinkpad+keys+in+firefox
    https://launchpad.net/ubuntu/+source/xkeyboard-config/+bug/20204
"""

import tempfile
import shutil
import subprocess

call = subprocess.check_call

if __name__ == "__main__":
    tempdir = tempfile.mkdtemp(prefix="firefox_keys_")
    try:
        call(["unzip", "/usr/share/firefox/chrome/browser.jar"], cwd=tempdir)
        p = subprocess.Popen(["patch", "-p6"], cwd=tempdir,
                             stdin=subprocess.PIPE)
        p.stdin.write("""\
--- browser.xul    2007-06-03 21:42:27.223182495 +0100
+++ /usr/share/firefox/chrome/browser/content/browser/browser.xul    2007-06-03 21:51:05.165814914 +0100
@@ -250,6 +250,8 @@
     <key keycode="VK_BACK" command="cmd_handleShiftBackspace" modifiers="shift"/>
     <key id="goBackKb"  keycode="VK_LEFT" command="Browser:Back" modifiers="alt"/>
     <key id="goForwardKb"  keycode="VK_RIGHT" command="Browser:Forward" modifiers="alt"/>
+    <key id="goBackKb" keycode="VK_F19" command="Browser:Back"/>
+    <key id="goForwardKb" keycode="VK_F20" command="Browser:Forward"/>
     <key id="goBackKb2" key="&goBackCmd.commandKey;" command="Browser:Back" modifiers="accel"/>
     <key id="goForwardKb2" key="&goForwardCmd.commandKey;" command="Browser:Forward" modifiers="accel"/>
     <key id="goHome" keycode="VK_HOME" command="Browser:Home" modifiers="alt"/>
""")
        p.stdin.close()
        p.wait()
        call(["zip", "-0r", "browser.jar", "content"], cwd=tempdir)
        call(["sudo", "install", "-m", "755", "browser.jar",
              "/usr/share/firefox/chrome/browser.jar"], cwd=tempdir)
        #call(["bash"], cwd=tempdir)
    finally:
        shutil.rmtree(tempdir)

