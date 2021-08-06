"""
	xsoap.py:  an X rpc module

	Copyright 2004 Kenneth Hayber <khayber@socal.rr.com>
		All rights reserved.

	This module implements a method to register a window to receive rpc commands and
	parameters from another application.

	Using the X root window, the server registers a property that gives both the name
	of the server/application and the X window ID of a window to receive messages.

	The server may either allow the register_server() function to create a new hidden window
	to receive these messages, or it may pass in an existing window for this purpose.

	The client can determine if a server is already running on the current display by
	checking the return value of get_server().

	There is currently no mechanism for passing values back from the server to the client.

	(Note: despite the name, this module does not (yet) implement any form of SOAP,
	 it is just a simple rpc mechanism)

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License.

	This program is distributed in the hope that it will be useful
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program; if not, write to the Free Software
	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

	(the idea of using X properties for rpc was taken from ROX-Filer, by Thomas Leonard)
"""

import gtk
from types import *

_callback = None

def register_server(appl_id, appl_window=None):
	"""register the soap server name and window id"""
	if not appl_id or not isinstance(appl_id, str):
		raise TypeError, "appl_id must be a string"

	#allow passing in an existing window...
	if appl_window:
		if not isinstance(window, gtk.Window):
			raise TypeError, "appl_window is not a gtk.Window"
		window = appl_window
	#or create our own.
	else:
		window = gtk.Window()
		#need to realize (or show) the window to get a gdkWindow
		window.realize()

	#set the appl_id and xid to the X root window for clients to find.
	gtk.gdk.get_default_root_window().property_change(gtk.gdk.atom_intern(appl_id, False),
				"WINDOW", 32, gtk.gdk.PROP_MODE_REPLACE, [window.window.xid])

	#Register for notification events
	window.add_events(gtk.gdk.PROPERTY_CHANGE_MASK)
	window.connect('property-notify-event', handle_event, gtk.gdk.atom_intern("_XSOAP", False))


def unregister_server(appl_id):
	"""remove the application xid from the root window"""
	if not appl_id or not isinstance(appl_id, str):
		raise TypeError, "appl_id must be a string"

	gtk.gdk.get_default_root_window().property_delete(gtk.gdk.atom_intern(appl_id, False))


def register_callback(callback):
	"""register the client's callback function for later"""
	global _callback
	_callback = callback


def handle_event(window, event, property):
	"""parse the property data and call the client's parser"""
	global _callback

	#msg is a list of atoms
	msg = window.window.property_get(property)

	#the use of str() gets the string value from the atom
	if msg:
		cmd = str(msg[2][0])
		args = []
		for a in msg[2][1:]:
			args.append(str(a))

		_callback(window, cmd, args)

		#don't want to do this again and again...
		window.window.property_delete(property)


def get_server(appl_id):
	"""query the root X window to find the server"""
	if not appl_id or not isinstance(appl_id, str):
		raise TypeError, "appl_id must be a string"

	appl_window = None

	#appl_id is the same as used in register_server
	#the xid is the X window id of the server's rpc window
	root_window = gtk.gdk.get_default_root_window()
	xid = root_window.property_get(gtk.gdk.atom_intern(appl_id, False))
	if xid:
		#the xid is in a list in a tuple: c from (a, b, [c])
		xid = xid[2][0]

		#create a gdk window from the xid
		appl_window = gtk.gdk.window_foreign_new(long(xid))

	#if we didn't find the window we return None
	return appl_window


def send_message(window, message, args):
	"""send a message and optional arguments to the server"""

	if not isinstance(window, gtk.gdk.Window):
		raise TypeError, "window must be a valid gtk.gdk.Window"
	if not isinstance(message, str):
		raise TypeError, "message must be a string"
	if args and not isinstance(args, list):
		raise TypeError, "args must be a list"

	#format the message and arguments as a list of atoms
	atom_args = [gtk.gdk.atom_intern(message, False)]
	for x in args:
		atom_args.append(gtk.gdk.atom_intern(x))

	#send the message by setting the _XSOAP property on the server window
	window.property_change(gtk.gdk.atom_intern("_XSOAP", False),
		"ATOM", 32, gtk.gdk.PROP_MODE_REPLACE, atom_args)

	#make sure the message gets through (triggers a notification event)
	gtk.gdk.flush()

