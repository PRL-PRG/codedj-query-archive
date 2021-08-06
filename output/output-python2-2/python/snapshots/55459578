import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevafacturacionbase import *



class NuevaFacturacion(QtGui.QDialog, Ui_NuevaFacturacionBase):
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
	self.command = 'kdesu -u postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	# Cargamos la esquematica de la base de datos
	self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_schema.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cargamos los datos minimos
	self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_data.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cambiamos el nombre de la empresa
	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'kdesu -u postgres -c \'psql ' + self.nomdb + ' -c \"' +self.subcomand+ '\"\''
	self.writecommand(self.command)
        os.system(self.command.toAscii().data())

	# Si esta seleccionado el checkbox de contratos cargamos el parche de contratos
	if (self.mui_contratos.isChecked()):
		self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-contratos.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1
	
	# Si hay plugins seleccionados escribimos la configuracion para esta empresa
	if (self.hayplugins == 1):
		self.writecommand("Escribiendo configuracion en /etc/bulmages")
		self.file = QFile("/etc/bulmages/bulmafact_" + self.nomdb + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMAFACT   "
		
		# Ponemos la configuracion de contratos
		if (self.mui_contratos.isChecked()):
			self.out << self.terminador << "libplugincontratos.so"
			self.terminador = "; \\ \n";
		
		# Ponemos la configuracion del plugin de llamadas
		if (self.mui_llamadas.isChecked()):
			self.out << self.terminador << "libpluginllamadas.so"
			self.terminador = "; \\ \n";

		self.out << "\n"
	self.mui_textBrowser.append("Done.")

	self.file.close()


def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
