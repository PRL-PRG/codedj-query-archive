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
import os
import thread

class Config:
	
        def __init__(self):
                self.opensync_path = os.getenv("HOME") + "/.opensync/"
		self.opensync_path_example == "/usr/share/guadalinex_noodle/example/group1"

        def main(self, device):

                if not os.path.exists(self.opensync_path):
                        os.mkdir (self.opensync_path)
                if os.path.exists(self.opensync_path + "group*"):
			##FIXME:: Should add a new one, not remove everyother
                        backup = os.mkdir (self.opensync_path + "backup" + date.today().strftine("%Y%m%d"))
                        shutil.move (self.opensync_path + "group*" , backup)
		if not os.path.exists(self.opensync_path + "/group1"):
			##FIXME:: Example path
			##FIXME:: create groupX
			shutil.copytree(opensync_path_example, self.opensync_path + "group1")
			xml = ConfigXML()
			xml.main(self.opensync_path + "group1", device)
                return
	
class ConfigXML:
	##FIXME:: Por defecto, los ficheros estan pensados para Nokia
	##FIXME:: Se deberia poder elegir calendario... de Evolution
	def  __init__(self):

	def main(self,config_path, device):
		for address, name in device:
			self.__change(config_path + "/syncgroup.conf", groupname, name)
			self.__change(config_path + "/2/syncml-obex-client.conf", bluetooth_address, address)
		return 0

	def __change(self,file_conf,node,value):
		from xml.dom.minidom import parse
		from xml.dom.ext import PrettyPrint
		doc = parse(file_conf)
		for child in  doc.getElementsByTagName(node)[0].childNodes:
			child.data=value
		doc_conf=open(file_conf,"w")
		PrettyPrint(doc,doc_conf)
		doc_conf.close(

#class Manage:
#	def __init__(self):
#	def main(self):
		
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
    
    def apply_config(self,widget, data=None):
        config = Config()
        config.main(self.device)
        gtk.main_hide()
        return True
    
    def refresh(self, widget, data=None):
        self.discover()
        return True

    def discover(self):
        ##FIXME:: discover_devices blocker. No progress bar. Thread
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

    
	
