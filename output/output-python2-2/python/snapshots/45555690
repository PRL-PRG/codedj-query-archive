#!/usr/bin/env python
# encoding: utf-8
# example of usage generated ui with custom widget
# first iteration:
#  * simple widget, without pyqt properties
#  * signals are connected by hand

import sys
from PyQt4 import QtCore, QtGui
from demo_ui import Ui_Form

class MainWidget(QtGui.QWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        self.ui = Ui_Form()
        self.ui.setupUi(self)
        self.connectSignals()
    
    def connectSignals(self):
        # мы хотим, чтобы при вводе данных в lineEdit
        # эти же символы появлялись в errStateLineEdit
        
        self.connect(
            self.ui.lineEdit, 
            QtCore.SIGNAL("textEdited(QString)"),
            self.ui.errStateLineEdit.setText
        )
        # и при каждом вводе тот бы мигал фоном
        self.connect(
            self.ui.lineEdit, 
            QtCore.SIGNAL("textEdited(QString)"),
            self.ui.errStateLineEdit.setErrorState
        )

class AppMainWindow(QtGui.QMainWindow):
    def __init__(self, parent=None):
        QtGui.QMainWindow.__init__(self, parent)
        self.setCentralWidget(MainWidget(self))


def run(args):
    app = QtGui.QApplication(args)
    window = AppMainWindow()
    window.show()
    sys.exit(app.exec_())

if __name__ == '__main__':
    run(sys.argv)
