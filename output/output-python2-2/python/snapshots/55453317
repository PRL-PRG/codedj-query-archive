# -*- coding: utf-8 -*-

import sys
import os,fnmatch
import re

global pathdbbulmafact
global pathdbbulmacont
global pathdbparches
global pathdbplugins
global configfiles
global confsharebulmages


class PluginsBulmaSetup:
    def __init__(self):
	    
	self.pluginsbulmafact = []
	self.pluginsbulmacont = []
	self.pluginsbulmatpv  = []
	    
	self.readfiles("bulmafact-plugins",self.pluginsbulmafact)
	self.readfiles("bulmalib-plugins",self.pluginsbulmafact)
	self.readfiles("bulmacont-plugins",self.pluginsbulmacont)
	self.readfiles("bulmalib-plugins",self.pluginsbulmacont)
	self.readfiles("bulmatpv-plugins",self.pluginsbulmatpv)


    def readfiles(self, folder, plugins):
	for fileName in os.listdir ( confsharebulmages+folder ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+folder+"/"+fileName)
			cont = ""
			for char in f.read():
				cont = cont + char
			nombre = ""
			biblioteca = ""
			descripcion = ""
			parmdb = ""
			archivosqlpatch = ""
			archivosqldeins = ""
			m = re.search('Nombre: (.*)', cont)
			if m <> None:
				nombre = m.group(1)
			m = re.search('Biblioteca: (.*)', cont)
			if m <> None:
				biblioteca = m.group(1)
			m = re.search("(?smu)Descripción: (.*)\n\nParm", cont)
			if m <> None:
				descripcion = m.group(1)
			m = re.search('ParmDB: (.*)', cont)
			if m <> None:
				parmdb = m.group(1)
			m = re.search('ArchivoSQLpatch: (.*)', cont)
			if m <> None:
				archivosqlpatch = m.group(1)
			m = re.search('ArchivoSQLdeins: (.*)', cont)
			if m <> None:
				archivosqldeins = m.group(1)
			plugins.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])


if __name__=="__main__":
   pathdbbulmafact = ""
   pathdbbulmacont = ""
   pathdbparches = ""
   configfiles = ""
   confsharebulmages = "/opt/bulmages/share/bulmages/"
   win=PluginsBulmaSetup()