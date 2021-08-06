#!/usr/bin/python

import pygtk

pygtk.require('2.0')

import gtk, gtk.glade

class MainWindow:
    def __init__(self, app):
        self.w_tree = gtk.glade.XML('gui/gui.glade')
        self.w_tree.signal_autoconnect(self)
        
        self.app = app
        self.window = self.w_tree.get_widget('frmMain')
        self.tbTools = self.w_tree.get_widget('tbTools')
        self.hboxWorkSpace = self.w_tree.get_widget('hboxWorkSpace')
        self.frmAbout = self.w_tree.get_widget('frmAbout')
        self.frmAbout_cmdOkay = self.w_tree.get_widget('cmdOkay')
        
        self.w_tree.get_widget('mnuUseCaseDiagram').set_sensitive(False)
    
    def on_frmMain_destroy(self, frm):
        self.app.quit()
    
    def on_mnuQuit_activate(self, mnu):
        self.app.quit()
    
    def on_mnuViewTools_activate(self, mnu):
        # self.tbTools.set_child_visible(mnu.get_active())
        if mnu.get_active():
            self.hboxWorkSpace.pack_start(self.tbTools, expand=False, fill=False)
            self.hboxWorkSpace.reorder_child(self.tbTools, 0)
        else:
            self.hboxWorkSpace.remove(self.tbTools)
    
    def on_mnuAbout_activate(self, mnu):
        self.frmAbout.set_property('visible', True)
    
    def on_frmAbout_delete_event(self, win, event):
        win.set_property('visible', False)
        return True
    
    def on_cmdOkay_clicked(self, btn):
        self.frmAbout.set_property('visible', False)

class MainApp:
    def __init__(self):
        self.main_window = MainWindow(self)
    
    def main(self):
        gtk.main()
    
    def quit(self):
        gtk.main_quit()

MainApp().main()
