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
	self.mui_textBrowser.append("error: " + QString(self.process.readAllStandardError()))

    def finished(self):
	self.mui_textBrowser.append("Done.")

    def started(self):
	self.mui_textBrowser.append("Start.")


    def on_mui_aceptar_released(self):
	self.hayplugins = 0
	self.nomdb = self.mui_nomdb.text()
	self.command = 'kdesu -u postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
	self.mui_textBrowser.append(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_schema.sql"'
	self.mui_textBrowser.append(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_data.sql"'
	self.mui_textBrowser.append(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'kdesu -u postgres -c \'psql ' + self.nomdb + ' -c \"' +self.subcomand+ '\"\''
	self.mui_textBrowser.append(self.command)
        os.system(self.command.toAscii().data())

	if (self.mui_contratos.isChecked()):
		self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-contratos.sql"'
		self.mui_textBrowser.append(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1
	
	if (self.hayplugins == 1):
		self.mui_textBrowser.append("Escribiendo configuracion en /etc/bulmages")
		self.file = QFile("/etc/bulmages/bulmafact_" + self.nomdb + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMAFACT   "
		
		if (self.mui_contratos.isChecked()):
			self.out << self.terminador << "libplugincontratos.so"
			self.terminador = "; \\ \n";
		
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
