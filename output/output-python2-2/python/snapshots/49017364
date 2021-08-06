#!/usr/bin/env python
# -*- coding: UTF-8 -*-

# Copyright 2006-2007 (C) Raster Software Vigo (Sergio Costas)
# Copyright 2006-2007 (C) Peter Gill - win32 parts

# This file is part of DeVeDe
#
# DeVeDe is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# DeVeDe is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.


import gtk

def show_error(message,arbol):

	label=arbol.get_widget("l_error")
	label.set_text(message)
	werror=arbol.get_widget("werror")
	werror.show()


def show_error2(message,arbol):

	w=arbol.get_widget("error2_label")
	w.set_text(message)
	w=arbol.get_widget("werror2")
	w.show()			
	return


def global_delete(arg1,arg2):

	""" Just hides the specified widget. Used as callback for nearly all the
	DELETE event from windows """

	arg1.hide()
	return True


def refresh_screen():

	""" Refresh the widgets in the screen. Used mainly to update the progress
	bars during file conversion """

	while gtk.events_pending():
		gtk.main_iteration()


def alwaystrue(args="",arg2=""):

	""" A stub function that always return True, for windows that must not be hiden
	when clicking the "close" button """

	return True
