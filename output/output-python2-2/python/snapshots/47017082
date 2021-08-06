#!/usr/bin/env python

import os.path

import dejumble.organizer
from dejumble.organizer import *


class FlatOrganizer(Organizer):
    def paths(self, realpath):
        yield addtrailingslash(os.path.basename(realpath));
