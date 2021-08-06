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


class DevClient(Exception):
    """
    Base class for all exceptions from DevClient.
    """
    pass


class BufferUnderSize(DevClient):
    """
    Handling error on losing data because buffer size is not sufficient.
    """
    pass


class ConnectionRefused(DevClient):
    """
    Handling error on establish connection.
    """
    pass


class ConnectionNotFound(DevClient):
    """
    Class for handling error on finding connection to load.
    """
    pass


class ConnectionLost(DevClient):
    """
    Handling error on losing connection.
    """
    pass


class IPCError(DevClient):
    """
    Handling error on Inter-Process Communication
    """
    pass