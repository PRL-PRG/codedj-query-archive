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


import signal
import os
import sys
import re
import shutil

import devede_executor

class subtitles_adder(devede_executor.executor):
	
	def __init__(self,videofile,filename,filefolder,progresbar,proglabel,disctype,title,chapter):

		""" This class adds the subtitles to an already converted file

		VIDEOFILE contains the parameters to convert the video
		FILENAME is the generic file name given by the user
		FILEFOLDER is the path where all the temporary and finall files will be created
		PROGRESBAR is the progress bar where the class will show the progress
		PROGLABEL is the label where the class will show what is it doing
		DISCTYPE can be dvd, vcd, svcd, cvd or divx
		TITLE and CHAPTER are the numbers used to identify the TITLE and CHAPTER number for this file
		SECONDS is the number of seconds we want to convert (for previews) """

		devede_executor.executor.__init__(self,filename,filefolder,progresbar)
		progresbar.pulse()
		proglabel.set_text(_("Adding subtitles to")+"\n"+videofile["filename"])
		self.currentfile=self.create_filename(filefolder+filename,title,chapter,disctype=="divx")

		# generate the XML file

		self.error=""

		try:
			fichero=open(filefolder+filename+"_sub.xml","w")
			fichero.write('<subpictures>\n\t<stream>')
			fichero.write('\n\t\t<textsub filename="'+videofile["subtitles"]+'"')
			fichero.write('\n\t\tfont="devedesans.ttf"')
			if ((videofile["sub_codepage"]!="") and (videofile["sub_codepage"]!="ASCII")):
				fichero.write('\n\t\tcharacterset="'+videofile["sub_codepage"]+'"')
			fichero.write('\n\t\thorizontal-alignment="center"')

			if (videofile["fps"]==25):
				ancho=716
				alto=572
				tamanofont=28
			else:
				ancho=716
				alto=476
				tamanofont=28

			margin_hor=int((58*ancho)/720)
			margin_vert=int((28*alto)/576)
			bottom_margin=margin_vert

			fichero.write('\n\t\tmovie-width="'+str(ancho-4)+'"')
			fichero.write('\n\t\tmovie-height="'+str(alto-4)+'"')
			fichero.write('\n\t\tleft-margin="'+str(margin_hor)+'"')
			fichero.write('\n\t\tright-margin="'+str(margin_hor)+'"')

			if videofile["subtitles_up"]:
				tamanofont-=1
				bottom_margin=4+(alto/8) # put it in the border of 16:9 aspect ratio

			fichero.write('\n\t\tbottom-margin="'+str(bottom_margin)+'"')
			fichero.write('\n\t\ttop-margin="'+str(margin_vert)+'"')

			fichero.write('\n\t\tfontsize="'+str(tamanofont)+'.0"')

			if (videofile["fps"]==30):
				if (videofile["ofps"]==24) and ((disctype=="dvd") or (disctype=="divx")):
					fps_out="24000/1001"
				else:
					fps_out="30000/1001"
			else:
				fps_out="25"

			fichero.write('\n\t\tmovie-fps="'+str(fps_out)+'"')
			fichero.write('\n\t\tsubtitle-fps="'+str(fps_out)+'"')
			fichero.write('\n\t\tvertical-alignment="bottom" />')
			fichero.write("\n\t</stream>\n</subpictures>")
			fichero.close()
		except IOError:
			self.print_error=_("Failed to write to the destination directory.\nCheck that you have privileges and free space there.")
			return True
		
		comando=""
		if sys.platform=='win32':
			comando="spumux.exe -m "
		else:
			comando="spumux -m "
		if disctype=="vcd":
			comando+="svcd"
		else:
			comando+=disctype
		
		comando+=' "'+filefolder+filename+'_sub.xml"'
		
		self.print_error=_("Conversion failed.\nIt seems a bug of SPUMUX.")
		self.launch_shell(comando,output=True,stdinout=[self.currentfile,self.currentfile+".sub"])


	def end_process(self,eraser,erase_temporal_files):

		shutil.move(self.currentfile+".sub", self.currentfile)
		if erase_temporal_files:
			eraser.delete_sub_xml()


	def set_progress_bar(self):

		self.bar.pulse()		
		position=self.cadena.find("STAT: ")
		if (position!=-1):
			position2=self.cadena.find(".",position+6)
			if position2!=-1:
				self.bar.set_text(self.cadena[position:position2])
		return True
