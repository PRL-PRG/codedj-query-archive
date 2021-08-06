#!/usr/bin/env python
# -*- coding: UTF-8 -*-

# Copyright 2006-2007 (C) Raster Software Vigo (Sergio Costas)
# Copyright 2006-2007 (C) Peter Gill - win32 parts

# This file is part of DeVeDe
#
# DeVeDe is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# DeVeDe is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.


#################################################################
# This block contains all the functions to convert a video file #
# into an MPEG2-PS DVD-compliant file                           #
#################################################################

import time
import select
import signal
import subprocess
import sys
import os
import re
import shutil
import glob
import posixpath
import gtk
import gtk.glade
import gobject
import cairo
if sys.platform=='win32':
	import win32api


from devede_other import *
from devede_gtk_helper import *
import devede_video_convert
import devede_subtitles
import devede_xml_menu
import devede_delete
import devede_dvd
import devede_bincue
import devede_executor


class create_all:
	
	def __init__(self,structure,global_vars):
		
		self.gladefile=global_vars["gladefile"]
		self.structure=structure
		self.global_vars=global_vars
		self.tree=gtk.glade.XML(self.gladefile,"wprogress",domain="devede")
		self.tree.signal_autoconnect(self)
		self.window=self.tree.get_widget("wprogress")
		self.partial=self.tree.get_widget("progresspartial")
		self.erase_temp=global_vars["erase_temporary_files"]
		
		total=0
		for element in structure:
			for element2 in element[1:]:
				if element2["subtitles"]=="":
					total+=1
				else:
					total+=2
		self.total_actions=total+global_vars["number_actions"]-1
		self.actions=global_vars["number_actions"]
		self.total=self.tree.get_widget("progress_total")
		self.label=self.tree.get_widget("lcreating")
		self.total.set_fraction(0)
		self.total.set_text("0/"+str(self.total_actions))
		self.partial.set_fraction(0)
		self.partial.set_text("0%")
		self.label.set_text("")
		self.start_time=time.time()
		self.disk_type=global_vars["disctocreate"]
		self.main_window=global_vars["main_window"]
		self.tiempo=time.time()


	def cancel_clicked(self,widget,temp=False):
		newtree=gtk.glade.XML(self.gladefile,"wcancel_job_dialog",domain="devede")
		window=newtree.get_widget("wcancel_job_dialog")
		window.show()
		value=window.run()
		window.hide()
		window.destroy()
		if value!=-5: # no
			return True
		self.runner.cancel()
		self.runner.wait_end()
		gobject.source_remove(self.timer)
		self.window.hide()
		self.window.destroy()
		newtree=gtk.glade.XML(self.gladefile,"waborted_dialog",domain="devede")
		window=newtree.get_widget("waborted_dialog")
		window.show()
		window.run()
		window.hide()
		window.destroy()
		self.main_window.show()
		return True


	def iso_changed(self,args,arbol):

		iso_name=arbol.get_widget("iso_filename")
		iso_folder=arbol.get_widget("final_directory")
	
		w=arbol.get_widget("button_folder_accept")
	
		mode=True
	
		if iso_name.get_text()=="":
			mode=False
		
		folder=iso_folder.get_filename()
		if folder==None:
			mode=False
		elif folder=="":
			mode=False
		
		w.set_sensitive(mode)


	def check_free_space(self,filefolder,structure,actions,erase_temporary_files):

		""" Returns TRUE if the free space in FILEFOLDER is insuficient to generate
		the disk STRUCTURE """
		# TODO Windows Stuff
		estado=''
		freespace=''
		if sys.platform!='win32':
			estado=os.statvfs(filefolder) # eg. f="C:\Documents and Settings\User name\Desktop"
			freespace=95*estado.f_bsize*estado.f_bavail/100000
		else:
			try:
				spc, bps, fc, tc = win32api.GetDiskFreeSpace(filefolder)
				freespace=fc * spc * bps

			except ImportError:
				pass
		print "Free space in "+str(filefolder)+": "+str(freespace)
		print "estatus ", estado, "\n"
	
		total=calcula_tamano_total(structure)

		print "Free: "+str(freespace)
		print "Needed: "+str(total)
	
		if (actions!=3):
			total*=actions # if we only create the MPEG files or the DVD structure...
		else:
			if erase_temporary_files: # or if we create the ISO image
				total*=2
			else:
				total*=3
		total*=1.1 # a safe margin of 10%

		if (freespace<total):
			return True,_("Insuficient free space. To create this disk\n%(total)d MBytes are needed, but only %(free)d MBytes are available.") % {'total':int(total/1000),'free':int(freespace/1000)}
		else:
			return False,""


	def preview(self,filefolder):
		
		newtree=gtk.glade.XML(self.gladefile,"wpreview_dialog",domain="devede")
		timev=newtree.get_widget("seconds_preview")
		timev.set_value(60)
		path=newtree.get_widget("temporary_files")
		path.set_filename(filefolder)
		w=newtree.get_widget("wpreview_dialog")
		w.show()
		ret=w.run()
		w.hide()
		self.filefolder=path.get_filename()
		if self.filefolder[-1]!=os.sep:
			self.filefolder+=os.sep
		self.seconds=timev.get_value()
		w.destroy()
		if ret!=-6:
			return self.filefolder

		self.runner=None
		self.current_action="add_subtitles"
		self.current_title=0
		self.current_chapter=0
		self.total_done=0.0
		self.filename="previewfile"
		
		try:
			fichero=open(self.filefolder+"write_check","w")
			fichero.write("Testing")
			fichero.close()
		except:
			self.show_error(_("Failed to write to the destination directory.\nCheck that you have privileges and free space there."))
			self.window.destroy()
			return self.filefolder
		
		try:
			os.remove(self.filefolder+"write_check")
		except:
			print "Failed to erase the write check file"
		
		self.eraser=devede_delete.delete_files(self.filename,self.filefolder)
		self.erase_temp=True
		self.timer=gobject.timeout_add(500,self.time_callback)
		self.window.show()
		return self.filefolder
		

	def create_disc(self):
		
		self.time=0
		
		# first, check for empty titles
		
		vacio=False
		for elemento in self.structure:
			if len(elemento)<2:
				vacio=True
				break
			
		if vacio:
			newtree=gtk.glade.XML(self.gladefile,"wempty_titles_dialog",domain="devede")
			w=newtree.get_widget("wempty_titles_dialog")
			w.show()
			value=w.run()
			w.hide()
			w.destroy()
			if value!=-6:
				return False

		# ask the folder and filename
		
		newtree=gtk.glade.XML(self.gladefile,"wfolder_dialog",domain="devede")
		wdir=newtree.get_widget("final_directory")
		wfile=newtree.get_widget("iso_filename")
		wfile.set_text("movie")
		wfile.connect("changed",self.iso_changed,newtree)
		self.iso_changed("",newtree)
		w=newtree.get_widget("wfolder_dialog")
		w.show()
		value=w.run()
		self.filename=wfile.get_text()
		self.filefolder=wdir.get_filename()
		if self.filefolder[-1]!=os.sep:
			self.filefolder+=os.sep
		w.hide()
		w.destroy()
		if value!=-6:
			return False
	
		self.eraser=devede_delete.delete_files(self.filename,self.filefolder)
		hasfree,msg=self.check_free_space(self.filefolder,self.structure,self.actions,self.erase_temp)
		if hasfree:
			self.window.hide()
			self.window.destroy()
			self.show_error(msg)
			self.main_window.show()
			return False
	
		# erase all conflicting files
		
		self.eraser.delete_all()
		
		# now, create the XML files (even with VCD, SVCD or CVD, to check if we have write permissions)

		xml_files=devede_xml_menu.xml_files()
		retorno=xml_files.create_files(self.filename,self.filefolder,self.structure,self.global_vars["do_menu"],self.global_vars["menu_PAL"],self.global_vars["menu_widescreen"],self.global_vars["path"],self.global_vars["menu_bg"],self.global_vars["fontname"],self.global_vars["path"])
		if retorno!=None:
			self.window.hide()
			self.window.destroy()
			self.show_error(retorno)
			self.main_window.show()
			return False
	
		xml_files.end_process(self.eraser,self.erase_temp)
		xml_files=None
		self.runner=None
		self.current_action="add_subtitles"
		self.current_title=0
		self.current_chapter=0
		self.seconds=0
		self.total_done=0.0
		self.timer=gobject.timeout_add(500,self.time_callback)
		self.window.show()
		return True


	def time_callback(self):

		""" This method launches all the conversion stages when needed, using the standard executor
		interface to manage all of them in an easy way """

		if self.runner!=None:
			retval=self.runner.refresh()
			if retval==0: # no error, still running
				return True
			else:
				self.total_done+=1.0
				self.total.set_fraction(self.total_done/(float(self.total_actions)))
				self.total.set_text(str(int(self.total_done))+"/"+str(self.total_actions))
				retval=self.runner.wait_end()
				if retval!=0:
					self.show_error(self.runner.print_error)
					self.main_window.show()
					return False
				else:
					self.runner.end_process(self.eraser,self.erase_temp)
				self.runner=None
				
		if (self.current_action=="add_subtitles"):
			self.current_action="convert_file"
			self.current_chapter+=1
			if (self.current_chapter>=len(self.structure[self.current_title])):
				self.current_chapter=1
				self.current_title+=1
			if (self.current_title<len(self.structure)):
				self.runner=devede_video_convert.video_converter(self.structure[self.current_title][self.current_chapter],self.filename,self.filefolder,self.partial,self.label,self.global_vars["disctocreate"],self.current_title+1,self.current_chapter,self.seconds)
				return True
			if self.seconds==0:
				self.current_action="create_disc"
			else:
				self.current_action="show_preview"
		
		if self.current_action=="convert_file":
			self.current_action="add_subtitles"
			if self.structure[self.current_title][self.current_chapter]["subtitles"]!="":
				self.runner=devede_subtitles.subtitles_adder(self.structure[self.current_title][self.current_chapter],self.filename,self.filefolder,self.partial,self.label,self.global_vars["disctocreate"],self.current_title+1,self.current_chapter)
				return True
		
		if self.current_action=="show_preview":
			self.window.hide()
			self.window.destroy()
			if sys.platform=='win32':
				mplay="mplayer.exe"
			else:
		 		mplay="mplayer"
			parameters=[mplay,"-sid","0x20",self.filefolder+"previewfile_01_01.mpg"]
			newtree=gtk.glade.XML(self.gladefile,"wpreviewagain_dialog",domain="devede")
			w=newtree.get_widget("wpreviewagain_dialog")
			while True:
				salida=devede_executor.executor("previewfile",self.filefolder,None)
				salida.launch_program(parameters,output=False)
				salida.wait_end()
				w.show()
				ret=w.run()
				w.hide()
				if ret!=-6:
					break
				while gtk.events_pending():
					gtk.main_iteration()
			w.destroy()
			os.remove(self.filefolder+"previewfile_01_01.mpg")
			return False
		
		if self.current_action=="create_disc":
			if self.actions==1: # only convert files
				self.show_final_time()
				return False
			if self.disk_type=="dvd":
				self.runner=devede_dvd.dvd_generator(self.filename,self.filefolder,self.partial,self.label)
			else:
				self.runner=devede_bincue.xvcd_generator(self.filename,self.filefolder,self.partial,self.label,self.structure,self.disk_type)
			self.current_action="create_iso"
			return True
		
		if self.current_action=="create_iso":
			if self.actions==2: # convert and do dvd structure/create BIN/CUE
				self.show_final_time()
				return False
			self.runner=devede_bincue.iso_generator(self.filename,self.filefolder,self.partial,self.label)
			self.current_action="ended"
			return True
			
		if self.current_action=="ended":
			self.show_final_time()
			return False
		return True


	def show_final_time(self):
		if (self.erase_temp):
			self.eraser.delete_xml()

		self.window.hide()
		self.window.destroy()
		newtree=gtk.glade.XML(self.gladefile,"wend_dialog",domain="devede")
		label=newtree.get_widget("elapsed")
		tiempo2=return_time(time.time()-self.tiempo,True)
		label.set_text(tiempo2)
		window=newtree.get_widget("wend_dialog")
		window.show()
		window.run()
		window.hide()
		window.destroy()
		window = None
		newtree = None
		self.main_window.show()


	def show_error(self,message):
		
		self.window.hide()
		self.window.destroy()
		newtree=gtk.glade.XML(self.gladefile,"werror_dialog",domain="devede")
		label=newtree.get_widget("label_error_dialog")
		label.set_text(message)
		window=newtree.get_widget("werror_dialog")
		window.show()
		window.run()
		window.hide()
		window.destroy()
		return

