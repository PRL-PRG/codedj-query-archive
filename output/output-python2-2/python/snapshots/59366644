# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'gui.ui'
#
# Created: Sat Oct  6 00:07:15 2007
#      by: PyQt4 UI code generator 4.1
#
# WARNING! All changes made in this file will be lost!

import sys
from PyQt4 import QtCore, QtGui

class Ui_DevClient(object):
    def setupUi(self, DevClient):
        DevClient.setObjectName("DevClient")
        DevClient.resize(QtCore.QSize(QtCore.QRect(0,0,800,600).size()).expandedTo(DevClient.minimumSizeHint()))

        self.centralwidget = QtGui.QWidget(DevClient)
        self.centralwidget.setObjectName("centralwidget")

        self.vboxlayout = QtGui.QVBoxLayout(self.centralwidget)
        self.vboxlayout.setMargin(9)
        self.vboxlayout.setSpacing(6)
        self.vboxlayout.setObjectName("vboxlayout")

        self.textOutput = QtGui.QTextEdit(self.centralwidget)
        self.textOutput.setObjectName("textOutput")
        self.vboxlayout.addWidget(self.textOutput)

        self.textInput = QtGui.QLineEdit(self.centralwidget)
        self.textInput.setObjectName("textInput")
        self.vboxlayout.addWidget(self.textInput)
        DevClient.setCentralWidget(self.centralwidget)

        self.toolBar = QtGui.QToolBar(DevClient)
        self.toolBar.setMovable(False)
        self.toolBar.setOrientation(QtCore.Qt.Horizontal)
        self.toolBar.setIconSize(QtCore.QSize(32,32))
        self.toolBar.setObjectName("toolBar")
        DevClient.addToolBar(self.toolBar)

        self.actionConnect = QtGui.QAction(DevClient)
        self.actionConnect.setIcon(QtGui.QIcon("../../images/connect.png"))
        self.actionConnect.setObjectName("actionConnect")

        self.actionExit = QtGui.QAction(DevClient)
        self.actionExit.setObjectName("actionExit")

        self.actionOption = QtGui.QAction(DevClient)
        self.actionOption.setIcon(QtGui.QIcon("../../images/option.png"))
        self.actionOption.setObjectName("actionOption")
        self.toolBar.addAction(self.actionConnect)
        self.toolBar.addAction(self.actionOption)

        self.retranslateUi(DevClient)
        QtCore.QMetaObject.connectSlotsByName(DevClient)

    def retranslateUi(self, DevClient):
        DevClient.setWindowTitle(QtGui.QApplication.translate("DevClient", "DevClient", None, QtGui.QApplication.UnicodeUTF8))
        self.actionConnect.setText(QtGui.QApplication.translate("DevClient", "Connetti", None, QtGui.QApplication.UnicodeUTF8))
        self.actionExit.setText(QtGui.QApplication.translate("DevClient", "Esci", None, QtGui.QApplication.UnicodeUTF8))
        self.actionOption.setText(QtGui.QApplication.translate("DevClient", "Opzioni", None, QtGui.QApplication.UnicodeUTF8))

