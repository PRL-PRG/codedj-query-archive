#!/usr/bin/env python

import sys
try:
 	import pygtk
  	pygtk.require("2.0")
except:
  	pass
try:
    import gtk
    import gtk.glade
    import gobject    
except:
	sys.exit(1)
    
import bluetooth

class DeviceBrowser:
    
    def __init__(self):
        gladefile = "main.glade"  
        
        self.xml = gtk.glade.XML(gladefile) 
        self.window = self.xml.get_widget("device_browser")
        
        self.xml.signal_autoconnect(self)
        
        if (self.window):
            self.window.connect("destroy", gtk.main_quit)
       
    def timeout(self,pbar):
        self.pbar.pulse()
        return True
    
    def discover(self):
        nearby_devices = bluetooth.discover_devices(lookup_names = True)
        self.parent=self.xml.get_widget("pbar_parent")
        self.parent.remove(self.pbar)
        table = gtk.Table(len(nearby_devices)+1, 2, True)
        self.parent.add(table)
        i=0
        for address, name in nearby_devices:
            label = gtk.Label(address)
            table.attach(label, 0,1,i,i+1)
            label.show()
            label = gtk.Label(name)
            table.attach(label, 1,2,i,i+1)
            label.show()
            i+=1
        table.show()
        self.window.show()
      
        
    def main(self, type):
        self.window.show()
        self.pbar = self.xml.get_widget("pbar")
        timer = gobject.timeout_add (100, self.timeout, self)
        self.discover()        
    
class NoodleGTK:

    def __init__(self):
        
        gladefile = "main.glade"  
        
        self.xml = gtk.glade.XML(gladefile) 
        self.window = self.xml.get_widget("main")
        
        self.xml.signal_autoconnect(self)
        
        if (self.window):
        	self.window.connect("destroy", gtk.main_quit)
       
    
    def on_bt_pda_clicked(self, widget):
        self.window.hide()
        deviceBrowser=DeviceBrowser()
        deviceBrowser.main("pda")
        
   
    def on_bt_mobile_clicked(self,widget):
        deviceBrowser=DeviceBrowser()
        deviceBrowser.main("mobile")
        self.window.hide()
    
    def main(self):
        self.window.show()
        gtk.main()

if __name__ == "__main__":
    noodleGTK = NoodleGTK()
    noodleGTK.main()

    
	
