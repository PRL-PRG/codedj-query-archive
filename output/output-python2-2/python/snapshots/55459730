# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'nuevousuariobase.ui'
#
# Created: Tue Feb 26 11:10:57 2008
#      by: PyQt4 UI code generator 4.1
#
# WARNING! All changes made in this file will be lost!

import sys
from PyQt4 import QtCore, QtGui

class Ui_NuevoUsuario(object):
    def setupUi(self, NuevoUsuario):
        NuevoUsuario.setObjectName("NuevoUsuario")
        NuevoUsuario.resize(QtCore.QSize(QtCore.QRect(0,0,355,118).size()).expandedTo(NuevoUsuario.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Policy(0),QtGui.QSizePolicy.Policy(0))
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(NuevoUsuario.sizePolicy().hasHeightForWidth())
        NuevoUsuario.setSizePolicy(sizePolicy)
        NuevoUsuario.setMaximumSize(QtCore.QSize(355,118))

        self.gridlayout = QtGui.QGridLayout(NuevoUsuario)
        self.gridlayout.setMargin(9)
        self.gridlayout.setSpacing(6)
        self.gridlayout.setObjectName("gridlayout")

        self.mui_botonera = QtGui.QDialogButtonBox(NuevoUsuario)
        self.mui_botonera.setOrientation(QtCore.Qt.Horizontal)
        self.mui_botonera.setStandardButtons(QtGui.QDialogButtonBox.Cancel|QtGui.QDialogButtonBox.NoButton|QtGui.QDialogButtonBox.Ok)
        self.mui_botonera.setObjectName("mui_botonera")
        self.gridlayout.addWidget(self.mui_botonera,2,0,1,2)

        self.mui_password = QtGui.QLineEdit(NuevoUsuario)
        self.mui_password.setEchoMode(QtGui.QLineEdit.Password)
        self.mui_password.setObjectName("mui_password")
        self.gridlayout.addWidget(self.mui_password,1,1,1,1)

        self.label_2 = QtGui.QLabel(NuevoUsuario)
        self.label_2.setObjectName("label_2")
        self.gridlayout.addWidget(self.label_2,1,0,1,1)

        self.mui_nombre = QtGui.QLineEdit(NuevoUsuario)
        self.mui_nombre.setObjectName("mui_nombre")
        self.gridlayout.addWidget(self.mui_nombre,0,1,1,1)

        self.label = QtGui.QLabel(NuevoUsuario)
        self.label.setObjectName("label")
        self.gridlayout.addWidget(self.label,0,0,1,1)
        self.label_2.setBuddy(self.mui_password)
        self.label.setBuddy(self.mui_nombre)

        self.retranslateUi(NuevoUsuario)
        QtCore.QObject.connect(self.mui_botonera,QtCore.SIGNAL("accepted()"),NuevoUsuario.accept)
        QtCore.QObject.connect(self.mui_botonera,QtCore.SIGNAL("rejected()"),NuevoUsuario.reject)
        QtCore.QMetaObject.connectSlotsByName(NuevoUsuario)

    def retranslateUi(self, NuevoUsuario):
        NuevoUsuario.setWindowTitle(QtGui.QApplication.translate("NuevoUsuario", "Dialog", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("NuevoUsuario", "Password", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("NuevoUsuario", "Nombre", None, QtGui.QApplication.UnicodeUTF8))

