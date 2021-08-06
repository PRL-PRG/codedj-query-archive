import sys
import os
from PyQt4 import *
from nuevafacturacionbase import *



class NuevaFacturacion(QtGui.QDialog, Ui_NuevaFacturacionBase):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	
    def on_mui_botonera_accepted(self):
	self.nomdb = self.mui_nomdb.text()
	self.command = 'su postgres -c createdb ' + self.nomdb
	QtGui.QMessageBox.warning(self, "My Application", self.command, QtGui.QMessageBox.Save | QtGui.QMessageBox.Discard | QtGui.QMessageBox.Cancel, QtGui.QMessageBox.Save)
#        os.system(command)


def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
