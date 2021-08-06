import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevafacturacionbase import *



class NuevaFacturacion(QtGui.QDialog, Ui_NuevaFacturacionBase):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
#	self.mui_textBrowser.setTextFormat(QTextBrowser.LogText)
	
	self.process = QtCore.QProcess()
	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	
#	self.connect(self.process, SIGNAL("processExited()"), self.resetButtons)

    def readOutput(self):
	self.mui_textBrowser.append(QString(self.process.readAllStandardOutput()))

    def readErrors(self):
	self.mui_textBrowser.append("error: " + QString(self.process.readAllStandardError()))

    def finished(self):
	self.mui_textBrowser.append("Done.")

    def started(self):
	self.mui_textBrowser.append("Start.")


    def on_mui_aceptar_released(self):
	self.nomdb = self.mui_nomdb.text()
	self.command = 'kdesu -u postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	self.command = 'kdesu -u postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_schema.sql"'
	self.process.start(self.command)
	self.process.waitForFinished(-1)
#        os.system(self.command.toAscii())

	self.command = 'psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_data.sql'
#        os.system(self.command.toAscii())

	self.nomempresa = self.mui_nomempresa.text()
	self.command = 'echo "UPDATE configuracion set valor=\'self.nomempresa\' WHERE nombre = \'NombreEmpresa\';\" | psql ' + self.nomdb
#        os.system(self.command.toAscii())

	



def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
