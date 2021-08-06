#!/usr/bin/env python

import dejumble.organizer
from dejumble.organizer import *


class OriginalOrganizer(Organizer):
    def _filelist(self, path):
	# TODO: dont use basename but remove root from path
        return increasefilelist([ os.file.basename(path) for path in self.cache.filelist() ])
