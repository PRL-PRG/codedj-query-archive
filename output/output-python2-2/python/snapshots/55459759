import sys
from PyQt4 import *
from nuevousuariobase import *



class NuevoUsuario(QtGui.QDialog, Ui_NuevoUsuario):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	


def main(args):
    app=QtGui.QApplication(args)
    win=NuevoUsuario()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
