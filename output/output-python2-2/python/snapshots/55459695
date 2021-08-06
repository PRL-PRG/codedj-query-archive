# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'nuevafacturacionbase.ui'
#
# Created: Thu Feb 28 21:25:47 2008
#      by: PyQt4 UI code generator 4.1
#
# WARNING! All changes made in this file will be lost!

import sys
from PyQt4 import QtCore, QtGui

class Ui_NuevaFacturacionBase(object):
    def setupUi(self, NuevaFacturacionBase):
        NuevaFacturacionBase.setObjectName("NuevaFacturacionBase")
        NuevaFacturacionBase.resize(QtCore.QSize(QtCore.QRect(0,0,578,513).size()).expandedTo(NuevaFacturacionBase.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Policy(0),QtGui.QSizePolicy.Policy(0))
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(NuevaFacturacionBase.sizePolicy().hasHeightForWidth())
        NuevaFacturacionBase.setSizePolicy(sizePolicy)
        NuevaFacturacionBase.setMaximumSize(QtCore.QSize(578,608))
        NuevaFacturacionBase.setWindowIcon(QtGui.QIcon("../images/png/bulmafact.png"))

        self.gridlayout = QtGui.QGridLayout(NuevaFacturacionBase)
        self.gridlayout.setMargin(9)
        self.gridlayout.setSpacing(6)
        self.gridlayout.setObjectName("gridlayout")

        self.mui_botonera = QtGui.QDialogButtonBox(NuevaFacturacionBase)
        self.mui_botonera.setOrientation(QtCore.Qt.Horizontal)
        self.mui_botonera.setStandardButtons(QtGui.QDialogButtonBox.Cancel|QtGui.QDialogButtonBox.NoButton|QtGui.QDialogButtonBox.Ok)
        self.mui_botonera.setObjectName("mui_botonera")
        self.gridlayout.addWidget(self.mui_botonera,3,0,1,2)

        self.groupBox = QtGui.QGroupBox(NuevaFacturacionBase)
        self.groupBox.setObjectName("groupBox")

        self.mui_contratos = QtGui.QCheckBox(self.groupBox)
        self.mui_contratos.setGeometry(QtCore.QRect(30,30,161,19))
        self.mui_contratos.setObjectName("mui_contratos")
        self.gridlayout.addWidget(self.groupBox,2,0,1,2)

        self.mui_nomdb = QtGui.QLineEdit(NuevaFacturacionBase)
        self.mui_nomdb.setObjectName("mui_nomdb")
        self.gridlayout.addWidget(self.mui_nomdb,1,1,1,1)

        self.label_2 = QtGui.QLabel(NuevaFacturacionBase)
        self.label_2.setObjectName("label_2")
        self.gridlayout.addWidget(self.label_2,1,0,1,1)

        self.label = QtGui.QLabel(NuevaFacturacionBase)
        self.label.setObjectName("label")
        self.gridlayout.addWidget(self.label,0,0,1,1)

        self.mui_nomempresa = QtGui.QLineEdit(NuevaFacturacionBase)
        self.mui_nomempresa.setObjectName("mui_nomempresa")
        self.gridlayout.addWidget(self.mui_nomempresa,0,1,1,1)
        self.label_2.setBuddy(self.mui_nomdb)
        self.label.setBuddy(self.mui_nomempresa)

        self.retranslateUi(NuevaFacturacionBase)
        QtCore.QObject.connect(self.mui_botonera,QtCore.SIGNAL("accepted()"),NuevaFacturacionBase.accept)
        QtCore.QObject.connect(self.mui_botonera,QtCore.SIGNAL("rejected()"),NuevaFacturacionBase.reject)
        QtCore.QMetaObject.connectSlotsByName(NuevaFacturacionBase)

    def retranslateUi(self, NuevaFacturacionBase):
        NuevaFacturacionBase.setWindowTitle(QtGui.QApplication.translate("NuevaFacturacionBase", "Nueva Facturacion", None, QtGui.QApplication.UnicodeUTF8))
        self.groupBox.setTitle(QtGui.QApplication.translate("NuevaFacturacionBase", "Plugins", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_contratos.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Contratos de Servicios", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Nombre Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Nombre Empresa", None, QtGui.QApplication.UnicodeUTF8))

