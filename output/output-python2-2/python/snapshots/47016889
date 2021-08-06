#!/usr/bin/env python

import dejumble.organizer
from dejumble.organizer import *


class DocumentsOrganizer(TagOrganizer):
    def __init__(self, cache):
        TagOrganizer.__init__(self, cache, 'filetype')
        self.filetypes = readconfig('filetypes')
        for filetype, extensions in self.filetypes.iteritems():
            self.filetypes[filetype] = map(extensionregex, extensions.split(','))

    def generatetags(self, realpath):
        hastag = False
        for filetype, extensions in self.filetypes.iteritems():
            for extension in extensions:
                if not extension.search(realpath) == None:
                    self.tag(realpath, self.category, _(filetype))
                    hastag = True
        if not hastag:
            self.tag(realpath, self.category, _('Other'))

