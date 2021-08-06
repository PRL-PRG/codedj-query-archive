#!/usr/bin/env python

import os
import time

import dejumble.organizer
from dejumble.organizer import *


class DateOrganizer(TagOrganizer):
    def __init__(self, cache):
        TagOrganizer.__init__(self, cache, 'date')

    def generatetags(self):
        for filename in filter(ignoretag, self.cache.filelist()):
            stats = os.stat(filename)
            lastmod = time.localtime(stats[8])
            today = time.localtime()
            self.tag(filename, self.category, time.strftime('%Y %B', lastmod))
            if time.strftime('%x', today) == time.strftime('%x', lastmod):
                self.tag(filename, self.category, _('Today'))
            if time.strftime('%Y%W', today) == time.strftime('%Y%W', lastmod):
                self.tag(filename, self.category, _('This Week'))
            lastweek = time.localtime(time.time() - 7 * 24 * 60 * 60)
            if time.strftime('%Y%W', lastweek) == time.strftime('%Y%W', lastmod):
                self.tag(filename, self.category, _('Last Week'))

