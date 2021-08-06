#! /usr/bin/env python

# Copyright (C) 2008 James D. Simmons
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
import sys
import os
import logging
import tempfile
import time
import zipfile
from zipfile import BadZipfile
import pygtk
import gtk
import string
import pygame, pygame.display
from pygame.locals import *
from sugar.activity import activity
from sugar import network
from sugar.datastore import datastore
from readtoolbar import ReadToolbar
from gettext import gettext as _
import pango
import dbus
import gobject
import hippo
import telepathy
import shutil
from decimal import *
from sugar.presence import presenceservice

_HARDWARE_MANAGER_INTERFACE = 'org.laptop.HardwareManager'
_HARDWARE_MANAGER_SERVICE = 'org.laptop.HardwareManager'
_HARDWARE_MANAGER_OBJECT_PATH = '/org/laptop/HardwareManager'

_TOOLBAR_READ = 1

_logger = logging.getLogger('view-slides')

class ReadHTTPRequestHandler(network.ChunkedGlibHTTPRequestHandler):
    def translate_path(self, path):
        return self.server._filepath

class ReadHTTPServer(network.GlibTCPServer):
    def __init__(self, server_address, filepath):
        self._filepath = filepath
        network.GlibTCPServer.__init__(self, server_address, ReadHTTPRequestHandler)

READ_STREAM_SERVICE = 'read-activity-http'

class ViewSlidesActivity(activity.Activity):
    def __init__(self, handle):
        "The entry point to the Activity"
        activity.Activity.__init__(self, handle)

        self._fileserver = None
        self._object_id = handle.object_id

        self.connect("expose_event", self.area_expose_cb)
        self.connect("delete_event", self.delete_cb)
        toolbox = activity.ActivityToolbox(self)
        self._read_toolbar = ReadToolbar()
        toolbox.add_toolbar(_('Read'), self._read_toolbar)
        self._read_toolbar.show()
        self.set_toolbox(toolbox)
        toolbox.show()
        self.scrolled = gtk.ScrolledWindow()
        self.scrolled.set_policy(gtk.POLICY_NEVER, gtk.POLICY_AUTOMATIC)
        self.scrolled.props.shadow_type = gtk.SHADOW_NONE
        self.image = gtk.Image()
        self.image.show()
        self.scrolled.add(self.image)
        self.set_canvas(self.scrolled)
        self.scrolled.show()
        self.scrolled.connect("key_press_event", self.keypress_cb)
        self.show_image("ViewSlides.jpg")
        self._read_toolbar.set_activity(self)
        self.page = 0
        self.temp_filename = ''
        self.saved_screen_width = 0
        self.scrolled.grab_focus()
        
        # Set up for idle suspend
        self._idle_timer = 0
        self._service = None

        # start with sleep off
        self._sleep_inhibit = True

        fname = os.path.join('/etc', 'inhibit-ebook-sleep')
        if not os.path.exists(fname):
            try:
                bus = dbus.SystemBus()
                proxy = bus.get_object(_HARDWARE_MANAGER_SERVICE,
                                       _HARDWARE_MANAGER_OBJECT_PATH)
                self._service = dbus.Interface(proxy, _HARDWARE_MANAGER_INTERFACE)
                self.connect("focus-in-event", self._focus_in_event_cb)
                self.connect("focus-out-event", self._focus_out_event_cb)
                self.connect("notify::active", self._now_active_cb)

                logging.debug('Suspend on idle enabled')
            except dbus.DBusException, e:
                _logger.info('Hardware manager service not found, no idle suspend.')
        else:
            logging.debug('Suspend on idle disabled')
    
        # start on the read toolbar
        self.toolbox.set_current_toolbar(_TOOLBAR_READ)
        self.unused_download_tubes = set()
        self._want_document = True
        self.connect("shared", self._shared_cb)
        h = hash(self._activity_id)
        self.port = 1024 + (h % 64511)

        self.is_received_document = False
        
        presenceService = presenceservice.get_instance()
        xoOwner = presenceService.get_owner()
        self.owner = xoOwner.props.nick
        
        if self._shared_activity:
            # We're joining
            if self.get_shared():
                # Already joined for some reason, just get the document
                self._joined_cb(self)
            else:
                # Wait for a successful join before trying to get the document
                self.connect("joined", self._joined_cb)
        # uncomment this and adjust the path for easier testing
        #else:
        #    self._load_document('file:///home/smcv/tmp/test.pdf')

    def keypress_cb(self, widget, event):
        "Respond when the user presses Escape or one of the arrow keys"
        keyname = gtk.gdk.keyval_name(event.keyval)
        if keyname == 'KP_Right':
            self.next_page()
            return True
        if keyname == 'Up' or keyname == 'KP_Up':
            self.previous_page()
            return True
        if keyname == 'KP_Left':
            self.previous_page()
            return True
        if keyname == 'Down' or keyname == 'KP_Down':
            self.next_page()
            return True
        return False

    def previous_page(self):
        page = self.page
        page=page-1
        if page < 0: page=0
        if self.save_extracted_file(self.zf, self.image_files[page]) == True:
            fname = "/tmp/" + self.make_new_filename(self.image_files[page])
            self.show_image(fname)
            os.remove(fname)
        self._read_toolbar.set_current_page(page)
        self.page = page

    def set_current_page(self, page):
        self.page = page

    def next_page(self):
        page = self.page
        page = page + 1
        if page >= len(self.image_files): page=len(self.image_files) - 1
        if self.save_extracted_file(self.zf, self.image_files[page]) == True:
            fname = "/tmp/" + self.make_new_filename(self.image_files[page])
            self.show_image(fname)
            os.remove(fname)
        self._read_toolbar.set_current_page(page)
        self.page = page

    def area_expose_cb(self, area, event):
        screen_width = gtk.gdk.screen_width()
        screen_height = gtk.gdk.screen_height()
        print 'width', screen_width, 'height',  screen_height
        if self.saved_screen_width != screen_width and self.saved_screen_width != 0:
            self.show_page(self.page)
        self.saved_screen_width = screen_width
        return False

    def show_page(self, page):
        if self.save_extracted_file(self.zf, self.image_files[page]) == True:
            fname = "/tmp/" + self.make_new_filename(self.image_files[page])
            self.show_image(fname)
            os.remove(fname)
        
    def show_image(self, filename):
        "display a resized image in a full screen window"
        TOOLBOX_HEIGHT = 100
        # get the size of the fullscreen display
        screen_width = gtk.gdk.screen_width()
        screen_height = gtk.gdk.screen_height()
        screen_height = screen_height - TOOLBOX_HEIGHT
        # get the size of the image.
        im = pygame.image.load(filename)
        image_width, image_height = im.get_size()
        getcontext().prec = 7
        s_a_ratio = Decimal(screen_height) / Decimal(screen_width)
        i_a_ratio = Decimal(image_height) / Decimal(image_width)
        new_width = image_width
        new_height = image_height
        if s_a_ratio >= i_a_ratio:
            new_width = screen_width
            new_height = image_height * screen_width
            if image_width > 1:
                new_height /= image_width

            if new_height > screen_width:
                new_height *= screen_width
                if new_width > 1:
                    new_height /= new_width
                new_width = screen_width
        else:
            new_height = screen_height
            new_width = image_width * screen_height
            if image_height > 1:
                new_width /= image_height
            if new_width > screen_height:
                new_width *= screen_height
                if new_height > 1:
                    new_width /= new_height
                new_height = screen_height
        pixbuf = gtk.gdk.pixbuf_new_from_file(filename)
        scaled_buf = pixbuf.scale_simple(new_width, new_height, gtk.gdk.INTERP_BILINEAR)
        self.image.set_from_pixbuf(scaled_buf)
        self.image.show()
 
    def save_extracted_file(self, zipfile, filename):
        "Extract the file to a temp directory for viewing"
        try:
            filebytes = zipfile.read(filename)
        except BadZipfile, err:
            print 'Error opening the zip file: %s' % (err)
            return False    
        outfn = self.make_new_filename(filename)
        if (outfn == ''):
            return False
        f = open("/tmp/" + outfn, 'w')
        try:
            f.write(filebytes)
        finally:
            f.close
        return True

    def read_file(self, file_path):
        """Load a file from the datastore on activity start"""
        self._load_document(file_path)

    def delete_cb(self, widget, event):
        os.remove(self.temp_filename)
        print 'deleted file', self.temp_filename
        return False

    def make_new_filename(self, filename):
        partition_tuple = filename.rpartition('/')
        return partition_tuple[2]
    
    def _load_document(self, file_path):
        "Read the Zip file containing the images"
        partition_tuple = file_path.rpartition('/')
        self.temp_filename = '/tmp/' + self.owner + partition_tuple[2]
        shutil.copyfile (file_path, self.temp_filename)
        if zipfile.is_zipfile(self.temp_filename):
            self.zf = zipfile.ZipFile(self.temp_filename, 'r')
            self.image_files = self.zf.namelist()
            self.image_files.sort()
            i = 0
            valid_endings = ('.jpg', '.JPG', '.gif', '.GIF', '.tiff', '.TIFF', '.png', '.PNG')
            while i < len(self.image_files):
                newfn = self.make_new_filename(self.image_files[i])
                if newfn.endswith(valid_endings):
                    i = i + 1
                else:   
                    del self.image_files[i]
            self.page = int(self.metadata.get('current_image', '0'))
            self.save_extracted_file(self.zf, self.image_files[self.page])
            currentFileName = "/tmp/" + self.make_new_filename(self.image_files[self.page])
            self.show_image(currentFileName)
            os.remove(currentFileName)
            self._read_toolbar.set_total_pages(len(self.image_files))
            self._read_toolbar.set_current_page(self.page)
            # We've got the document, so if we're a shared activity, offer it
            if self.get_shared():
                self.watch_for_tubes()
                self._share_document()
        else:
            print 'not a zipfile', file_path

    def write_file(self, file_path):
        "Save meta data for the file."
        if not os.path.exists(self.temp_filename):
            print 'No file to save', self.temp_filename
            return

        if self.is_received_document == True and self.temp_filename != '':
            # This document was given to us by someone, so we have
            # to save it to the Journal.
            shutil.copyfile (self.temp_filename, file_path)
        self.metadata['current_image']  = str(self.page)
        if self.temp_filename != '':
            os.remove(self.temp_filename)

    # The code from here on down is for sharing.
    def _download_result_cb(self, getter, tempfile, suggested_name, tube_id):
        del self.unused_download_tubes

        _logger.debug("Got document %s (%s) from tube %u",
                      tempfile, suggested_name, tube_id)
        self._load_document(tempfile)
        _logger.debug("Saving %s to datastore...", tempfile)
        self.save()

    def _download_progress_cb(self, getter, bytes_downloaded, tube_id):
        total = getter._info.headers["Content-Length"]
        self._read_toolbar.set_downloaded_bytes(bytes_downloaded,  total)

    def _download_error_cb(self, getter, err, tube_id):
        _logger.debug("Error getting document from tube %u: %s",
                      tube_id, err)
        self._want_document = True
        gobject.idle_add(self._get_document)

    def _download_document(self, tube_id):
        chan = self._shared_activity.telepathy_tubes_chan
        iface = chan[telepathy.CHANNEL_TYPE_TUBES]
        addr = iface.AcceptStreamTube(tube_id,
                telepathy.SOCKET_ADDRESS_TYPE_IPV4,
                telepathy.SOCKET_ACCESS_CONTROL_LOCALHOST, 0,
                # 2, 0, 0, 
                utf8_strings=True)
        _logger.debug('Accepted stream tube: listening address is %r', addr)
        # SOCKET_ADDRESS_TYPE_IPV4 is defined to have addresses of type '(sq)'
        assert isinstance(addr, dbus.Struct)
        assert len(addr) == 2
        assert isinstance(addr[0], str)
        assert isinstance(addr[1], (int, long))
        assert addr[1] > 0 and addr[1] < 65536
        port = int(addr[1])

        getter = network.GlibURLDownloader("http://%s:%d/document"
                                           % (addr[0], port))
        getter.connect("finished", self._download_result_cb, tube_id)
        getter.connect("progress", self._download_progress_cb, tube_id)
        getter.connect("error", self._download_error_cb, tube_id)
        _logger.debug("Starting download to %s...", self._jobject.file_path)
        getter.start(self._jobject.file_path)
        self.is_received_document = True
        return False

    def _get_document(self):
        if not self._want_document:
            return False

        # Assign a file path to download if one doesn't exist yet
        if not self._jobject.file_path:
            self._jobject.file_path = os.path.join(tempfile.gettempdir(), '%i' % time.time())
            self._owns_file = True

        # Pick an arbitrary tube we can try to download the document from
        try:
            tube_id = self.unused_download_tubes.pop()
        except (ValueError, KeyError), e:
            _logger.debug('No tubes to get the document from right now: %s',
                          e)
            return False

        # Avoid trying to download the document multiple times at once
        self._want_document = False
        gobject.idle_add(self._download_document, tube_id)
        return False

    def _joined_cb(self, also_self):
        self.watch_for_tubes()
        gobject.idle_add(self._get_document)

    def _share_document(self):
        if self._jobject is None:
            self._jobject = datastore.get(self._object_id)
        elif not os.path.exists(self._jobject.get_file_path()):
            self._jobject.destroy()
            self._jobject = datastore.get(self._object_id)

        self._fileserver = ReadHTTPServer(("", self.port),
            self._jobject.get_file_path())

        # Make a tube for it
        chan = self._shared_activity.telepathy_tubes_chan
        iface = chan[telepathy.CHANNEL_TYPE_TUBES]
        self._fileserver_tube_id = iface.OfferStreamTube(READ_STREAM_SERVICE,
                {},
                telepathy.SOCKET_ADDRESS_TYPE_IPV4,
                # 2,
                ('127.0.0.1', dbus.UInt16(self.port)),
                telepathy.SOCKET_ACCESS_CONTROL_LOCALHOST, 0)
               # 0)

    def watch_for_tubes(self):
        tubes_chan = self._shared_activity.telepathy_tubes_chan

        tubes_chan[telepathy.CHANNEL_TYPE_TUBES].connect_to_signal('NewTube',
            self._new_tube_cb)
        tubes_chan[telepathy.CHANNEL_TYPE_TUBES].ListTubes(
            reply_handler=self._list_tubes_reply_cb,
            error_handler=self._list_tubes_error_cb)

    def _new_tube_cb(self, tube_id, initiator, tube_type, service, params,
                     state):
        _logger.debug('New tube: ID=%d initator=%d type=%d service=%s '
                      'params=%r state=%d', tube_id, initiator, tube_type,
                      service, params, state)
        if service == READ_STREAM_SERVICE:
            _logger.debug('I could download from that tube')
            self.unused_download_tubes.add(tube_id)
            # if no download is in progress, let's fetch the document
            if self._want_document:
                gobject.idle_add(self._get_document)

    def _list_tubes_reply_cb(self, tubes):
        for tube_info in tubes:
            self._new_tube_cb(*tube_info)

    def _list_tubes_error_cb(self, e):
        _logger.error('ListTubes() failed: %s', e)

    def _shared_cb(self, activity):
        # We initiated this activity and have now shared it, so by
        # definition we have the file.
        _logger.debug('Activity became shared')
        self.watch_for_tubes()
        self._share_document()

    # From here down is power management stuff.
    def _now_active_cb(self, widget, pspec):
        if self.props.active:
            # Now active, start initial suspend timeout
            if self._idle_timer > 0:
                gobject.source_remove(self._idle_timer)
            self._idle_timer = gobject.timeout_add(15000, self._suspend_cb)
            self._sleep_inhibit = False
        else:
            # Now inactive
            self._sleep_inhibit = True

    def _focus_in_event_cb(self, widget, event):
        self._sleep_inhibit = False
        self._user_action_cb(self)

    def _focus_out_event_cb(self, widget, event):
        self._sleep_inhibit = True

    def _user_action_cb(self, widget):
        if self._idle_timer > 0:
            gobject.source_remove(self._idle_timer)
        self._idle_timer = gobject.timeout_add(5000, self._suspend_cb)

    def _suspend_cb(self):
        # If the machine has been idle for 5 seconds, suspend
        self._idle_timer = 0
        if not self._sleep_inhibit:
            self._service.set_kernel_suspend()
        return False
        
