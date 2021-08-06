__version__ = "$Revision: 0.1 $"[11:-2]
__date__ = "$Date: 2007/01/07 08:09:13 $"[7:-2]
__author__ = "Tomeu Borras <tborras@conetxia.com>"
__all__ = ["bulmasetup"]
__doc__ = "Sistema de Depuracion para BulmaGes a partir de los logs.\r\n"

import sys
from bulmasetup import *
from PyQt4 import *
from nuevousuario import NuevoUsuario
from nuevafacturacion import NuevaFacturacion
from nuevacontabilidad import NuevaContabilidad
from listempresas import ListEmpresas

class HelloWindow(QtGui.QMainWindow, Ui_MainWindow):
    def __init__(self, *args):
        apply(QtGui.QMainWindow.__init__, (self,) + args)
	self.setupUi(self)
	
    def on_mui_crearusuario_released(self):
	    win = NuevoUsuario()
	    win.exec_()
	    
    def on_mui_crearbulmafact_released(self):
	    win = NuevaFacturacion()
	    win.exec_()
	    
    def on_mui_crearbulmacont_released(self):
	    win = NuevaContabilidad()
	    win.exec_()
	    
    def on_mui_adminempresas_released(self):
	    win = ListEmpresas()
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
