#!/usr/bin/env python

import dejumble.organizer
from dejumble.organizer import *


class FlatOrganizer(Organizer):
    def _filelist(self, path):
        return increasefilelist([ os.file.basename(path) for path in self.cache.filelist() ])
