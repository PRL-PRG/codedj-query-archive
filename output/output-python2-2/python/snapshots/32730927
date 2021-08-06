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

import os.path
from btpdwebui.webui.htmltmpl import TemplateManager, TemplateProcessor

base_directory = '.'    # directory containing templates
tmpl_extension = 'tmpl' # file extension of template files

use_cache = False  # use the template cache?
_cache = {}        # cache of previously loaded templates

# args: include, max_include, precompile, comments, gettext, debug
manager = TemplateManager(False, False, False, True, False, False)

# args: html_escape, magic_vars, global_vars, debug
processor = TemplateProcessor(False, True, False, False)

def set(name, value):
    """Set a single template variable"""
    processor.set(name, value)

def render(tmpl_name):
    """Load, process, and return template data"""
    if not use_cache or not _cache.has_key(tmpl_name):
        # we need to load the template
        filename = tmpl_name + '.' + tmpl_extension
        filename = os.path.join(base_directory, filename)
        tmplfile = manager.prepare(filename)
        # if we are using the cache, save it for next time
        if use_cache: 
            _cache[tmplname] = tmplfile
    else:
        # use the template in the cache
        tmplfile = _cache[tmplname]
    tmpldata = processor.process(tmplfile)
    processor.reset()
    return tmpldata

