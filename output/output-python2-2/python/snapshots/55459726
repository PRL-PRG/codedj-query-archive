import sys
import os
from PyQt4 import *
from nuevousuariobase import *




class NuevoUsuario(QtGui.QDialog, Ui_NuevoUsuario):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	
    def on_mui_botonera_accepted(self):
	QtGui.QMessageBox.warning(self, "My Application", "The document has been modified.\n" "Do you want to save your changes?",QtGui.QMessageBox.Save | QtGui.QMessageBox.Discard | QtGui.QMessageBox.Cancel, QtGui.QMessageBox.Save)
        os.system('xeyes')

def main(args):
    app=QtGui.QApplication(args)
    win=NuevoUsuario()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
