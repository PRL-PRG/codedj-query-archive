import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevacontabilidadbase import *



class NuevaContabilidad(QtGui.QDialog, Ui_NuevaContabilidadBase):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)

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
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmacont/bulmacont_schema.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cargamos los datos minimos
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmacont/t_configuracion_data.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cambiamos el nombre de la empresa
	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'su postgres -c \'psql ' + self.nomdb + ' -c \"' +self.subcomand+ '\"\''
	self.writecommand(self.command)
        os.system(self.command.toAscii().data())

	# Si esta seleccionado el checkbox de contratos cargamos el parche de contratos
	if (self.mui_pluginresumcta.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginbalancetree.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginregistrodeiva.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginbalance1.isChecked()):
		self.hayplugins = 1
		
	if (self.mui_plugindocked.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginsubformsxc.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginclipboardbf.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginsubformods.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugincanualesods.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginimpers.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginreportods.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugincorrector.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginproyectos.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/rev-pluginpresupuestos.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_plugindebugbc.isChecked()):
		self.hayplugins = 1

	# Si hay plugins seleccionados escribimos la configuracion para esta empresa
	if (self.hayplugins == 1):
		self.writecommand("Escribiendo configuracion en /etc/bulmages")
		self.file = QFile("/etc/bulmages/bulmacont_" + self.nomdb + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMACONT   "
			
		if (self.mui_pluginresumcta.isChecked()):
			self.out << self.terminador << "libpluginresumcta.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginbalancetree.isChecked()):
			self.out << self.terminador << "libpluginbalancetree.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginregistrodeiva.isChecked()):
			self.out << self.terminador << "libpluginregistrodeiva.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginbalance1.isChecked()):
			self.out << self.terminador << "libpluginbalance1.so"
			self.terminador = "; \\\n";
			
		if (self.mui_plugindocked.isChecked()):
			self.out << self.terminador << "libplugindocked.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginsubformsxc.isChecked()):
			self.out << self.terminador << "libpluginsubformsxc.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginclipboardbf.isChecked()):
			self.out << self.terminador << "libpluginclipboardbf.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginsubformods.isChecked()):
			self.out << self.terminador << "libpluginsubformods.so"
			self.terminador = "; \\\n";
	
		if (self.mui_plugincanualesods.isChecked()):
			self.out << self.terminador << "libplugincanualesods.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginimpers.isChecked()):
			self.out << self.terminador << "libpluginimpers.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginreportods.isChecked()):
			self.out << self.terminador << "libpluginreportods.so"
			self.terminador = "; \\\n";
	
		if (self.mui_plugincorrector.isChecked()):
			self.out << self.terminador << "libplugincorrector.so"
			self.terminador = "; \\\n";
	
		if (self.mui_pluginproyectos.isChecked()):
			self.out << self.terminador << "libpluginproyectos.so"
			self.terminador = "; \\\n";
	
		if (self.mui_plugindebugbc.isChecked()):
			self.out << self.terminador << "libplugindebugbc.so"
			self.terminador = "; \\\n";

		self.out << "\n"
		self.file.close()
	self.mui_textBrowser.append("Done.")

def main(args):
    app=QtGui.QApplication(args)
    win=NuevaContabilidad()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
