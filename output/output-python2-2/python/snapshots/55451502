# -*- coding: utf-8 -*-

import sys
import os
from PyQt4 import QtCore, QtGui
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from plugins import PluginsBulmaSetup
from empresa import Empresa
import plugins

class Contabilidad(Empresa):
	def __init__(self, parent = None):
		Empresa.__init__(self)
		self.database = ''
		
	def actualizarDatabase(self):
		self.revisiones = ["rev-0.5.3.sql", "rev-0.9.1.sql", "rev-0.9.3.sql", "rev-0.10.sql", "rev-0.11.sql"]
		#Parcheamos todo lo que hay que parchear
		for self.parche in self.revisiones:
			self.command = 'su postgres -c \"psql -t -f ' + plugins.pathdbparches + self.parche + ' ' + self.database  + '\"'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.writecommand(self.process.readAllStandardOutput())
			self.actualizarPlugins()
			

	def buscaPlugins1(self):
		self.plugins = self.execQuery('SELECT nombre, valor FROM configuracion WHERE nombre LIKE \'DBRev-%\'')
		self.writecommand(self.plugins)
		print self.plugins
		self.arrplugins = self.plugins.split(QString("\n"))
		self.mui_plugins.setRowCount(self.arrplugins.count() -3)
		self.i = 0
		while (self.i < self.arrplugins.count() ):
			self.writecommand(self.arrplugins[self.i])
			self.valores = self.arrplugins[self.i].split(QString("|"))
			if (self.valores.count() >= 2):
				self.mui_plugins.setItem(self.i-1 , 1 , QTableWidgetItem(self.valores[1].replace('\n', '')))
				self.mui_plugins.setItem(self.i-1 , 0 , QTableWidgetItem(self.valores[0].replace('\n', '')))
	
			self.i = self.i + 1
	
	def buscaPluginInstalado(self, plugin, libreria):
		print '-->' + plugin +  ' ' + libreria
		self.version = self.execQuery('SELECT valor FROM configuracion WHERE nombre = \'' + plugin +'\'').replace('\n','').replace(' ','')
		if (len(self.version) > 2):
			return self.version
		
		if (libreria == ''):
			return ''
		
		self.mfile = QFile(plugins.configfiles + 'bulmacont_' + self.database + '.conf')
		if (self.mfile.exists()):
			self.command = 'grep '+libreria+' '+ plugins.configfiles + 'bulmacont_' + self.database + '.conf'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.version = self.process.readAllStandardOutput()

		if (self.version != ''):
			self.version = '0.11'
		return QString(self.version)
	
		
	def buscaPlugins(self):
		self.writecommand("Buscando Pluggins")
		self.semaforo = 0
	
		self.mui_plugins.setRowCount(len(self.pluginsbulmacont))
		self.i = 0
		while (self.i < len(self.pluginsbulmacont)):
			self.versioninst = self.buscaPluginInstalado(self.pluginsbulmacont[self.i][3], self.pluginsbulmacont[self.i][1])
			self.check = QTableWidgetItem(QtGui.QApplication.translate("MainWindow", self.pluginsbulmacont[self.i][0], None, QtGui.QApplication.UnicodeUTF8))
			self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
			self.check.setCheckState(Qt.Unchecked)
			if (self.versioninst != ''):
				self.check.setCheckState(Qt.Checked)
			self.mui_plugins.setItem(self.i, 0, self.check)
			self.mui_plugins.setItem(self.i, 2, QTableWidgetItem(self.versioninst))
			self.mui_plugins.setItem(self.i , 1 , QTableWidgetItem(QtGui.QApplication.translate("MainWindow",self.pluginsbulmacont[self.i][2], None, QtGui.QApplication.UnicodeUTF8)))
			self.mui_plugins.setRowHeight(self.i, 50)
			self.i = self.i + 1
	
	def actualizarPlugins(self):
		self.writecommand('ACTUALIZANDO PLUGINS')
		self.i = 0
		while (self.i < self.mui_plugins.rowCount()):
			self.writecommand('Tratando ' + self.pluginsbulmacont[self.i][0])
			if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
				self.writecommand('Ha que actualizar ' + self.pluginsbulmacont[self.i][0])
				self.command = 'su postgres -c \"psql -t -f ' + plugins.pathdbplugins + self.pluginsbulmacont[self.i][4] + ' ' + self.database + '\"'
				self.writecommand(self.command)
				self.process.start(self.command)
				self.process.waitForFinished(-1)
				self.writecommand(self.process.readAllStandardOutput())
			self.i = self.i +1
			
	def marcar(self, plug):
		self.j = 0
		for self.it in self.pluginsbulmacont:
			if (self.pluginsbulmacont[self.j][1] == plug):
				self.mui_plugins.item(self.j,0).setCheckState(Qt.Checked)
			self.j = self.j + 1

	def desmarcar(self, plug):
		self.j = 0
		for self.it in self.pluginsbulmacont:
			if (self.pluginsbulmacont[self.j][1] == plug):
				self.mui_plugins.item(self.j,0).setCheckState(Qt.Unchecked)
			self.j = self.j + 1

	def on_mui_plugins_cellClicked(self, row, col):
		if (self.semaforo == 1):
			# Marcamos las dependencias
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmacont[self.i][5].split(QString(","))
					for self.dep in self.arr:
						self.marcar(self.dep)
				self.i = self.i +1
			# Desmarcamos las incompatibilidades
			self.arr = self.pluginsbulmacont[row][6].split(QString(","))
			for self.dep in self.arr:
				self.desmarcar(self.dep)
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmacont[self.i][6].split(QString(","))
					for self.dep in self.arr:
						self.desmarcar(self.dep)
				self.i = self.i +1

	def writeConfig(self):
		self.writecommand('ESCRIBIENDO CONFIGURACION')
		self.writecommand("Escribiendo configuracion en " + plugins.configfiles )
		self.file = QFile( plugins.configfiles + "bulmacont_" + self.database + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMACONT   "
		
		
		# Como los plugins van por orden iteramos sobre el orden para arreglarlo.
		self.x = 1
		while (self.x < 100) :
			# Iteramos sobre la lista de plugins disponibles en bulmacont
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				# Si el plugin tiene el orden adecuado lo consideramos.
				if (self.pluginsbulmacont[self.i][7] == self.x ):
					self.writecommand('Tratando ' + self.pluginsbulmacont[self.i][0])
					# Si el plugin esta checked lo escribimos.
					if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked and len(self.pluginsbulmacont[self.i][1]) > 3):
						self.writecommand('Ha que actualizar ' + self.pluginsbulmacont[self.i][0])
						self.out << self.terminador << self.pluginsbulmacont[self.i][1]
						self.terminador = "; \\\n";
				self.i = self.i + 1
			self.x = self.x + 1
		self.out << "\n"
		self.file.close()
