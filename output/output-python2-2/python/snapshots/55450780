# -*- coding: utf-8 -*-

import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevousuariobase import *




class NuevoUsuario(QtGui.QDialog, Ui_NuevoUsuario):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	self.process = QtCore.QProcess()	
	
    def on_mui_botonera_accepted(self):
	# Creamos el usuario
	self.command = 'su postgres -c "createuser -s -d -r  \'' + self.mui_nombre.text().replace('\\','\\\\').replace('\'','\\\'') +'\'"'
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cambiamos el password del usuario
	self.subcomand = 'ALTER USER "' +self.mui_nombre.text().replace('"','""')+ '" WITH PASSWORD E\'' +self.mui_password.text().replace('\\','\\\\').replace('\'','\\\'') +'\' ;'
	self.command = 'su postgres -c \'' + ('psql template1 -c \"' +self.subcomand.replace('\\','\\\\').replace('`','\\`').replace('$','\\$').replace('\n','\\\n').replace('"','\\"')+ '\"' ).replace('\'','\'"\'"\'')+'\''
        os.system(self.command.toAscii().data())
	
	self.accept()


def main(args):
    app=QtGui.QApplication(args)
    win=NuevoUsuario()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
