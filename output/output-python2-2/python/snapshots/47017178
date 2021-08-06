#!/usr/bin/env python

import dejumble.organizer
from dejumble.organizer import *



class DocumentsOrganizer(TagOrganizer):
    def __init__(self, organizer):
        TagOrganizer.__init__(self, organizer, 'filetype')
        self.filetypes = readconfig('filetypes')
        for filetype, extensions in self.filetypes.iteritems():
            self.filetypes[filetype] = map(extensionregex, extensions.split(','))

    def _refreshcache(self):
        for filename in filter(ignoretag, self.filelist()):
            hastag = False
            for filetype, extensions in self.filetypes.iteritems():
                for extension in extensions:
                    if not extension.search(filename) == None:
                        self.cache.tag(filename, self.category, _(filetype))
                        hastag = True
            if not hastag:
                self.cache.tag(filename, self.category, _('Other'))

