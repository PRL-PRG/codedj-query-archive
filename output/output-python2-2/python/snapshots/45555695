#!/usr/bin/env python
# encoding: utf-8
# example of usage generated ui with custom widget
# second iteration:
#  * use pyqt-properties in widget
#  * signals are connected inside ui (in designer)

import sys
from PyQt4 import QtCore, QtGui
from demo_ui import Ui_Form

class MainWidget(QtGui.QWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        self.ui = Ui_Form()
        self.ui.setupUi(self)

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
