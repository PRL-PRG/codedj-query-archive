import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from modificarfacturacionbase import *



class ModificarFacturacion(QtGui.QDialog, Ui_ModificarFacturacionBase):
    def __init__(self, database, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
#	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	self.mui_nomdb.setText(database)
	self.mui_nomempresa.setText("hola mundo")
	self.mui_databaserevision.setText("adios mundo")
	# Establecemos cual va a ser la base de datos con la que trabajaremos todo el rato
	self.database = database
	self.nombre = self.execQuery('SELECT valor FROM configuracion where nombre =\'NombreEmpresa\';').replace('\n', '')
	self.databaserevision = self.execQuery('SELECT valor FROM configuracion where nombre =\'DatabaseRevision\';').replace('\n', '')
	self.mui_nomempresa.setText(self.nombre.replace('\n', ''))
	self.mui_databaserevision.setText(self.databaserevision.replace('\n', ''))

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

    def guardaQuery(self, query):
	self.query = query
	self.fileHandle = open ( '/tmp/query.sql', 'w' )
	self.fileHandle.write ( query )
	self.fileHandle.close()

    def execQuery(self, query):
	self.writecommand(QString("----") + self.database + QString("----"))
	self.subcomand = query
	self.guardaQuery(self.subcomand)
	self.command = 'su postgres -c \"psql -t -f /tmp/query.sql ' + self.database + '\"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	return QString(self.process.readAllStandardOutput())

    def on_mui_actualizardatabase_released(self):
	self.writecommand("Muy mal")

    def on_mui_hacerbackup_released(self):
	self.writecommand("Backup")
	
	self.savefile = QFileDialog.getSaveFileName(self,  QString("Guardar  Elija archivo destino"), QString("/home"), QString("SQL (*.sql *.pgdump)") )
	self.command = 'su postgres -c \"pg_dump -f ' + self.savefile + ' ' + self.database  + '\"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	self.writecommand(self.process.readAllStandardOutput())

def main(args):
    app=QtGui.QApplication(args)
    win=ModificarFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
