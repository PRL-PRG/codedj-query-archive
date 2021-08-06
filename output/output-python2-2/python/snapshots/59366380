#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import unittest

import test_socket
import test_parser
import test_model
import test_storage
import test_alias
import test_history

socket = unittest.makeSuite(test_socket.TestSocket)
parser = unittest.makeSuite(test_parser.TestParser)
model  = unittest.makeSuite(test_model.TestCircularList)
storage = unittest.makeSuite(test_storage.TestStorage)
storage2 = unittest.makeSuite(test_storage.TestStorage2)
alias = unittest.makeSuite(test_alias.TestAlias)
history = unittest.makeSuite(test_history.TestHistory)

alltests = unittest.TestSuite((socket, parser, alias,
                               model, storage, storage2,
                               history))
unittest.TextTestRunner(verbosity=2).run(alltests)
