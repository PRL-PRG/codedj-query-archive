__version__ = "$Revision: 0.1 $"[11:-2]
__date__ = "$Date: 2007/01/07 08:09:13 $"[7:-2]
__author__ = "Tomeu Borras <tborras@conetxia.com>"
__all__ = ["depurator"]
__doc__ = "Sistema de Depuracion para BulmaGes a partir de los logs.\r\n"

import sys
from bulmasetup import *
from PyQt4 import *
from nuevousuario import NuevoUsuario


class HelloWindow(QtGui.QMainWindow, Ui_MainWindow):
    def __init__(self, *args):
        apply(QtGui.QMainWindow.__init__, (self,) + args)
	self.setupUi(self)
	
    def on_mui_crearusuario_clicked(self):
	    win = NuevoUsuario()
	    win.exec_()

def main(args):
    app=QtGui.QApplication(args)
    win=HelloWindow()
    win.show()
    app.connect(app, QtCore.SIGNAL("lastWindowClosed()"),
                app, QtCore.SLOT("quit()"))
    app.exec_()

if __name__=="__main__":
    main(sys.argv)
