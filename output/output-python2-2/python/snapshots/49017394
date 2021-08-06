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


import cairo
import time
import sys
import os
import devede_executor
import devede_other

class xml_files(devede_executor.executor):
	
	""" This class creates the XML files and the menues """
	
	def create_files(self,filename,filefolder,structure,do_menu,menu_PAL,menu_widescreen,menu_path,menu_bg,font_name,path,extcr=None):
		
		devede_executor.executor.__init__(self,filename,filefolder,None)
		
		if self.create_xml(structure,do_menu,menu_PAL,menu_widescreen):
			return _("Failed to write to the destination directory.\nCheck that you have privileges and free space there.")
	
		if do_menu:
			if self.create_menu_stream(structure,menu_PAL,menu_widescreen,menu_path):
				return _("Failed to write to the destination directory.\nCheck that you have privileges and free space there.")
			if None==self.create_menu_bg(structure, menu_bg, menu_PAL, font_name, extcr):
				return _("Can't find the menu background.\nCheck the menu options.")
			if self.create_menu_mpg(menu_PAL,path):
				return _("Menu generation failed.")
			if self.menu_mplex_buttons():
				return _("Can't add the buttons to the menus.\nIt seems a bug of SPUMUX.")
			
		return None
		
		
	def create_xml(self,structure,do_menu,menu_PAL,menu_widescreen):

		""" Creates the XML file for DVDAuthor """

		try:
			fichero=open(self.filefolder+self.filename+".xml","w")
			fichero.write('<dvdauthor dest="'+self.filefolder+self.filename+'">\n')
			fichero.write("<vmgm>\n")
			fichero.write("</vmgm>\n")
			titles=0
			total_t=len(structure)
			fichero.write("<titleset>\n")
		
			if do_menu:
				fichero.write("<menus>\n")
				fichero.write('<video format="')
				if menu_PAL:
					fichero.write("pal")
				else:
					fichero.write("ntsc")
				fichero.write('" aspect="')
				if menu_widescreen:
					fichero.write("16:9")
				else:
					fichero.write("4:3")
				fichero.write('"> </video>\n')
				fichero.write('<pgc>\n')
				fichero.write('<vob file="'+self.filefolder+self.filename+'_menu2.mpg"></vob>\n')
				contador=0
				for elemento in structure:
					fichero.write('<button name="boton'+str(contador)+'">jump title '+str(contador+1)+';</button>\n')
					contador+=1
				fichero.write('</pgc>\n</menus>\n')
		
			fichero.write("<titles>\n")
			for elemento in structure:
				action=elemento[0]["jumpto"]
				titles+=1
				fichero.write("<pgc>\n")
				if len(elemento)>1:
					files=0
					for elemento2 in elemento[1:]:
						files+=1
						currentfile=self.create_filename(self.filefolder+self.filename,titles,files,False)
						fichero.write('<vob file="'+currentfile+'" ')
						fichero.write('chapters="0')
						if (elemento2["olength"]>5):
							if (elemento2["lchapters"]!=0): # add chapters
								toadd=int(elemento2["lchapters"])
								seconds=toadd*60
								while seconds<(elemento2["olength"]-4):
									thetime=devede_other.return_time(seconds,False)
									fichero.write(","+thetime)
									seconds+=(toadd*60)
							fichero.write(','+devede_other.return_time((elemento2["olength"]-2),False))
						fichero.write('" />\n')
					if action=="menu":
						if do_menu:
							fichero.write("<post>call menu;</post>\n")
					elif action=="first":
						fichero.write("<post>jump title 1;</post>\n")
					elif action=="prev":
						if titles==1:
							prev_t=total_t
						else:
							prev_t=titles-1
						fichero.write("<post>jump title "+str(prev_t)+";</post>\n")
					elif action=="loop":
						fichero.write("<post>jump title "+str(titles)+";</post>\n")
					elif action=="next":
						if titles==total_t:
							next_t=1
						else:
							next_t=titles+1
						fichero.write("<post>jump title "+str(next_t)+";</post>\n")
					elif action=="last":
						fichero.write("<post>jump title "+str(total_t)+";</post>\n")
					fichero.write("</pgc>\n")
			fichero.write("</titles>\n</titleset>\n")
			fichero.write("</dvdauthor>")
			fichero.close()
			return False
		except IOError:
			return True

		
	def create_menu_stream(self,structure,menu_PAL,menu_widescreen,menu_path):

		""" Creates the menu XML file """

		cantidad=len(structure)

		if menu_PAL:
			formato="pal"
		else:
			formato="ntsc"

		if menu_widescreen:
			menu_wide="_wide"
		else:
			menu_wide=""

		try:
		
			fichero=open(self.filefolder+self.filename+"_menu.xml","w")
			fichero.write('<subpictures>\n<stream>\n<spu force="yes" start="00:00:00.00" transparent="ffffff"')
			fichero.write(' highlight="'+menu_path+formato+menu_wide+'_active.png" >\n')
			if menu_PAL:
				coord_y=[(53,90),(92,129),(130,167),(168,205),(207,244),(245,282),(283,320),(322,359),(360,397),(398,435),(437,474),(475,512)]
			else:
				coord_y=[(44,75),(76,107),(108,139),(140,171),(172,203),(204,235),(236,267),(268,299),(300,331),(332,363),(364,395),(396,427)]
			for contador in range(cantidad):
				fichero.write('<button name="boton'+str(contador))
				fichero.write('" x0="0" y0="'+str(coord_y[contador][0])+'" x1="719" y1="'+str(1+coord_y[contador][1])+'"')
				fichero.write(' up="boton')
				if contador==0:
					fichero.write(str(cantidad-1))
				else:
					fichero.write(str(contador-1))
				fichero.write('" down="boton')
			
				if contador==(cantidad-1):
					fichero.write("0")
				else:
					fichero.write(str(contador+1))
				fichero.write('" > </button>\n')
			fichero.write("</spu>\n</stream>\n</subpictures>\n")
			fichero.close()
			
			return False
		except IOError:
			return True
		
		
	def menu_set_text(self,cr,y,texto,widescreen,myfontname="Sans",myfontstyle=cairo.FONT_WEIGHT_BOLD,myfontslant=cairo.FONT_SLANT_NORMAL,myfontsize=12):

		if widescreen:
			x=0
		else:
			x=0.125

		radius=0.0375
		border=0.0048
		linea=0.0024
		fontsize2=myfontsize*0.00315

		cr.set_line_width(linea)
		cr.select_font_face(myfontname,myfontslant,myfontstyle)

		# I created the button image for this size, so I must respect it :(
		#xb,yb,width,height,cx,cy=cr.text_extents("Título 1")
		height=0.0391604010025
	
		cr.set_font_size(fontsize2)
		xb,y2,width,h2,cx,cy2=cr.text_extents(texto)

		cr.set_source_rgba(0,0,0,.75)
		cr.move_to(x,y-border)
		cr.line_to(1-x,y-border)
		cr.curve_to(1-x+radius,y-border,1-x+radius,y+height+border,1-x,y+height+border)
		cr.line_to(x,y+height+border)
		cr.curve_to(x-radius,y+height+border,x-radius,y-border,x,y-border)
		cr.fill()
	
		cr.set_source_rgb(1,1,1)
		cr.move_to(.5-width/2-xb,y-y2+(height-h2)/2)
		cr.show_text(texto)


	def create_menu_bg(self,structure,menu_bg,menu_PAL,fontname,extcr=None):

		wide=False
	
		try:
			sf_base=cairo.ImageSurface.create_from_png(menu_bg)
		except:
			return None

		if menu_PAL:
			y=576.0
		else:
			y=480.0

		sf=cairo.ImageSurface(cairo.FORMAT_ARGB32,720,int(y))
	
		wbase=float(sf_base.get_width())
		hbase=float(sf_base.get_height())
		cr=cairo.Context(sf)
		cr.identity_matrix()
		cr.scale(720.0/wbase,y/hbase)
		cr.set_source_surface(sf_base)
		cr.paint()
	
		cr.identity_matrix()
	
		if wide:
			cr.scale(0.75*sf.get_width(),1.33*sf.get_height()) # picture gets from 0 to 1 in X and from 0 to 0.75 in Y
			cr.translate(0.166666666,0)		
		else:
			cr.scale(sf.get_width(),1.33*sf.get_height()) # picture gets from 0 to 1 in X and from 0 to 0.75 in Y

		pos_y=0.075
		font_elements=[]
		font_temp=fontname
		while True:
			pos=font_temp.find(" ")
			if pos==-1:
				font_elements.append(font_temp) # add the last element
				break
			font_elements.append(font_temp[:pos])
			longitud=len(font_temp)
			if pos+1==longitud: # a missed blank space at the end
				break
			font_temp=font_temp[pos+1:]
	
		if (len(font_elements))<2:
			fontname="Sans"
			fontstyle=cairo.FONT_WEIGHT_NORMAL
			fontslant=cairo.FONT_SLANT_NORMAL
			fontsize=12
		else:
			fontname=""
			fontstyle=cairo.FONT_WEIGHT_NORMAL
			fontslant=cairo.FONT_SLANT_NORMAL
			for counter in range(len(font_elements)-1):
				if font_elements[counter]=="Bold":
					fontstyle=cairo.FONT_WEIGHT_BOLD
				elif font_elements[counter]=="Italic":
					fontslant=cairo.FONT_SLANT_ITALIC
				else:
					fontname+=" "+font_elements[counter]
			if fontname!="":
				fontname=fontname[1:]
			else:
				fontname="Sans"
	
		try:
			fontsize=float(font_elements[-1])
		except:
			fontsize=12
	
		for entrada in structure:
			self.menu_set_text(cr,pos_y,entrada[0]["nombre"],wide,fontname,fontstyle,fontslant,fontsize)
			pos_y+=0.05

		if extcr==None:
			sf.write_to_png(self.filefolder+self.filename+"_menu_bg.png")
		
		return sf
	
	
	def create_menu_mpg(self,menu_PAL,path):
	
		print "Creating menus"
	
		menu_widescreen=False
		
		command_var=[]
		if sys.platform=='win32':
			command_var=["mencoder.exe"]
		else:
			command_var=["mencoder"]
	
		currentfile=self.filefolder+self.filename+"_menu.mpg"
	
		command_var.append("-srate")
		command_var.append("48000")
		command_var.append("-af")
		command_var.append("lavcresample=48000")
		command_var.append("-oac")
		command_var.append("lavc")
		command_var.append("-ovc")
		command_var.append("lavc")
		command_var.append("-of")
		command_var.append("mpeg")
		command_var.append("-mpegopts")
		command_var.append("format=dvd:tsaf")
		command_var.append("-ofps")
		audio=path+"silence.wav"
		if menu_PAL:
			command_var.append("25")
			key="15"
		else:
			command_var.append("30000/1001")
			key="18"

		if menu_widescreen:
			wide="16/9"
		else:
			wide="4/3"
		command_var.append("-vf")
		if menu_PAL:
			command_var.append("scale=720:576,harddup")
		else:
			command_var.append("scale=720:480,harddup")
		command_var.append("-lavcopts")
		command_var.append("vcodec=mpeg2video:trell:mbd=2:vstrict=0:vrc_maxrate=7501:vrc_buf_size=1835:vbitrate=5001:keyint="+key+":acodec=mp2:abitrate=224:aspect="+wide)
		command_var.append("-o")
		command_var.append(currentfile)
		command_var.append("-audiofile")
		command_var.append(audio)
		command_var.append("-mf")
		command_var.append("type=png:fps=1")
		origDir=os.getcwd()
		if sys.platform=="win32":
			temp=os.path.split(self.filefolder+self.filename+"_menu_bg.png")
			picDir=temp[0]
			picName=temp[1]
			command_var.append("mf://"+picName)
			os.chdir(picDir)
		else:
			command_var.append("mf://"+self.filefolder+self.filename+"_menu_bg.png")
		print "Lanzo "+str(command_var)

		self.launch_program(command_var)
		
		while(0==self.refresh()):
			time.sleep(.5)
		if 0!=self.wait_end():
			return True
	
		if sys.platform=="win32":
			os.chdir(origDir)
		return False

	
	def menu_mplex_buttons(self):
	
		if sys.platform=='win32':
			comando="spumux.exe"
		else:
			comando="spumux"
			
		comando+=' "' +self.filefolder+self.filename+'_menu.xml"'

		print "Launch: "+comando
		self.launch_shell(comando,stdinout=[self.filefolder+self.filename+"_menu.mpg",self.filefolder+self.filename+"_menu2.mpg"])

		while(0==self.refresh()):
			time.sleep(.5)
		if 0!=self.wait_end():
			return True
	
		return False


	def end_process(self,eraser,erase_temporary_files):
		
		if erase_temporary_files:
			eraser.delete_menu_temp()