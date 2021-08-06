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
import re
import shutil

class Config:
        def __init__(self):
                self.opensync_path = os.getenv("HOME") + ".opensync"
                self.username = os.getlogin()

                if not os.path.exits(self.opensync_path):
                        os.mkdir (opensync_path)
                if os.path.exits(self.opensynce_path + "group*"):
                        backup = os.mkdir (opensync_path + "backup" + date.today().strftine("%Y%m%d"))
                        shutil.move (opensync_path + "group*" , backup)
                
        def main(device):
                if not os.path.exists(opensync_path + "/group1"):
                        bluetooth_regex=re.compile("__BLUETOOTH__")
                        readlines=open(example_path + "/" ,'r').readlines()
                        for currentline in readlines:
                                if cregex.search(currentline):
                                        currentline = re.sub("__HOME__",home,currentline)
                        write_file = open(multisync_path + "/1/localsettings",'w')
                        for line in readlines:
                                write_file.write(line)
                        write_file.close()
                                                                                                                  
                return

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
    
    def active_rb(self, widget, data=None):
        self.active = self.nearby_devices[data]
        print self.active
    
    def apply_config(self,widget, data=None):
        config = Config()
        config.main()
        gtk.main_hide()
        return True
    
    def refresh(self, widget, data=None):
        self.discover()
        return True

    def discover(self):
        try:    
                nearby_devices = bluetooth.discover_devices(lookup_names = True)
                print nearby_devices
        except:
                label = gtk.Label("No ha dispositivo blueetooth")
                self.parent=self.xml.get_widget("pbar_parent")
                self.parent.remove(self.pbar)
                self.parent.add(label)
                label.show()
                return 0
        if len(nearby_devices) != 0:
            self.parent=self.xml.get_widget("pbar_parent")
            self.parent.remove(self.pbar)
            
            self.table_main = gtk.Table(2,1,True)
            self.parent.add(self.table_main)
            button = gtk.Button(stock=gtk.STOCK_APPLY)
            button.connect_object("clicked", self.apply_config, None)
            self.table_main.attach(button, 0,1,1,2)
            button.show()
            
            table = gtk.Table(len(nearby_devices), 1, True)
            self.table_main.attach(table,0,1,0,1)
            i=0
            self.active=nearby_devices[i]
            for address, name in nearby_devices:
                button = gtk.RadioButton(None, name)
                button.connect("toggled", self.active_rb, name)
                table.attach(button, 0,1,i,i+1)
                i+=1
                button.show()
            table.show()
            self.table_main.show()
        else:
            self.parent=self.xml.get_widget("pbar_parent")
            self.parent.remove(self.pbar)
            self.table_main = gtk.Table(2,1,True)
            self.parent.add(self.table_main)
            button = gtk.Button(stock=gtk.STOCK_REFRESH)
            button.connect_object("clicked", self.refresh, None)
            self.table_main.attach(button, 0,1,1,2)
            button.show()
            label = gtk.Label("No se han encontrado dispositivos blueetooth")
            self.table_main.attach(label,0,1,0,1)
            label.show()
            self.table_main.show()
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

    
	
