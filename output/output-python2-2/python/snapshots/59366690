# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'gui_qt.ui'
#
# Created: Wed Sep 19 22:41:24 2007
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_DevClient(object):
    def setupUi(self, DevClient):
        DevClient.setObjectName("DevClient")
        DevClient.resize(QtCore.QSize(QtCore.QRect(0,0,800,680).size()).expandedTo(DevClient.minimumSizeHint()))

        self.centralwidget = QtGui.QWidget(DevClient)
        self.centralwidget.setObjectName("centralwidget")

        self.widget = QtGui.QWidget(self.centralwidget)
        self.widget.setGeometry(QtCore.QRect(10,10,771,631))
        self.widget.setObjectName("widget")

        self.vboxlayout = QtGui.QVBoxLayout(self.widget)
        self.vboxlayout.setMargin(0)
        self.vboxlayout.setSpacing(6)
        self.vboxlayout.setObjectName("vboxlayout")

        self.textOutput = QtGui.QTextBrowser(self.widget)
        self.textOutput.setObjectName("textOutput")
        self.vboxlayout.addWidget(self.textOutput)

        self.textInput = QtGui.QLineEdit(self.widget)
        self.textInput.setObjectName("textInput")
        self.vboxlayout.addWidget(self.textInput)
        DevClient.setCentralWidget(self.centralwidget)

        self.menubar = QtGui.QMenuBar(DevClient)
        self.menubar.setGeometry(QtCore.QRect(0,0,800,29))
        self.menubar.setObjectName("menubar")

        self.menuClient = QtGui.QMenu(self.menubar)
        self.menuClient.setObjectName("menuClient")
        DevClient.setMenuBar(self.menubar)

        self.actionConnect = QtGui.QAction(DevClient)
        self.actionConnect.setObjectName("actionConnect")

        self.actionDisconnect = QtGui.QAction(DevClient)
        self.actionDisconnect.setObjectName("actionDisconnect")

        self.actionExit = QtGui.QAction(DevClient)
        self.actionExit.setObjectName("actionExit")
        self.menuClient.addAction(self.actionConnect)
        self.menuClient.addAction(self.actionDisconnect)
        self.menuClient.addSeparator()
        self.menuClient.addAction(self.actionExit)
        self.menubar.addAction(self.menuClient.menuAction())

        self.retranslateUi(DevClient)
        QtCore.QObject.connect(self.actionExit,QtCore.SIGNAL("activated()"),DevClient.close)
        QtCore.QMetaObject.connectSlotsByName(DevClient)

    def retranslateUi(self, DevClient):
        DevClient.setWindowTitle(QtGui.QApplication.translate("DevClient", "DevClient", None, QtGui.QApplication.UnicodeUTF8))
        self.menuClient.setTitle(QtGui.QApplication.translate("DevClient", "Client", None, QtGui.QApplication.UnicodeUTF8))
        self.actionConnect.setText(QtGui.QApplication.translate("DevClient", "Connetti", None, QtGui.QApplication.UnicodeUTF8))
        self.actionDisconnect.setText(QtGui.QApplication.translate("DevClient", "Disconnetti", None, QtGui.QApplication.UnicodeUTF8))
        self.actionExit.setText(QtGui.QApplication.translate("DevClient", "Esci", None, QtGui.QApplication.UnicodeUTF8))

