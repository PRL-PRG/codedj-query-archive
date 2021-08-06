from .. import util
from ..organizer import TagOrganizer


class DocumentsOrganizer(TagOrganizer):

    def __init__(self, cache):
        TagOrganizer.__init__(self, cache, 'filetype')
        self.filetypes = util.readconfig('filetypes')
        for filetype, extensions in self.filetypes.items():
            self.filetypes[filetype] = map(util.extensionregex, #IGNORE:W0141
                                           extensions.split(','))

    def generatetags(self, realpath):
        hastag = False
        for filetype, extensions in self.filetypes.iteritems():
            for extension in extensions:
                if not extension.search(realpath) == None:
                    self.tag(realpath, self.category, _(filetype))
                    hastag = True
        if not hastag:
            self.tag(realpath, self.category, _('Other'))
