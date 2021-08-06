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

class video_converter(devede_executor.executor):
	
	def __init__(self,videofile,filename,filefolder,progresbar,proglabel,disctype,title,chapter,seconds=0):

		""" This class converts a video file to MPEG-1 or MPEG-2 format

		VIDEOFILE contains the parameters to convert the video
		FILENAME is the generic file name given by the user
		FILEFOLDER is the path where all the temporary and finall files will be created
		PROGRESBAR is the progress bar where the class will show the progress
		PROGLABEL is the label where the class will show what is it doing
		DISCTYPE can be dvd, vcd, svcd, cvd or divx
		TITLE and CHAPTER are the numbers used to identify the TITLE and CHAPTER number for this file
		SECONDS is the number of seconds we want to convert (for previews) """
		
		devede_executor.executor.__init__(self,filename,filefolder,progresbar)
		self.printout=False
		
		self.percent2=120
		if seconds==0:
			self.divide=float(videofile["olength"])
			if (videofile["cutting"]==1) or (videofile["cutting"]==2): # if we want only one half of the file
				self.divide/=2
		else:
			self.divide=float(seconds)

		if self.divide==0:
			self.divide=1

		self.error=""
		progresbar.set_fraction(0)
		progresbar.set_text("")
		
		if videofile["ismpeg"]: # if the file hasn't to be converted, we simply copy or link it
			self.pulse=True
			self.print_error=_("File copy failed\nMaybe you ran out of disk space?")
			if seconds==0:
				texto=_("Copying the file")+"\n"
			else:
				texto=_("Creating preview")+"\n"
			proglabel.set_text(texto+videofile["filename"])
			currentfile=self.create_filename(filefolder+filename,title,chapter,disctype=="divx")
		
			print "\ncurrentfile is: ", currentfile , "\n" 

			if sys.platform=='win32':
				if videofile["subtitles"]=="":
					#self.launch_program('ln.exe -s "'+videofile["path"]+'" "'+currentfile+'"',output=False)
					self.launch_program(["ln.exe", "-s", videofile["path"], currentfile],output=False)
				else:
					self.launch_shell('copy "'+videofile["path"] +'" "'+currentfile+'"',output=False)
			else:
				if videofile["subtitles"]=="":
					self.launch_shell('ln -s "'+videofile["path"]+'" "'+currentfile+'"',output=False)
				else:
					self.launch_shell('cp "'+videofile["path"]+'" "'+currentfile+'"',output=False)
			return

		self.pulse=False
		if seconds==0:
			texto=_("Converting files from title")
			proglabel.set_text(texto+" "+str(title)+"\n\n"+videofile["filename"])
		else:
			texto=_("Creating preview")
			proglabel.set_text(texto+"\n"+videofile["filename"])

		addbars=False
		framerate=int(videofile["ofps"])
		videorate=int(videofile["vrate"])
		audiorate=int(videofile["arate"])
		audio_final_rate=int(videofile["arateunc"])
		audiodelay=float(videofile["adelay"])
		final_framerate=float(videofile["fps"])
		aspect_ratio_original=videofile["oaspect"]
		aspect_ratio_final=videofile["aspect"]
		resx_final=videofile["width"]
		resy_final=videofile["height"]
		resx_original=videofile["owidth"]
		resy_original=videofile["oheight"]
	
		if aspect_ratio_original<1.3:
			aspect_ratio_original=float(videofile["owidth"])/(float(videofile["oheight"]))
		if aspect_ratio_original<1.33333333:
			aspect_ratio_original=1.33333333
	
		max_videorate=int(videorate*1.5)
	
		if disctype=="vcd":
			max_videorate=1152
		elif (disctype=="svcd") or (disctype=="cvd"):
			if (max_videorate+audiorate)>2550:
				max_videorate=2550-audiorate
		elif (disctype=="dvd") or (disctype=="divx"):
			if (max_videorate+audiorate)>8700:
				max_videorate=8700-audiorate
		else:
			print ("Error, disc format is not DVD, DIVX, VCD, SVCD or CVD. Contact with the author")
			print disctype
			sys.exit(1)
	
		if videofile["blackbars"]==0: # check if has to add black bars
			addbars=True
			if (resx_original%2)==1:
				resx_original+=1
			if (resy_original%2)==1:
				resy_original+=1
			resx_inter=resx_original
			resy_inter=int((resy_original*aspect_ratio_original)/aspect_ratio_final)
			if (resy_inter%2)==1:
				resy_inter+=1
			
			# due to a bug in MENCODER, we put bars only up and down, never left and right,
			# and we don't scale it if we have to add only 4 or less lines, because it is
			# too much work for so little profit
			
			if ((resy_inter<resy_original) or (resy_original+5>resy_inter)):
				addbars=False 
	
		if addbars==False:
			resx_inter=resx_original
			resy_inter=resy_original
		else:
			addx=0
			addy=int((resy_inter-resy_original)/2)

		command_var=[]
		if sys.platform!='win32':
			command_var=["mencoder"]
		else:
			command_var=["mencoder.exe"]

		if (disctype=="dvd") or (disctype=="divx"):
			audio_desired_final_rate=48000
		else:
			audio_desired_final_rate=44100

		if audio_final_rate!=audio_desired_final_rate:
			command_var.append("-srate")
			command_var.append(str(audio_desired_final_rate))
			command_var.append("-af")
			command_var.append("lavcresample="+str(audio_desired_final_rate))

		command_var.append("-oac")
		if (disctype=="divx"):
			command_var.append("mp3lame")
		else:
			command_var.append("lavc")
		
		command_var.append("-ovc")
		command_var.append("lavc")
		if (disctype!="divx"):
			command_var.append("-of")
			command_var.append("mpeg")
			command_var.append("-mpegopts")
			if disctype=="dvd":
				command_var.append("format=dvd:tsaf")
			elif disctype=="vcd":
				command_var.append("format=xvcd")
			elif (disctype=="svcd") or (disctype=="cvd"):
				command_var.append("format=xsvcd")
			else:
				print "Error, disc format incorrect. Talk with the creator."
				sys.exit(1)

		if seconds!=0:
			command_var.append("-endpos")
			command_var.append(str(seconds))
		else:
			if videofile["cutting"]==1: # first half only
				command_var.append("-endpos")
				command_var.append(str(videofile["olength"]/2))
			elif videofile["cutting"]==2: # second half only
				command_var.append("-ss")
				command_var.append(str((videofile["olength"]/2)-5)) # start 5 seconds before

		if audiodelay!=0.0:
			command_var.append("-delay")
			command_var.append(str(audiodelay))

		if final_framerate==30:
			if (framerate==24) and ((disctype=="dvd") or (disctype=="divx")):
				str_final_framerate="24000/1001"
			else:
				str_final_framerate="30000/1001"
			keyintv=18
		else:
			str_final_framerate=str(int(final_framerate))
			keyintv=15
	
		command_var.append("-ofps")
		command_var.append(str_final_framerate)

		if disctype=="divx":
			command_var.append("-ffourcc")
			command_var.append("DX50")

		lineatemp=""
	
		acoma=False;
	
		if videofile["deinterlace"]!="none":
			lineatemp+="pp="+videofile["deinterlace"]
			acoma=True

		if addbars and ((resx_inter!=resx_original) or (resy_inter!=resy_original)):
			if acoma:
				lineatemp+=","
			lineatemp+="expand="+str(resx_inter)+":"+str(resy_inter)+":"+str(addx)+":"+str(addy)
			acoma=True

		if (resx_inter!=resx_final) or (resy_inter!=resy_final):
			if acoma:
				lineatemp+=","
			lineatemp+="scale="+str(resx_final)+":"+str(resy_final)
			acoma=True
		
		if disctype!="divx":
			if acoma:
				lineatemp+=","
			lineatemp+="harddup"
			acoma=True

		if (lineatemp!=""):
			command_var.append("-vf")		
			command_var.append(lineatemp)

		command_var.append("-lavcopts")
	
		lavcopts="vcodec="
		if disctype=="vcd":
			lavcopts+="mpeg1video"
		elif disctype=="divx":
			lavcopts+="mpeg4"
		else:
			lavcopts+="mpeg2video"
	
		if videofile["trellis"]:
			lavcopts+=":trell"
	
		if videofile["mbd"]==0:
			lavcopts+=":mbd=0"	
		elif videofile["mbd"]==1:
			lavcopts+=":mbd=1"
		elif videofile["mbd"]==2:
			lavcopts+=":mbd=2"

		if disctype!="divx":
			lavcopts+=":vstrict=0:vrc_maxrate="+str(max_videorate)
			lavcopts+=":vrc_buf_size="
			if (disctype=="vcd"):
				lavcopts+="327:vrc_minrate=1152"
			elif (disctype=="svcd") or (disctype=="cvd"):
				lavcopts+="917:vrc_minrate=600"
			elif (disctype=="dvd"):
				lavcopts+="1835"
	
		lavcopts+=":vbitrate="+str(videorate)
	
		if disctype!="divx":
			lavcopts+=":keyint="+str(keyintv)+":acodec=mp2"
			lavcopts+=":abitrate="+str(audiorate)
	
		if aspect_ratio_final>1.4:
			lavcopts+=":aspect=16/9"
		else:
			lavcopts+=":aspect=4/3"

		command_var.append(lavcopts)
	
		if disctype=="divx":
			lameopts="abr:br="+str(audiorate)
			command_var.append("-lameopts")
			command_var.append(lameopts)
	
		currentfile=self.create_filename(filefolder+filename,title,chapter,disctype=="divx")

		command_var.append("-o")
		command_var.append(currentfile)
		command_var.append(videofile["path"])
	
		extra_params=videofile["params"] # take the extra params
		while (extra_params!=""):
			extra_params,new_param=get_new_param(extra_params)
			if new_param!="":
				command_var.append(new_param)

		self.print_error=_("Conversion failed.\nIt seems a bug of Mencoder.")
		self.error_not_done=True
		self.launch_program(command_var,read_chars=300)


	def end_process(self,eraser,erase_temporal_files):

		return


	def set_progress_bar(self):

		if self.pulse:
			self.bar.pulse()
			return True

		pos=self.cadena.find("Pos:")
		if pos==-1:
			return False # don't erase the string
		pos2=self.cadena.find("s",pos+3)
		if pos2==-1:
			return False
		valuetemp=float(self.cadena[pos+4:pos2])
		value=(100.0*valuetemp)/self.divide
		if (value!=self.percent2) or (self.percent2==120):
			if (value)>100.0:
				value=100.0
			self.bar.set_fraction(value/100.0)
			self.bar.set_text(str(int(value))+"%")
			self.percent2=value
			if self.error_not_done:
				self.error_not_done=False
				self.print_error=_("Conversion failed\nMaybe you ran out of disk space?")
		return True
