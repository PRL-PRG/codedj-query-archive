import os

from ..cache import Cache
from .. import util
from ..fs import getserver


class SandboxCache(Cache):

    class DejumbleFile(Cache.DejumbleFile):

        def getfdandfile(self, path, flags, *mode):
            # TODO: when changing a file don't open the original but copy
            # and open the copy
            realpath = getserver().organizer.realpath(path)
            fd = os.open(realpath, flags, *mode) #IGNORE:W0142
            file = os.fdopen(fd, util.flags2mode(flags))
            return (fd, file)
