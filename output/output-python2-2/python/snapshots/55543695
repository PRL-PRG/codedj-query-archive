#! /usr/bin/env python
import sys
import os
import zipfile
import pygtk
import gtk
import pygame, pygame.display
from pygame.locals import *
from sugar.activity import activity
from readtoolbar import ReadToolbar
from gettext import gettext as _

i=0

class ViewSlidesActivity(activity.Activity):
    def __init__(self, handle):
        "The entry point to the Activity"
        activity.Activity.__init__(self, handle)
        self.connect("expose_event", self.area_expose_cb)
        self.connect("key_press_event", self.keypress_cb)
        toolbox = activity.ActivityToolbox(self)
        self._read_toolbar = ReadToolbar()
        toolbox.add_toolbar(_('Read'), self._read_toolbar)
        self._read_toolbar.show()
        self.set_toolbox(toolbox)
        toolbox.show()
        self.image = gtk.Image()
        self.set_canvas(self.image)
        self.show_image("ViewSlides.jpg")
        self._read_toolbar.set_activity(self)

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
        self.save_extracted_file(self.zf, self.image_files[page])
        self.show_image("/tmp/" + self.image_files[page])
        os.remove("/tmp/" + self.image_files[page])
        self._read_toolbar.set_current_page(page)
        self.page = page

    def set_current_page(self, page):
        self.page = page

    def next_page(self):
        page = self.page
        page = page + 1
        if page >= len(self.image_files): page=len(self.image_files) - 1
        self.save_extracted_file(self.zf, self.image_files[page])
        self.show_image("/tmp/" + self.image_files[page])
        os.remove("/tmp/" + self.image_files[page])
        self._read_toolbar.set_current_page(page)
        self.page = page

    def area_expose_cb(self, area, event):
        return False

    def show_page(self, page):
        self.save_extracted_file(self.zf, self.image_files[page])
        self.show_image("/tmp/" + self.image_files[page])
        os.remove("/tmp/" + self.image_files[page])
        
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
        new_width = image_width
        new_height = image_height
        if image_width >= image_height:
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
        filebytes = zipfile.read(filename)
        f = open("/tmp/" + filename, 'w')
        try:
            f.write(filebytes)
        finally:
            f.close

    def read_file(self, file_path):
        """Load a file from the datastore on activity start"""
        self._load_document(file_path)

    def _load_document(self, file_path):
        "Read the Zip file containing the images"
        self.zf = zipfile.ZipFile(file_path, 'r')
        self.image_files = self.zf.namelist()
        self.image_files.sort()
        self.page = int(self.metadata.get('current_image', '0'))
        self.save_extracted_file(self.zf, self.image_files[self.page])
        currentFileName = "/tmp/" + self.image_files[self.page]
        self.show_image(currentFileName)
        os.remove(currentFileName)
        self._read_toolbar.set_total_pages(len(self.image_files))
        self._read_toolbar.set_current_page(self.page)

    def write_file(self, file_path):
        "Save meta data for the file."
        self.metadata['current_image'] =str(self.page)
