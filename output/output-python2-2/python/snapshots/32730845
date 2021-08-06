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

from distutils.core import setup

setup(name='BtpdWebui',
      version='0.2',
      description='Webui for the Bittorrent Protocol Daemon',
      author='Scott Walker',
      author_email='iswalker@gmail.com',
      url='http://code.google.com/p/btpd-webui/',
      packages=['btpdwebui', 
                'btpdwebui.btpd', 
                'btpdwebui.config', 
                'btpdwebui.webui'],
      package_data={'btpdwebui' : ['data/static/*', 'data/templates/*']},
      scripts=['scripts/btpd-webui-server', 'scripts/btpd-webui'],
      requires=['twisted.web', 
                'twisted.internet', 
                'twisted.application'])

