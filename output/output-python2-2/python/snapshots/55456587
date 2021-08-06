# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'nuevafacturacionbase.ui'
#
# Created: Sun Apr 20 17:58:29 2008
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_NuevaFacturacionBase(object):
    def setupUi(self, NuevaFacturacionBase):
        NuevaFacturacionBase.setObjectName("NuevaFacturacionBase")
        NuevaFacturacionBase.resize(QtCore.QSize(QtCore.QRect(0,0,800,608).size()).expandedTo(NuevaFacturacionBase.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(NuevaFacturacionBase.sizePolicy().hasHeightForWidth())
        NuevaFacturacionBase.setSizePolicy(sizePolicy)
        NuevaFacturacionBase.setMaximumSize(QtCore.QSize(800,608))
        NuevaFacturacionBase.setWindowIcon(QtGui.QIcon("../images/png/bulmafact.png"))

        self.gridlayout = QtGui.QGridLayout(NuevaFacturacionBase)
        self.gridlayout.setMargin(5)
        self.gridlayout.setObjectName("gridlayout")

        self.label = QtGui.QLabel(NuevaFacturacionBase)
        self.label.setObjectName("label")
        self.gridlayout.addWidget(self.label,0,0,1,1)

        self.mui_nomempresa = QtGui.QLineEdit(NuevaFacturacionBase)
        self.mui_nomempresa.setObjectName("mui_nomempresa")
        self.gridlayout.addWidget(self.mui_nomempresa,0,1,1,1)

        self.label_2 = QtGui.QLabel(NuevaFacturacionBase)
        self.label_2.setObjectName("label_2")
        self.gridlayout.addWidget(self.label_2,0,2,1,1)

        self.mui_nomdb = QtGui.QLineEdit(NuevaFacturacionBase)
        self.mui_nomdb.setObjectName("mui_nomdb")
        self.gridlayout.addWidget(self.mui_nomdb,0,3,1,2)

        self.mui_soporteTPV = QtGui.QCheckBox(NuevaFacturacionBase)
        self.mui_soporteTPV.setChecked(False)
        self.mui_soporteTPV.setObjectName("mui_soporteTPV")
        self.gridlayout.addWidget(self.mui_soporteTPV,1,0,1,2)

        self.label_4 = QtGui.QLabel(NuevaFacturacionBase)
        self.label_4.setObjectName("label_4")
        self.gridlayout.addWidget(self.label_4,2,0,1,2)

        self.tabWidget = QtGui.QTabWidget(NuevaFacturacionBase)
        self.tabWidget.setObjectName("tabWidget")

        self.tab = QtGui.QWidget()
        self.tab.setObjectName("tab")

        self.vboxlayout = QtGui.QVBoxLayout(self.tab)
        self.vboxlayout.setObjectName("vboxlayout")

        self.mui_plugins = QtGui.QTableWidget(self.tab)
        self.mui_plugins.setAlternatingRowColors(True)
        self.mui_plugins.setTextElideMode(QtCore.Qt.ElideNone)
        self.mui_plugins.setWordWrap(False)
        self.mui_plugins.setObjectName("mui_plugins")
        self.vboxlayout.addWidget(self.mui_plugins)
        self.tabWidget.addTab(self.tab,"")

        self.tab_2 = QtGui.QWidget()
        self.tab_2.setObjectName("tab_2")

        self.vboxlayout1 = QtGui.QVBoxLayout(self.tab_2)
        self.vboxlayout1.setObjectName("vboxlayout1")

        self.mui_plugins1 = QtGui.QTableWidget(self.tab_2)
        self.mui_plugins1.setAlternatingRowColors(True)
        self.mui_plugins1.setTextElideMode(QtCore.Qt.ElideNone)
        self.mui_plugins1.setWordWrap(False)
        self.mui_plugins1.setObjectName("mui_plugins1")
        self.vboxlayout1.addWidget(self.mui_plugins1)
        self.tabWidget.addTab(self.tab_2,"")
        self.gridlayout.addWidget(self.tabWidget,3,0,1,5)

        self.mui_checkbox = QtGui.QCheckBox(NuevaFacturacionBase)
        self.mui_checkbox.setChecked(True)
        self.mui_checkbox.setObjectName("mui_checkbox")
        self.gridlayout.addWidget(self.mui_checkbox,4,0,1,2)

        self.mui_textBrowser = QtGui.QTextBrowser(NuevaFacturacionBase)
        self.mui_textBrowser.setObjectName("mui_textBrowser")
        self.gridlayout.addWidget(self.mui_textBrowser,5,0,1,5)

        self.mui_aceptar = QtGui.QPushButton(NuevaFacturacionBase)
        self.mui_aceptar.setObjectName("mui_aceptar")
        self.gridlayout.addWidget(self.mui_aceptar,6,2,1,2)

        self.mui_cancelar = QtGui.QPushButton(NuevaFacturacionBase)
        self.mui_cancelar.setObjectName("mui_cancelar")
        self.gridlayout.addWidget(self.mui_cancelar,6,4,1,1)
        self.label.setBuddy(self.mui_nomempresa)
        self.label_2.setBuddy(self.mui_nomdb)

        self.retranslateUi(NuevaFacturacionBase)
        self.tabWidget.setCurrentIndex(0)
        QtCore.QObject.connect(self.mui_cancelar,QtCore.SIGNAL("released()"),NuevaFacturacionBase.reject)
        QtCore.QObject.connect(self.mui_checkbox,QtCore.SIGNAL("toggled(bool)"),self.mui_textBrowser.setVisible)
        QtCore.QMetaObject.connectSlotsByName(NuevaFacturacionBase)

    def retranslateUi(self, NuevaFacturacionBase):
        NuevaFacturacionBase.setWindowTitle(QtGui.QApplication.translate("NuevaFacturacionBase", "Nueva Facturacion", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Nombre Empresa", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Nombre Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_soporteTPV.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Soporte para TPV", None, QtGui.QApplication.UnicodeUTF8))
        self.label_4.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Plugins Displonibles", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.clear()
        self.mui_plugins.setColumnCount(2)
        self.mui_plugins.setRowCount(0)

        headerItem = QtGui.QTableWidgetItem()
        headerItem.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Plugin", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(0,headerItem)

        headerItem1 = QtGui.QTableWidgetItem()
        headerItem1.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Descripcion", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(1,headerItem1)
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab), QtGui.QApplication.translate("NuevaFacturacionBase", "BulmaFact", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins1.clear()
        self.mui_plugins1.setColumnCount(2)
        self.mui_plugins1.setRowCount(0)

        headerItem2 = QtGui.QTableWidgetItem()
        headerItem2.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Plugin", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins1.setHorizontalHeaderItem(0,headerItem2)

        headerItem3 = QtGui.QTableWidgetItem()
        headerItem3.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Descripcion", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins1.setHorizontalHeaderItem(1,headerItem3)
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_2), QtGui.QApplication.translate("NuevaFacturacionBase", "BulmaTPV", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_checkbox.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Ver Consola", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_aceptar.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Generar", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_cancelar.setText(QtGui.QApplication.translate("NuevaFacturacionBase", "Cerrar", None, QtGui.QApplication.UnicodeUTF8))

