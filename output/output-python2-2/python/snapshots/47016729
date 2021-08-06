#!/usr/bin/env python

import gettext
import commands
import sys
import os.path

import wx

from .. import util

gettext.install('dejumblefs')


class DejumbleFSUI(wx.App):

    def OnInit(self):
        self.main = MainWindow()
        self.main.Show()
        self.SetTopWindow(self.main)
        return True


class MainWindow(wx.Frame):
 
    def __init__(self):
        wx.Frame.__init__(self, None, title=_('DejumbleFS mounter'),
                          style=wx.CAPTION|wx.CLOSE_BOX)

        self._createtoolbar()

        externalborder = 10
        internalborder = 3

        self.panel = wx.Panel(self)

        vbox = wx.BoxSizer(wx.VERTICAL)

        ##################################
        # Mountpoint Options
        sizer = wx.FlexGridSizer(1, 2, hgap = 5)

        label = wx.StaticText(self.panel, label=_('Mount point:'),
                              size=(100, -1), style=wx.ALIGN_RIGHT)
        self.mountpoint = wx.DirPickerCtrl(self.panel, size=(300, -1))
        self.mountpoint.Refresh()
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
        
        hbox = wx.BoxSizer(wx.HORIZONTAL)
        hbox.Add(vbox, flag=wx.ALL, border=10)
        self.panel.SetSizer(hbox)
        hbox.Fit(self)

    def _createtoolbar(self):
        self.ToolBar = wx.ToolBar(self, style=wx.TB_TEXT|wx.TB_HORIZONTAL|wx.TB_TOP)

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

        self.Bind(wx.EVT_TOOL, self.new, id=1)
        self.Bind(wx.EVT_TOOL, self.open, id=2)
        self.Bind(wx.EVT_TOOL, self.save, id=3)
        self.Bind(wx.EVT_TOOL, self.mount, id=4)
        self.Bind(wx.EVT_TOOL, self.umount, id=5)

    def new(self, event=None):
        self.mountpoint.SetPath('')
        self.nonempty.Value = False
        self.noappledouble.Value = False
        self.filter.Select(0)
        self.root.SetPath('')
        self.query.Value = ''
        self.cache.Select(0)
        self.organizer.Select(0)
        pass

    def open(self, event):
        pass

    def save(self, event):
        pass

    def mount(self, event):
        flags = []

        if self.nonempty.Value:
            flags.append(",nonempty")

        if self.noappledouble.Value:
            flags.append(",noappledouble")

        command = 'dejumble "%s" -o root="%s",query="%s",filter="%s",cache="%s",organizer="%s"%s' %\
            (self.mountpoint.Path, self.root.Path, self.query.Value,
             self.filter.Value, self.cache.Value, self.organizer.Value,
             ''.join(flags));

        status, output = commands.getstatusoutput(command)

        if output:
            wx.MessageDialog(self, "Error mounting: %s" % output, 'Error', wx.OK | 
                             wx.ICON_ERROR).ShowModal()

        self._setenabledall()

    def umount(self, event):
        command = 'umountdejumble "%s"' % self.mountpoint.Path

        status, output = commands.getstatusoutput(command)

        if output:
            wx.MessageDialog(None, 'Error umounting: %s' % output, 'Error', wx.OK | 
                             wx.ICON_ERROR).ShowModal()

        self._setenabledall()

    def _setenabledall(self):
        enable = not os.path.isdir(os.path.join(self.mountpoint.Path, util.ORIGINAL_DIR))

        for attr in ('mountpoint', 'root', 'query', 'filter', 'cache', 'organizer',
                     'nonempty', 'noappledouble'):
            getattr(self, attr).Enabled = enable

        self.ToolBar.EnableTool(4, enable)
        self.ToolBar.EnableTool(5, not enable)


def main():
    application = DejumbleFSUI(0)
    application.MainLoop()


if __name__ == '__main__':
    sys.exit(main())
