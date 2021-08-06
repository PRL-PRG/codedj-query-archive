#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Example of handmade UI widget
"""

from PyQt4 import QtCore, QtGui

from designed_ui import Ui_DesignedWidget

class WidgetWithUiAsAttr(QtGui.QWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        self.ui = Ui_DesignedWidget()
        self.ui.setupUi(self)

if __name__ == '__main__':
    import sys
    app = QtGui.QApplication(sys.argv)
    widget = WidgetWithUiAsAttr()
    widget.show()
    sys.exit(app.exec_())
