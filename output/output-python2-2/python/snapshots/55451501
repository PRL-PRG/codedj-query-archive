# -*- coding: utf-8 -*-

import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevafacturacionbase import *
from plugins import PluginsBulmaSetup
from facturacion import Facturacion
import plugins

class NuevaFacturacion(Ui_NuevaFacturacionBase, Facturacion):
    def __init__(self, parent = None):
	Facturacion.__init__(self)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	
	self.buscaPlugins()
	# Ajustamos la presentacion
	self.mui_plugins.resizeColumnsToContents()
	self.mui_plugins1.resizeColumnsToContents()	
	self.mui_checkbox.setCheckState(Qt.Unchecked)

    def on_mui_aceptar_released(self):
	self.mui_textBrowser.clear()
	
	self.database = self.mui_nomdb.text()
	
	# Creamos la base de datos
	self.command = 'su postgres -c "createdb -E UNICODE ' + self.database +'"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	# Cargamos la esquematica de la base de datos
	self.command = 'su postgres -c "psql ' + self.database + ' < '+ plugins.pathdbbulmafact+'bulmafact_schema.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cargamos los datos minimos
	self.command = 'su postgres -c "psql ' + self.database + ' < '+ plugins.pathdbbulmafact+'bulmafact_data.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Aplicamos el parche de bulmatpv si es necesario
	if (self.mui_soporteTPV.isChecked()):
		self.command = 'su postgres -c "psql ' + self.database + ' < '+ plugins.pathdbbulmatpv+'bulmatpv_schema.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)

	# Cambiamos el nombre de la empresa
	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'su postgres -c \'psql ' + self.database + ' -c \"' +self.subcomand+ '\"\''
	self.writecommand(self.command)
        os.system(self.command.toAscii().data())

	self.actualizarPlugins()

	# Escribimos la configuracion
	self.writeConfig()
		
	self.mui_textBrowser.append("Done.")




def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
