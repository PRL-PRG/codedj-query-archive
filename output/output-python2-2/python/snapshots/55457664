# -*- coding: utf-8 -*-

import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from restbackupbase import *




class RestBackup(QtGui.QDialog, Ui_RestBackupBase):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	self.process = QtCore.QProcess()	
	

    def on_mui_aceptar_released(self):
	self.nomdb = self.mui_dbname.text()
	
	# Creamos la base de datos
	self.command = 'su postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
#	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	# Cargamos la esquematica de la base de datos
	self.command = 'su postgres -c \"psql  -f '+self.mui_filename.text() +' ' + self.nomdb + ' \"'
	print self.command
#	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	self.accept()

    def on_mui_filesearch_released(self):
	self.openfile = QFileDialog.getOpenFileName(self,  QString("Restaurar  Elija archivo de backup"), QString("/home"), QString("SQL (*.sql *.pgdump)") )
	self.mui_filename.setText(self.openfile)
	
def main(args):
    app=QtGui.QApplication(args)
    win=RestBackup()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
