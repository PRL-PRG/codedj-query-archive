# -*- coding: utf-8 -*-

import sys
import os,fnmatch
import re

global pathdbbulmafact
global pathdbbulmacont
global pathdbparches
global configfiles
global confsharebulmages


class PluginsBulmaSetup:
    def __init__(self):
	    
	self.pluginsbulmafact = []
	self.pluginsbulmacont = []
	self.pluginsbulmatpv  = []
	    
	for fileName in os.listdir ( confsharebulmages+"bulmafact-plugins" ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+"bulmafact-plugins/"+fileName)
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
			m = re.search("(?smu)Descripción: (.*)Parm", cont)
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
			self.pluginsbulmafact.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])
	for fileName in os.listdir ( confsharebulmages+"bulmalib-plugins" ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+"bulmalib-plugins/"+fileName)
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
			m = re.search("(?smu)Descripción: (.*)Parm", cont)
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
			self.pluginsbulmafact.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])
				    






	for fileName in os.listdir ( confsharebulmages+"bulmacont-plugins" ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+"bulmacont-plugins/"+fileName)
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
			m = re.search("(?smu)Descripción: (.*)Parm", cont)
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
			self.pluginsbulmacont.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])
	for fileName in os.listdir ( confsharebulmages+"bulmalib-plugins" ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+"bulmalib-plugins/"+fileName)
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
			m = re.search("(?smu)Descripción: (.*)Parm", cont)
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
			self.pluginsbulmacont.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])




	for fileName in os.listdir ( confsharebulmages+"bulmatpv-plugins" ):
		if fnmatch.fnmatch ( fileName, 'README*' ):
			f = open(confsharebulmages+"bulmatpv-plugins/"+fileName)
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
			m = re.search("(?smu)Descripción: (.*)Parm", cont)
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
			self.pluginsbulmatpv.append([nombre, biblioteca, descripcion, parmdb, archivosqlpatch])





if __name__=="__main__":
  win=PluginsBulmaSetup()