#!/usr/bin/env python

from __future__ import with_statement

import gettext
import commands
import sys
import os.path
import pickle
import pkg_resources

import wx

from dejumblefs import util

gettext.install('dejumblefs')

_TB_NEW = 1
_TB_OPEN = 2
_TB_SAVE = 3
_TB_MOUNT = 4
_TB_UMOUNT = 5
_TITLE = _('DejumbleFS Mounter')
_EXTENSION = 'dfo'
_DEJUMBLE_FILES = _('DejumbleFS options') + '(*.%s)|*.%s' % (_EXTENSION,
                                                             _EXTENSION)


class DejumbleFSUI(wx.App):

    def OnInit(self):
        self.main = MainWindow()
        self.main.Show()
        self.TopWindow = self.main

        return True


class MainWindow(wx.Frame):

    def __init__(self):
        wx.Frame.__init__(self, None, title=_TITLE,
                          style=wx.CAPTION|wx.CLOSE_BOX)

        # FIXME: set icon
        #iconpath = pkg_resources.resource_filename('dejumblefs.ui',
        #                                           'images/icon.png')
        #self.Icon = wx.IconFromLocation(wx.IconLocation(iconpath))
        self.panel = wx.Panel(self)

        externalborder = 10
        internalborder = 3

        self.vbox = vbox = wx.BoxSizer(wx.VERTICAL)

        ##################################
        # Mountpoint Options
        sizer = wx.FlexGridSizer(1, 2, hgap = 5)

        label = wx.StaticText(self.panel, label=_('Mount point:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.mountpoint = wx.DirPickerCtrl(self.panel, size=(300, -1))
        self.mountpoint.Bind(wx.EVT_DIRPICKER_CHANGED, self._setenabledall)
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.mountpoint, flag=wx.ALL, border=internalborder)

        self.nonempty = wx.CheckBox(self.panel, label='nonempty')
        sizer.AddSpacer(0)
        sizer.Add(self.nonempty, flag=wx.ALL, border=internalborder)

        self.noappledouble = wx.CheckBox(self.panel, label='noappledouble')
        sizer.AddSpacer(0)
        sizer.Add(self.noappledouble, flag=wx.ALL, border=internalborder)

        vbox.Add(sizer, flag=wx.ALL, border=externalborder)
        vbox.Add(wx.StaticLine(self.panel), 0, wx.ALL|wx.EXPAND)

        ##################################
        # Filter Options
        sizer = wx.FlexGridSizer(1, 2, hgap = 5)

        choices = ['CompleteDirectory', 'OriginalDirectory', 'Null', 'Shell']
        label = wx.StaticText(self.panel, label=_('Filter:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.filter = wx.ComboBox(self.panel, choices=choices,
                                  style=wx.CHOICEDLG_STYLE,
                                  value=choices[0])
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.filter, flag=wx.ALL, border=internalborder)

        label = wx.StaticText(self.panel, label=_('Root:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.root = wx.DirPickerCtrl(self.panel, size=(300, -1))
        self.root.TextCtrlGrowable = True
        self.root.PickerCtrlProportion = 0.1
        self.root.TextCtrlProportion = 0.1
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.root)

        label = wx.StaticText(self.panel, label=_('Query:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.query = wx.TextCtrl(self.panel, size=(300, -1))
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.query, flag=wx.ALL, border=5)

        vbox.Add(sizer, flag=wx.ALL, border=externalborder)
        vbox.Add(wx.StaticLine(self.panel), 0, wx.ALL|wx.EXPAND)

        ##################################
        # Other Options
        sizer = wx.FlexGridSizer(1, 2, hgap = 5)

        choices = ['PassThrough', 'Sandbox']
        label = wx.StaticText(self.panel, label=_('Cache:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.cache = wx.ComboBox(self.panel, choices=choices,
                                 style=wx.CHOICEDLG_STYLE,
                                 value=choices[0])
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.cache, flag=wx.ALL, border=internalborder)

        choices = ['Original', 'Flat', 'ISO9660', 'Documents', 'Date']
        label = wx.StaticText(self.panel, label=_('Organizer:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.organizer = wx.ComboBox(self.panel, choices=choices,
                                     style=wx.CHOICEDLG_STYLE,
                                     value=choices[0])
        sizer.Add(label, flag=wx.ALIGN_CENTER_VERTICAL)
        sizer.Add(self.organizer, flag=wx.ALL, border=internalborder)

        vbox.Add(sizer, flag=wx.ALL, border=externalborder)

        ##################################
        # Layout and other

        self._createtoolbar()

        hbox = wx.BoxSizer(wx.HORIZONTAL)
        hbox.Add(vbox, flag=wx.ALL, border=10)
        self.panel.Sizer = hbox
        hbox.Fit(self)
        self._setenabledall()

        self.new()
        self.Center()

    def _createtoolbar(self):
        self.ToolBar = wx.ToolBar(self,
                                  style=wx.TB_TEXT|wx.TB_HORIZONTAL|wx.TB_TOP)

        img = wx.ArtProvider.GetBitmap(wx.ART_NEW)
        self.ToolBar.AddLabelTool(1, _('New'), img)
        self.ToolBar.AddSeparator()
        img = wx.ArtProvider.GetBitmap(wx.ART_FILE_OPEN)
        self.ToolBar.AddLabelTool(2, _('Open'), img)
        img = wx.ArtProvider.GetBitmap(wx.ART_FILE_SAVE)
        self.ToolBar.AddLabelTool(3, _('Save'), img)
        self.ToolBar.AddSeparator()
        img = wx.ArtProvider.GetBitmap(wx.ART_NEW_DIR)
        self.ToolBar.AddLabelTool(4, _('Mount'), img)
        img = wx.ArtProvider.GetBitmap(wx.ART_DELETE)
        self.ToolBar.AddLabelTool(5, _('Umount'), img)
        self.ToolBar.Realize()

        self.Bind(wx.EVT_TOOL, self.new, id=_TB_NEW)
        self.Bind(wx.EVT_TOOL, self.open, id=_TB_OPEN)
        self.Bind(wx.EVT_TOOL, self.save, id=_TB_SAVE)
        self.Bind(wx.EVT_TOOL, self.mount, id=_TB_MOUNT)
        self.Bind(wx.EVT_TOOL, self.umount, id=_TB_UMOUNT)

    def new(self, event=None, mountpoint='', nonempty=False,
            noappledouble=False, filter=None, root='', query='',
            cache=None, organizer=None, filename=None):
        self.mountpoint.Path = mountpoint
        self.nonempty.Value = nonempty
        self.noappledouble.Value = noappledouble
        self.root.Path = root
        self.query.Value = query

        if filter:
            self.filter.Value = filter
        else:
            self.filter.Select(0)

        if cache:
            self.cache.Value = cache
        else:
            self.cache.Select(0)

        if organizer:
            self.organizer.Value = organizer
        else:
            self.organizer.Select(0)

        self.filename = filename
        self._settitle()

    def open(self, event):
        dialog = wx.FileDialog(self, wildcard=_DEJUMBLE_FILES)
        dialog.ShowModal()
        filename = dialog.Path
        if filename:
            with open(filename, 'rb') as file:
                result = pickle.load(file)
            self.new(filename=filename, **result)
            self._setenabledall()

    def save(self, event):
        if not self.filename:
            dialog = wx.FileDialog(self, wildcard=_DEJUMBLE_FILES,
                                   style=wx.FD_SAVE)
            dialog.ShowModal()
            filename = dialog.Path
            if not filename:
                return
            if filename.endswith('.%s.' % _EXTENSION):
                filename = filename[:-1]
            if filename.endswith('.'):
                filename = filename + _EXTENSION
            elif not filename.endswith('.%s' % _EXTENSION):
                filename = filename + '.%s' % _EXTENSION
            self.filename = filename
            self._settitle()

        with open(self.filename, 'wb') as file:
            values = {'mountpoint': self.mountpoint.Path,
                      'nonempty': self.nonempty.Value,
                      'noappledouble': self.noappledouble.Value,
                      'filter': self.filter.Value,
                      'root': self.root.Path,
                      'query': self.query.Value,
                      'organizer': self.organizer.Value}
            pickle.dump(values, file)

    def _settitle(self):
        if self.filename:
            self.Title = '%s - %s' % (_TITLE,
                                      self.filename.split(os.path.sep)[-1])
        else:
            self.Title = '%s - %s' % (_TITLE, 'Untitled')

    def mount(self, event):
        flags = []

        if self.nonempty.Value:
            flags.append(",nonempty")

        if self.noappledouble.Value:
            flags.append(",noappledouble")

        command = 'dejumble "%s" -o root="%s",query="%s",' \
                  'filter="%s",cache="%s",organizer="%s"%s' % \
                  (self.mountpoint.Path, self.root.Path, self.query.Value,
                   self.filter.Value, self.cache.Value, self.organizer.Value,
                   ''.join(flags))

        status, output = commands.getstatusoutput(command)

        if output:
            wx.MessageDialog(self, "Error mounting: %s" % output, 'Error',
                             wx.OK | wx.ICON_ERROR).ShowModal()

        self._setenabledall()

    def umount(self, event):
        command = 'umountdejumble "%s"' % self.mountpoint.Path

        status, output = commands.getstatusoutput(command)

        if output:
            wx.MessageDialog(None, 'Error umounting: %s' % output, 'Error',
                             wx.OK | wx.ICON_ERROR).ShowModal()

        self._setenabledall()

    def _setenabledall(self, event=None):
        enable = not os.path.isdir(os.path.join(self.mountpoint.Path,
                                                util.ORIGINAL_DIR))

        for child in self.Children:
            child.Enabled = enable

        self.ToolBar.EnableTool(_TB_MOUNT, enable)
        self.ToolBar.EnableTool(_TB_UMOUNT, not enable)


def main():
    application = DejumbleFSUI(0)
    application.MainLoop()


if __name__ == '__main__':
    main()
