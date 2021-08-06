


import os
import time


pid = os.fork()

if pid == 0:
    os._exit(1)
else:
    time.sleep(1)
    # perhaps there is a problem checking the status of zombie?
    # process is no longer "alive" tho
    os.kill(pid, 0)
    # can still wait ok?
    pid, status = os.waitpid(pid, os.WNOHANG)
    print pid, os.WIFEXITED(status), os.WEXITSTATUS(status)


import pexpect

p = pexpect.spawn("true")
time.sleep(1)
p.wait() # 



# http://pexpect.svn.sourceforge.net/viewvc/pexpect/trunk/pexpect/pexpect.py?view=log
# see rev 78 and 77 comments

"""
isalive - should use os.kill ?

wait - 2nd os.waitpid has problems if child dead on alive() call

terminate - doesn't use signal.SIGTERM ?
"""

