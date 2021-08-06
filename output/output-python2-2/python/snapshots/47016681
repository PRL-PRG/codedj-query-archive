import os
import time

from ..organizer import TagOrganizer


class DateOrganizer(TagOrganizer):

    def __init__(self, cache):
        TagOrganizer.__init__(self, cache, 'date')

    def generatetags(self, realpath):
        stats = os.stat(realpath)
        lastmod = time.localtime(stats[8])
        today = time.localtime()
        lastweek = time.localtime(time.time() - 7 * 24 * 60 * 60)

        self.tag(realpath, self.category, time.strftime('%Y %B', lastmod))

        if time.strftime('%x', today) == time.strftime('%x', lastmod):
            self.tag(realpath, self.category, _('Today'))

        if time.strftime('%Y%W', today) == time.strftime('%Y%W', lastmod):
            self.tag(realpath, self.category, _('This Week'))

        if time.strftime('%Y%W', lastweek) == time.strftime('%Y%W', lastmod):
            self.tag(realpath, self.category, _('Last Week'))
