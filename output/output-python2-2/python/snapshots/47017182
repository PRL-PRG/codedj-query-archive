#!/usr/bin/env python

import dejumble.organizer
from dejumble.organizer import *


class DateOrganizer(TagOrganizer):
    def __init__(self, provider):
        TagOrganizer.__init__(self, provider, 'date')

    def _refreshcache(self):
        for filename in filter(ignoretag, self.provider.filelist()):
            stats = os.stat(self.provider.realpath(addtrailingslash(filename)))
            lastmod = time.localtime(stats[8])
            today = time.localtime()
            self.provider.storage.tag(filename, self.category, time.strftime('%Y %B', lastmod))
            if time.strftime('%x', today) == time.strftime('%x', lastmod):
                self.provider.storage.tag(filename, self.category, _('Today'))
            if time.strftime('%Y%W', today) == time.strftime('%Y%W', lastmod):
                self.provider.storage.tag(filename, self.category, _('This Week'))
            lastweek = time.localtime(time.time() - 7 * 24 * 60 * 60)
            if time.strftime('%Y%W', lastweek) == time.strftime('%Y%W', lastmod):
                self.provider.storage.tag(filename, self.category, _('Last Week'))

