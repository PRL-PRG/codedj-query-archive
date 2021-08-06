# -*- coding: utf-8 -*-

import sys
import os
from PyQt4 import QtCore, QtGui
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from plugins import PluginsBulmaSetup
from empresa import Empresa
import plugins

class Facturacion(Empresa):
	def __init__(self, parent = None):
		Empresa.__init__(self)
		self.database = ''
		
	def writeConfig(self):
		self.writecommand('ESCRIBIENDO CONFIGURACION')
		self.writecommand("Escribiendo configuracion en "+ plugins.configfiles)
		self.file = QFile( plugins.configfiles + "bulmafact_" + self.database + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMAFACT   "
		
	
		# Como los plugins van por orden iteramos sobre el orden para arreglarlo.
		self.x = 1
		while (self.x < 100) :
			# Iteramos sobre la lista de plugins disponibles en bulmafact
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				# Si el plugin tiene el orden adecuado lo consideramos.
				if (self.pluginsbulmafact[self.i][7] == self.x ):
					self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
					# Si el plugin esta checked lo escribimos.
					if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked and len(self.pluginsbulmafact[self.i][1]) > 3):
						self.writecommand('Ha que actualizar ' + self.pluginsbulmafact[self.i][0])
						self.out << self.terminador << self.pluginsbulmafact[self.i][1]
						self.terminador = "; \\\n";
				self.i = self.i + 1
			self.x = self.x + 1
		self.out << "\n"
		self.file.close()
	
		if (self.mui_soporteTPV.isChecked()):
			self.file = QFile( plugins.configfiles + "bulmatpv_" + self.database + ".conf");
			if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
				return;
			self.out = QTextStream(self.file)
			self.terminador = ""
			self.out << "CONF_PLUGINS_BULMATPV   "
			
			# Como los plugins van por orden iteramos sobre el orden para arreglarlo.
			self.x = 1
			while (self.x < 100) :
				self.i = 0
				while (self.i < self.mui_plugins1.rowCount()):
					if (self.pluginsbulmatpv[self.i][7] == self.x):
						print 'Tratando:' + self.pluginsbulmatpv[self.i][1]
						self.writecommand('Tratando ' + self.pluginsbulmatpv[self.i][0])
						if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked and len(self.pluginsbulmatpv[self.i][1]) > 3):
							self.writecommand('Ha que actualizar ' + self.pluginsbulmatpv[self.i][0])
							self.out << self.terminador << self.pluginsbulmatpv[self.i][1]
							self.terminador = "; \\\n";
					self.i = self.i + 1
				self.x = self.x + 1
			self.out << "\n"
			self.file.close()
			

	def marcar(self, plug):
		self.j = 0
		for self.it in self.pluginsbulmatpv:
			if (self.pluginsbulmatpv[self.j][1] == plug):
				self.mui_plugins1.item(self.j,0).setCheckState(Qt.Checked)
			self.j = self.j + 1
		self.j = 0
		for self.it in self.pluginsbulmafact:
			if (self.pluginsbulmafact[self.j][1] == plug):
				self.mui_plugins.item(self.j,0).setCheckState(Qt.Checked)
			self.j = self.j + 1

	def desmarcar(self, plug):
		self.j = 0
		for self.it in self.pluginsbulmatpv:
			if (self.pluginsbulmatpv[self.j][1] == plug):
				self.mui_plugins1.item(self.j,0).setCheckState(Qt.Unchecked)
			self.j = self.j + 1
		self.j = 0
		for self.it in self.pluginsbulmafact:
			if (self.pluginsbulmafact[self.j][1] == plug):
				self.mui_plugins.item(self.j,0).setCheckState(Qt.Unchecked)
			self.j = self.j + 1

	def on_mui_plugins_cellClicked(self, row, col):
		if (self.semaforo == 1):
			# Marcamos las dependencias
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmafact[self.i][5].split(QString(","))
					for self.dep in self.arr:
						self.marcar(self.dep)
				self.i = self.i +1
			# Desmarcamos las incompatibilidades
			self.arr = self.pluginsbulmafact[row][6].split(QString(","))
			for self.dep in self.arr:
				self.desmarcar(self.dep)
			self.i = 0
			while (self.i < self.mui_plugins.rowCount()):
				if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmafact[self.i][6].split(QString(","))
					for self.dep in self.arr:
						self.desmarcar(self.dep)
				self.i = self.i +1

	def on_mui_plugins1_cellClicked(self, row, col):
		if (self.semaforo == 1):
			# Marcamos las dependencias
			self.i = 0
			while (self.i < self.mui_plugins1.rowCount()):
				if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmatpv[self.i][5].split(QString(","))
					for self.dep in self.arr:
						self.marcar(self.dep)
				self.i = self.i +1
			# Desmarcamos las incompatibilidades
			self.arr = self.pluginsbulmatpv[row][6].split(QString(","))
			for self.dep in self.arr:
				self.desmarcar(self.dep)
			while (self.i < self.mui_plugins1.rowCount()):
				if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked):
					self.arr = self.pluginsbulmatpv[self.i][6].split(QString(","))
					for self.dep in self.arr:
						self.desmarcar(self.dep)
				self.i = self.i +1
				
				
	def actualizarPlugins(self):
		self.writecommand('ACTUALIZANDO PLUGINS')
		self.i = 0
		while (self.i < self.mui_plugins.rowCount()):
			self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
			if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
                		self.writecommand('Actualizando ' + self.pluginsbulmafact[self.i][0])
				if (len(self.pluginsbulmafact[self.i][4]) > 0):
				    self.command = 'su postgres -c \"psql -t -f  ' + plugins.pathdbplugins + self.pluginsbulmafact[self.i][4] +' '+ self.database +'\"'
				    self.writecommand(self.command)
				    self.process.start(self.command)
				    self.process.waitForFinished(-1)
				    self.writecommand(self.process.readAllStandardOutput())
			self.i = self.i +1
	
		if (self.mui_soporteTPV.isChecked()):
			self.i = 0
			while (self.i < self.mui_plugins1.rowCount()):
				self.writecommand('Tratando ' + self.pluginsbulmatpv[self.i][0])
				if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked):
					if (len(self.pluginsbulmatpv[self.i][4]) >0):
					    self.writecommand('Actualizando ' + self.pluginsbulmatpv[self.i][0])
					    self.command = 'su postgres -c \"psql -t -f  ' + plugins.pathdbplugins + self.pluginsbulmatpv[self.i][4] +' '+ self.database +'\"'
					    self.writecommand(self.command)
					    self.process.start(self.command)
					    self.process.waitForFinished(-1)
					    self.writecommand(self.process.readAllStandardOutput())
				self.i = self.i +1

	def buscaPlugins(self):
		self.writecommand("Buscando Pluggins")
		self.semaforo = 0
	
		self.mui_plugins.setRowCount(len(self.pluginsbulmafact))
		self.i = 0
		while (self.i < len(self.pluginsbulmafact)):
			self.versioninst = self.buscaPluginInstalado(self.pluginsbulmafact[self.i][3], self.pluginsbulmafact[self.i][1])
			self.check = QTableWidgetItem(QtGui.QApplication.translate("MainWindow", self.pluginsbulmafact[self.i][0], None, QtGui.QApplication.UnicodeUTF8))
			self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
			self.check.setCheckState(Qt.Unchecked)
			if (self.versioninst != ''):
				self.check.setCheckState(Qt.Checked)
			self.mui_plugins.setItem(self.i, 0, self.check)
			self.mui_plugins.setItem(self.i, 2, QTableWidgetItem(self.versioninst))
			self.mui_plugins.setItem(self.i , 1 , QTableWidgetItem(QtGui.QApplication.translate("MainWindow",self.pluginsbulmafact[self.i][2], None, QtGui.QApplication.UnicodeUTF8)))
			self.mui_plugins.setRowHeight(self.i, 50)
			self.i = self.i + 1
		
		self.mui_plugins1.setRowCount(len(self.pluginsbulmatpv))
		self.i = 0
		while (self.i < len(self.pluginsbulmatpv)):
			self.versioninst = self.buscaPluginInstalado(self.pluginsbulmatpv[self.i][3], self.pluginsbulmatpv[self.i][1])
			self.check = QTableWidgetItem(QtGui.QApplication.translate("MainWindow", self.pluginsbulmatpv[self.i][0], None, QtGui.QApplication.UnicodeUTF8))
			self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
			self.check.setCheckState(Qt.Unchecked)
			if (self.versioninst != ''):
				self.check.setCheckState(Qt.Checked)
			self.mui_plugins1.setItem(self.i, 0, self.check)
			self.mui_plugins1.setItem(self.i, 2, QTableWidgetItem(self.versioninst))
			self.mui_plugins1.setItem(self.i , 1 , QTableWidgetItem(QtGui.QApplication.translate("MainWindow",self.pluginsbulmatpv[self.i][2], None, QtGui.QApplication.UnicodeUTF8)))
			self.mui_plugins1.setRowHeight(self.i, 50)
			self.i = self.i + 1
		self.semaforo = 1
		
		
	def buscaPluginInstalado(self, plugin, libreria):
		print '-->' + plugin +  ' ' + libreria
		self.version = self.execQuery('SELECT valor FROM configuracion WHERE nombre = \'' + plugin +'\'').replace('\n','').replace(' ','')
		if (len(self.version) > 2):
			return self.version
		
		if (libreria == ''):
			return ''
		
		self.mfile = QFile(plugins.configfiles + 'bulmafact_' + self.database + '.conf')
		if (self.mfile.exists()):
			self.command = 'grep '+libreria+' '+ plugins.configfiles + 'bulmafact_' + self.database + '.conf'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.version = self.process.readAllStandardOutput()
		
		if (self.version == ''):
			self.mfile = QFile(plugins.configfiles + 'bulmatpv_' + self.database + '.conf')
			if (self.mfile.exists()):
				self.command = 'grep '+libreria+' '+ plugins.configfiles + 'bulmatpv_' + self.database + '.conf'
				self.writecommand(self.command)
				self.process.start(self.command)
				self.process.waitForFinished(-1)
				self.version = self.process.readAllStandardOutput()			
			
		if (self.version != ''):
			self.version = '0.11'
		return QString(self.version)
		
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
			
	def actualizarDatabase(self):
		# Aplicamos el parche de bulmatpv
		if (self.mui_soporteTPV.isChecked()):
			self.command = 'su postgres -c "psql ' + self.database + ' < '+ plugins.pathdbbulmatpv+'bulmatpv_schema.sql"'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			
			
		self.revisiones = ["revf-0.5.9.sql","revf-0.9.1.sql", "revf-0.9.3.sql", "revf-0.10.sql", "revf-0.11.sql"]
		#Parcheamos todo lo que hay que parchear
		for self.parche in self.revisiones:
			self.command = 'su postgres -c \"psql -t -f  ' + plugins.pathdbparches + self.parche + ' ' + self.database  + '\"'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.writecommand(self.process.readAllStandardOutput())