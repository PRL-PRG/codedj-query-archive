# Copyright (C) Scott Walker 2007 <iswalker at gmail dot com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2, or (at your option)
# any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with this program.  If not, write to:
# 	The Free Software Foundation, Inc.,
# 	51 Franklin Street, Fifth Floor
# 	Boston, MA  02110-1301, USA.
#

import os

class ConfigFile(object):
    """Simple configuration file class with validation"""

    def __init__(self, cfile, defaults):
        self.cfile = cfile
        self.defaults = defaults
        self.config = {}

    def __setitem__(self, key, value):
        if not self.defaults.has_key(key):
            raise KeyError('invalid config item: %s' % key)
        validator = self.defaults[key][1]
        if validator and not validator(value):
            raise ValueError('invalid config value: %s' % value)
        self.config[key] = value

    def __getitem__(self, key):
        if self.config.has_key(key):
            return self.config[key]
        elif self.defaults.has_key(key):
            return self.defaults[key][0]
        raise KeyError('config item does not exist: %s' % key)

    def keys(self):
        return self.defaults.keys()  

    def load(self):
        fp = open(self.cfile, mode='r')
        try:
            for line in fp:
                line = line.strip(' \n\r\t')
                if len(line) == 0 or line.startswith('#'):
                    continue
                kv = [x.strip(' \n\r\t') for x in line.split('=', 1)]
                if len(kv) == 2 and len(kv[0]) > 0:
                    self[kv[0]] = kv[1]
                else:
                    raise SyntaxError('invalid line in config: %s' % line)
        finally:
            fp.close()

    def create_default(self, force=False):
        if not force and os.path.exists(self.cfile):
            return
        fp = open(self.cfile, mode='w')
        for k in self.defaults.keys():
            fp.write('## %s\n' % self.defaults[k][2])
            fp.write('#%s = %s\n\n' % (k, self.defaults[k][0]))
        fp.close()
        os.chmod(self.cfile, 0600)


