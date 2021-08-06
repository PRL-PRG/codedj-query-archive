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


###########################################################################
# This block contains generic help functions that are used in the program #
###########################################################################

import os
import subprocess
import stat
import sys
if sys.platform == 'win32':
	import _winreg
	import win32process


def separa_dnd(datos):

	""" Takes the list of files dragged into the window and returns them in a list """

	lista=[]
	item=""
	tempo=""
	modo=0
	for elemento in datos:
		if (elemento=="\n") and (item!=""):
			if item[:8]=="file:///":
				lista.append(item[7:])
				item=""
				continue
		if ord(elemento)<32:
			continue
		if modo==0:
			if elemento=="%":
				modo=1
			else:
				item+=elemento
		elif modo==1:
			tempo=elemento
			modo=2
		else:
			modo=0
			item+=chr(int(tempo+elemento,16))
	return lista


def launch_program(program,salida=True):

	""" Launchs a program that can be located in any of the directories stored in PATHLIST """
	
	curDir=sys.path[-1:] # Allow launching programs from local directory when use py2exe
	pathlist=[]
	if not sys.platform == 'win32':
		pathlist=["/usr/bin","/usr/local/bin","/usr/share/bin","/usr/share/local/bin","/bin"]
	else:
		pathlist=[os.path.join(curDir[0],"bin"), r'C:\WINDOWS', r'C:\WINDOWS\system32', r'C:\WINNT']
	
	print "Launching program:"
	print "program: ",program
	print "Current working Directory: ", os.getcwd()
	
	for elemento in pathlist:
		#if elemento[-1]!="/":
		print "elemento: ", elemento
		if elemento[-1]!=os.sep:
			elemento+=os.sep
		try:
			program2=program[:]
			program2[0]=elemento+program2[0]

			if salida:
				#handle=subprocess.Popen(program2,executable=program[0],shell=False,bufsize=32768,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
				
				if sys.platform=='win32':
					handle=subprocess.Popen(program2,executable=program2[0], shell=False,bufsize=32767,stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
				else:
					handle=subprocess.Popen(program2,executable=program[0],shell=False,bufsize=32767,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
				
			else:
				#handle=subprocess.Popen(program2,executable=program[0],shell=False)
				
				if sys.platform=='win32':
					print "b"
					handle=subprocess.Popen(program2,executable=program2[0],shell=False,stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
				else:
					handle=subprocess.Popen(program2,executable=program[0],shell=False)
				
		except OSError:
			print "error in launch program\n\n\n\n"
			pass
		else:
			return handle
	return None


def calcula_tamano_total(structure):

	""" Calculates the total size of the DVD """

	total=0.0
	for elemento in structure:
		if len(elemento)>1:
			for encontrado in elemento[1:]:
				if encontrado["ismpeg"]:
					l=encontrado["filesize"]/1000
				else:
					l=float(((encontrado["vrate"]+encontrado["arate"])*encontrado["olength"])/8)
					if encontrado["cutting"]!=0:
						l/=2
				total+=l
	return total


def check_program(programa):

	""" This function allows to check that a program is available in the system, just
	by calling it without arguments and checking the error returned """

	#TODO switch /dev/null to windows compatable maybe "null"
	if not sys.platform=='win32':
		p=subprocess.Popen(programa+" >/dev/null 2>/dev/null",shell=True)
	else:
		#p=subprocess.Popen(programa)
		p=launch_program(programa) # call in list form

	p.wait()
	return p.returncode


def load_config(global_vars):

	""" Load the configuration """
	home=get_home_directory()
	global_vars["PAL"]=True

	# TODO change to allow a windows temp directory

	if sys.platform=='win32':
		t=os.path.split(os.path.split(home)[0])[0]+os.sep+"Local Settings"+os.sep+"Temp"+os.sep
		global_vars["temp_folder"]= t#r'C:\temp'
	else:
		global_vars["temp_folder"]="/var/tmp"
	
	print "Temp Directory is: " , global_vars["temp_folder"]
	
	if sys.platform=='win32':
		home=os.path.split(os.path.split(home)[0])[0]+os.sep+".devede"
	else:
		home+=".devede"

	print "home load: ", home
	menuformat_found=False
	try:
		archivo=open(home,"r")
		while True:
			linea=archivo.readline()
			print "linea: ", linea
			if linea=="":
				break
			if linea[-1]=="\n":
				linea=linea[:-1]
			if linea=="pal":
				global_vars["PAL"]=True
			if linea=="ntsc":
				global_vars["PAL"]=False
			if linea[:13]=="video_format:":
				if linea[13:]=="pal":
					global_vars["PAL"]=True
				if linea[13:]=="ntsc":
					global_vars["PAL"]=False
			if linea[:12]=="temp_folder:":
				global_vars["temp_folder"]=linea[12:]
			if linea[:12]=="menu_format:":
				if linea[12:]=="pal":
					menuformat_found=True
					global_vars["menu_PAL"]=True
				if linea[12:]=="ntsc":
					menuformat_found=True
					global_vars["menu_PAL"]=False
		archivo.close()
	except IOError:
		pass

	if menuformat_found==False:
		global_vars["menu_PAL"]=global_vars["PAL"] # if it's still not defined, we use the same format than the others


def save_config(global_vars):

	""" Stores the configuration """

	home=get_home_directory()

	if sys.platform=='win32':
		home=os.path.split(os.path.split(home)[0])[0]+os.sep+".devede"
	else:
		home+=".devede"

	if global_vars["temp_folder"][-1]!=os.sep:
		global_vars["temp_folder"]+=os.sep
	try:	
		archivo=open(home,"w")
		if global_vars["PAL"]:
			archivo.write("video_format:pal\n")
		else:
			archivo.write("video_format:ntsc\n")
		archivo.write("temp_folder:"+global_vars["temp_folder"]+"\n")
		if global_vars["menu_PAL"]:
			archivo.write("menu_format:pal")
		else:
			archivo.write("menu_format:ntsc")
		archivo.close()
	except IOError:
		pass


def addbarr(oldfilename):

	""" Transform each blank space into '\ ' to comply with the filename rules """

	output=""
	for letter in oldfilename:
		if letter==' ':
			output+='\\'
		output+=letter
	return output


def get_new_param(parameters):

	""" This function groups the parameters passed by the user into a list """

	new_param=""
	
	while(True):
		if (parameters.find(" ")==0):
			parameters=parameters[1:] # erase blank spaces at start
		else:
			break

	if len(parameters)==0:
		return "",""
	
	p0=0
	while True:
		p1=parameters.find('\\',p0)
		p2=parameters.find(' ',p0)
		if p2==p1+1:
			p0=p2+1
		else:
			if p2<0: # no next space, take all the string
				retorno=""
				doble=False
				print parameters
				for letra in parameters:
					if (letra!='\\') or doble:
						retorno+=letra
						doble=False
					else:
						doble=True
				return "",retorno
			else:
				retorno=""
				doble=False
				print parameters[:p2]
				for letra in parameters[:p2]:
					if (letra!='\\') or doble:
						retorno+=letra
						doble=False
					else:
						doble=True
				return parameters[p2+1:],retorno


def get_home_directory():
	
	if sys.platform == 'win32':
		name='Personal'
		SHELL_FOLDERS = r'Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders'
		HKCU=_winreg.HKEY_CURRENT_USER
		key=HKCU
		subkey=SHELL_FOLDERS
		key=_winreg.OpenKey(key,subkey)
		ret=_winreg.QueryValueEx(key,name)
		home=ret[0]
	else:
		home=os.environ.get("HOME")

	if home[-1]!=os.sep:
		home=home+os.sep

	print home
	return home


def return_time(seconds,empty):

	""" cuts a time in seconds into seconds, minutes and hours """

	seconds2=int(seconds)

	hours=str(seconds2/3600)
	if empty:
		if len(hours)==1:
			hours="0"+hours
	else:
		if hours=="0":
			hours=""
	if hours!="":
		hours+=":"
	
	minutes=str((seconds2/60)%60)
	if empty or (hours!=""):
		if len(minutes)==1:
			minutes="0"+minutes
	elif (minutes=="0") and (hours==""):
			minutes=""
	if minutes!="":
		minutes+=":"

	secs=str(seconds2%60)
	if (len(secs)==1) and (minutes!=""):
		secs="0"+secs

	return hours+minutes+secs
