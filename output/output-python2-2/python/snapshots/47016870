import os.path

from ..organizer import Organizer


class FlatOrganizer(Organizer):
    def __init__(self, cache):
        Organizer.__init__(self, cache, False)

    def generatepaths(self, realpath):
        if not os.path.isdir(realpath):
            yield addtrailingslash(os.path.basename(realpath))
