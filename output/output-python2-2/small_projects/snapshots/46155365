#!/usr/bin/env python

import findrox; findrox.version(2,0,0)
import rox, os
from rox import applet, g, app_options, basedir
from rox.Menu import Menu, set_save_name
from rox.options import Option


APP_NAME = "Wifi"
APP_DIR = rox.app_dir
APP_DOMAIN = 'm4rs.org'

from rox import choices
choices.migrate(APP_NAME, APP_DOMAIN)
rox.setup_app_options(APP_NAME, site=APP_DOMAIN)
set_save_name(APP_NAME, site=APP_DOMAIN)




menu=Menu('main',[(_('/Options'),'show_options', '<StockItem>', '', g.STOCK_PREFERENCES),
				  (_('/Refresh'), 'update', '<StockItem>', '', g.STOCK_REFRESH),
				  (_('/Quit'), 'quit', '<StockItem>', '', g.STOCK_QUIT) ])

REFRESH   = Option('REFRESH', '2')
THEME	 = Option('THEME', 'Simple')
INTERFACE = Option('INTERFACE', 'eth1')
rox.app_options.notify()

wirelessStruct = ('interface',
				  'status',
				  'qualityLink',
				  'qualityLevel',
				  'qualityNoise',
				  )
				  
class Wifi(applet.Applet):
	def __init__(self, arg):
		applet.Applet.__init__(self, arg)
		self.vertical = self.get_panel_orientation() in ('Left', 'Right')
		if self.vertical:
			self.set_size_request(8, -1)
		else:
			self.set_size_request(-1, 8)
		self.size	  = 0
		self.theme	 = THEME.value
		self.interface = INTERFACE.value
		self.refresh   = REFRESH.int_value
		
		self.timeout = g.timeout_add(int(self.refresh)*1000, self.update)
		def destroyed(self):
			g.timeout_remove(self.timeout)

		self.connect('button-press-event', self.button_press)
		self.lImgs = []
		self.getImg()
		self.hbox  = g.HBox()
		for hImg in self.lImgs:
			self.hbox.add(hImg['image'])
		
		self.tooltip = g.Tooltips()
		self.hbox.set_border_width(0)
		self.add(self.hbox)
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('size-allocate', self.event_callback)
		rox.app_options.add_notify(self.options_changed)
		self.show_all()
		self.update()

	def getImg(self):
		themeDir = os.path.join(APP_DIR, 'themes', self.theme)
		for hImg in self.lImgs:
			hImg['image'].destroy()
		self.lImgs = []
		try:
			f = open(os.path.join(themeDir, 'theme.desc'))
		except IOError:
			rox.croak('No theme.desc !')
		for l in f.readlines():
			#print l.split()
			(low, high, filename) = l.split()
			imgDict = {}
			imgDict['low'] = low
			imgDict['high'] = high
			imgDict['fileName'] = os.path.join(themeDir, filename)
			imgDict['pixbuf'] = g.gdk.pixbuf_new_from_file(os.path.join(themeDir, filename))
			imgDict['image'] = g.Image()
			self.lImgs.append(imgDict)
		f.close()
		
	
	def hashLine(self, line):
		lLine = line.split()
		h = {}
		for k in wirelessStruct:
			v = lLine.pop(0)
			v = v.rstrip('. :')
			h[k] = v
		return h

	def update(self):
		try:
			f = open('/proc/net/wireless')
			#TODO: /proc is deprecated, use sysfs instead?
		except IOError:
			rox.croak(_("Wireless not available on this system."))
		lines = f.readlines()
		f.close()
		try:
			interfaces = {}
			for l in lines[2:]:
				h = self.hashLine(l)
				interfaces[h['interface']] = h
			h = interfaces[self.interface]
			quality = h['qualityLink']
			level   = str(int(h['qualityLevel'])-256)
			noise   = str(int(h['qualityNoise'])-256)
			self.tooltip.set_tip(self, "Quality: "+quality+"\nLevel:  "+level+"dBm\nNoise: "+noise+'dBm')
			q = int(quality)
			for hImg in self.lImgs:
				if (q >= int(hImg['low']) and q <= int(hImg['high'])):
					#print hImg['fileName']
					hImg['image'].show()
				else:
					hImg['image'].hide()
		except KeyError:
			print 'wrong wireless interface'
			for hImg in self.lImgs:
				if (int(hImg['low']) < 0):
					hImg['image'].show()
				else:
					hImg['image'].hide()
		return g.TRUE
	
	def event_callback(self, widget, rectangle):
		side = self.get_panel_orientation()
		if self.vertical:
			size = rectangle[2] -2
		else:
			size = rectangle[3] -2
		if size != self.size:
			self.resize_image(size)

	def resize_image(self, size):
		for hImg in self.lImgs:
			scaled_pixbuf = hImg['pixbuf'].scale_simple(size, size, g.gdk.INTERP_HYPER)
			hImg['image'].set_from_pixbuf(scaled_pixbuf)
		self.size = size
		
	def get_panel_orientation(self):
		"Return the panel orientation ('Top', 'Bottom', 'Left', 'Right')"
		try:
			pos = self.socket.property_get('_ROX_PANEL_MENU_POS', 'STRING', False)
			if pos:
				return pos[2].split(',')[0]
		except:
			return 'Bottom'

	def quit(self):
		self.destroy()
		
	def show_options(self):
		rox.edit_options()
		
	def quit(self):
		self.destroy()
		
	def show_options(self):
		rox.edit_options()
		
	def options_changed(self):
		if THEME.has_changed:
			self.theme   = THEME.value
			self.getImg()
			self.resize_image(self.size)
			for hImg in self.lImgs:
				print hImg['fileName']
				self.hbox.add(hImg['image'])
		if REFRESH.has_changed:
			self.refresh = REFRESH.int_value
			g.timeout_remove(self.timeout)
			self.timeout = g.timeout_add(int(self.refresh)*1000, self.update)
		self.interface = str(INTERFACE.value)

	def button_press(self, window, event):
		if event.button == 3:
			menu.popup(self, event, self.position_menu)
		if event.button == 1:
			self.update()

