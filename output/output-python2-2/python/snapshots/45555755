#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Example of handmade UI widget
"""

from PyQt4 import QtCore, QtGui

from designed_ui import Ui_DesignedWidget

class InheritedWidget(QtGui.QWidget, Ui_DesignedWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        self.setupUi(self)

if __name__ == '__main__':
    import sys
    app = QtGui.QApplication(sys.argv)
    widget = InheritedWidget()
    widget.show()
    sys.exit(app.exec_())
