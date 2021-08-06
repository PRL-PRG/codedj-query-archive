# -*- coding: utf-8 -*-

import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevafacturacionbase import *
from plugins import PluginsBulmaSetup
import plugins

class NuevaFacturacion(QtGui.QDialog, Ui_NuevaFacturacionBase, PluginsBulmaSetup):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	PluginsBulmaSetup.__init__(self)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	
	self.buscaPlugins()
	# Ajustamos la presentacion
	self.mui_plugins.resizeColumnsToContents()
	self.mui_checkbox.setCheckState(Qt.Unchecked)
	
    def readOutput(self):
	self.mui_textBrowser.append(QString(self.process.readAllStandardOutput()))

    def readErrors(self):
	self.mui_textBrowser.append("<font color =\"#FF0000\">error: " + QString(self.process.readAllStandardError()) + "</font>")

    def finished(self):
	self.mui_textBrowser.append("<font color =\"#00FF00\">Done.</font>")

    def started(self):
	self.mui_textBrowser.append("<font color =\"#00FF00\">Start.</font>")

    def writecommand(self, comm):
	self.mui_textBrowser.append("<font color =\"#0000FF\">"+comm+"</font>")


    def buscaPlugins(self):
	self.writecommand("Buscando Pluggins")

	self.mui_plugins.setRowCount(len(self.pluginsbulmafact))
	self.i = 0
	while (self.i < len(self.pluginsbulmafact)):
		self.check = QTableWidgetItem(QtGui.QApplication.translate("MainWindow", self.pluginsbulmafact[self.i][0], None, QtGui.QApplication.UnicodeUTF8))
		self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
		self.check.setCheckState(Qt.Unchecked)
		self.mui_plugins.setItem(self.i, 0, self.check)
		self.mui_plugins.setItem(self.i , 1 , QTableWidgetItem(QtGui.QApplication.translate("MainWindow",self.pluginsbulmafact[self.i][2], None, QtGui.QApplication.UnicodeUTF8)))
		self.mui_plugins.setRowHeight(self.i, 50)
		self.i = self.i + 1
	
	self.mui_plugins1.setRowCount(len(self.pluginsbulmatpv))
	self.i = 0
	while (self.i < len(self.pluginsbulmatpv)):
		self.check = QTableWidgetItem(QtGui.QApplication.translate("MainWindow", self.pluginsbulmatpv[self.i][0], None, QtGui.QApplication.UnicodeUTF8))
		self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
		self.check.setCheckState(Qt.Unchecked)
		self.mui_plugins1.setItem(self.i, 0, self.check)
		self.mui_plugins1.setItem(self.i , 1 , QTableWidgetItem(QtGui.QApplication.translate("MainWindow",self.pluginsbulmatpv[self.i][2], None, QtGui.QApplication.UnicodeUTF8)))
		self.mui_plugins1.setRowHeight(self.i, 50)
		self.i = self.i + 1
	
	
	
    def actualizarPlugins(self):
	self.writecommand('ACTUALIZANDO PLUGINS')
	self.i = 0
	while (self.i < self.mui_plugins.rowCount()):
		self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
		if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
			self.writecommand('Ha que actualizar ' + self.pluginsbulmafact[self.i][0])
			self.command = 'su postgres -c \"psql -t -f  ' + plugins.pathdbparches + self.pluginsbulmafact[self.i][4] +' '+ self.nomdb +'\"'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.writecommand(self.process.readAllStandardOutput())
			self.hayplugins = 1
		self.i = self.i +1

	if (self.mui_soporteTPV.isChecked()):
		self.i = 0
		while (self.i < self.mui_plugins1.rowCount()):
			self.writecommand('Tratando ' + self.pluginsbulmatpv[self.i][0])
			if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked):
				self.writecommand('Ha que actualizar ' + self.pluginsbulmatpv[self.i][0])
				self.command = 'su postgres -c \"psql -t -f  ' + plugins.pathdbparches + self.pluginsbulmatpv[self.i][4] +' '+ self.nomdb +'\"'
				self.writecommand(self.command)
				self.process.start(self.command)
				self.process.waitForFinished(-1)
				self.writecommand(self.process.readAllStandardOutput())
				self.haypluginstpv = 1
			self.i = self.i +1


    def writeConfig(self):
	self.writecommand('ESCRIBIENDO CONFIGURACION')
	self.writecommand("Escribiendo configuracion en " + plugins.configfiles)
	self.file = QFile( plugins.configfiles + "bulmafact_" + self.nomdb + ".conf");
	if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
		return;
	self.out = QTextStream(self.file)
	self.terminador = ""
	self.out << "CONF_PLUGINS_BULMAFACT   "
	
	
	self.i = 0
	while (self.i < self.mui_plugins.rowCount()):
		self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
		if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
			self.writecommand('Ha que actualizar ' + self.pluginsbulmafact[self.i][0])
			self.out << self.terminador << self.pluginsbulmafact[self.i][1]
			self.terminador = "; \\\n";
		self.i = self.i +1
	self.out << "\n"
	self.file.close()

	self.file = QFile( plugins.configfiles + "bulmatpv_" + self.nomdb + ".conf");
	if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
		return;
	self.out = QTextStream(self.file)
	self.terminador = ""
	self.out << "CONF_PLUGINS_BULMATPV   "
	
	if (self.mui_soporteTPV.isChecked()):
		self.i = 0
		while (self.i < self.mui_plugins1.rowCount()):
			self.writecommand('Tratando ' + self.pluginsbulmatpv[self.i][0])
			if (self.mui_plugins1.item(self.i, 0).checkState() == Qt.Checked):
				self.writecommand('Ha que actualizar ' + self.pluginsbulmatpv[self.i][0])
				self.out << self.terminador << self.pluginsbulmatpv[self.i][1]
				self.terminador = "; \\\n";
			self.i = self.i +1
		self.out << "\n"
		self.file.close()


    def on_mui_aceptar_released(self):
	self.mui_textBrowser.clear()
	
	self.hayplugins = 0
	self.nomdb = self.mui_nomdb.text()
	
	# Creamos la base de datos
	self.command = 'su postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	# Cargamos la esquematica de la base de datos
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < '+ plugins.pathdbbulmafact+'bulmafact_schema.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cargamos los datos minimos
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < '+ plugins.pathdbbulmafact+'bulmafact_data.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Aplicamos el parche de bulmatpv
	if (self.mui_soporteTPV.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < '+ plugins.pathdbbulmatpv+'bulmatpv_schema.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)

	# Cambiamos el nombre de la empresa
	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'su postgres -c \'psql ' + self.nomdb + ' -c \"' +self.subcomand+ '\"\''
	self.writecommand(self.command)
        os.system(self.command.toAscii().data())

	self.hayplugins = 0
	self.actualizarPlugins()

	# Si hay plugins seleccionados escribimos la configuracion para esta empresa
	if (self.hayplugins == 1):
		self.writeConfig()
		
	self.mui_textBrowser.append("Done.")




def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
