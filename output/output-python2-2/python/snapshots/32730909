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
from btpdwebui.config.configfile import ConfigFile

# global install information
PROGRAM_NAME    = 'BTPDWebui'
PROGRAM_VERSION = '0.1'
HOME_DIR     = os.path.join(os.path.expanduser('~'), '.btpdwebui')
CONFIG_FILE  = os.path.join(HOME_DIR, 'config')
PID_FILE     = os.path.join(HOME_DIR, 'pid')
STATIC_DIR   = os.path.join(__path__[0], '../data/static') 
TEMPLATE_DIR = os.path.join(__path__[0], '../data/templates') 

# Validation functions for the configuration
def valid_int(x):
    try:
        x = int(x)
    except ValueError:
        return False
    return True

def valid_port(x):
    if valid_int(x):
        x = int(x)
        return (x > 0 and x <= 65535)
    return False

def valid_ascii(x):
    for s in x:
        if ord(s) > 127:
            return False
    return True

# config defaults structure
_defaults = {
    'port'              : ('12321', valid_port,  'HTTP Server port'),
    'username'          : ('btpd',  valid_ascii, 'Web UI username'),
    'password'          : ('btpd',  valid_ascii, 'Web UI password'),
    'update_frequency'  : ('2',     valid_int,   'Web UI update interval (in seconds)'),
    'content_directory' : (os.path.join(HOME_DIR, 'content'), None, 'Base directory for downloaded content')
}

# user editable configuration
userconf = ConfigFile(CONFIG_FILE, _defaults)

