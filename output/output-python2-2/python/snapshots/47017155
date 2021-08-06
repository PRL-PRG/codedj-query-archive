#!/usr/bin/env python

import os
import os.path
import math

import dejumble.organizer
from dejumble.organizer import *

iso9660_increase_regex = re.compile('^(.*)~(\d+)$')

class ISO9660Organizer(Organizer):
    def generatepaths(self):
        for realpath in self.filter.filelist():
            # Add all subdirectories first
            currentpath = os.sep
            
            for pathpart in pathparts(realpath):
                currentpath = os.path.join(pathpart)
                self.addfile(currentpath)

    def paths(self, realpath):
        yield os.path.join(os.path.dirname(realpath), self._path(os.path.basename(realpath)))

    def increasefilename(self, filename):
        root, ext = os.path.splitext(filename)
    
        num = 1
        m = iso9660_increase_regex.match(root)
    
        if not m is None:
            num = int(m.group(2)) + 1
            filename = m.group(1)

        return self._path(realpath, num)

    def _path(self, filename, num=0):
        root, ext = os.path.splitext(filename)

        # TODO: exclude all non valid characters
        root = root.replaceall(' ', '')
        root = root.replaceall('+', '_')

        size = int(6 - math.log10(len(str(num))))
        
        if len(root) > size or num > 0:
            if num == 0:
                num = 1
            return "%s~%s%s" % (root.upper()[0:size], num, ext.upper()[0:4])
        else:
            return "%s%s" % (root.upper(), ext.upper()[0:4])

