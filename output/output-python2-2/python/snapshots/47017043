#!/usr/bin/env python

import os.path

import dejumble.organizer
from dejumble.organizer import *


class FlatOrganizer(Organizer):
    def __init__(self, cache):
        Organizer.__init__(self, cache, False)

    def generatepaths(self, realpath):
        if not os.path.isdir(realpath):
            yield addtrailingslash(os.path.basename(realpath))
