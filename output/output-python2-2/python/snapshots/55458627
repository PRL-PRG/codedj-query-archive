# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'nuevacontabilidadbase.ui'
#
# Created: Mon Mar 10 20:31:12 2008
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_NuevaContabilidadBase(object):
    def setupUi(self, NuevaContabilidadBase):
        NuevaContabilidadBase.setObjectName("NuevaContabilidadBase")
        NuevaContabilidadBase.resize(QtCore.QSize(QtCore.QRect(0,0,800,559).size()).expandedTo(NuevaContabilidadBase.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(NuevaContabilidadBase.sizePolicy().hasHeightForWidth())
        NuevaContabilidadBase.setSizePolicy(sizePolicy)
        NuevaContabilidadBase.setMaximumSize(QtCore.QSize(800,608))
        NuevaContabilidadBase.setWindowIcon(QtGui.QIcon("../../../../.designer/images/png/bulmacont.png"))

        self.gridlayout = QtGui.QGridLayout(NuevaContabilidadBase)
        self.gridlayout.setObjectName("gridlayout")

        self.label = QtGui.QLabel(NuevaContabilidadBase)
        self.label.setObjectName("label")
        self.gridlayout.addWidget(self.label,0,0,1,1)

        self.mui_nomempresa = QtGui.QLineEdit(NuevaContabilidadBase)
        self.mui_nomempresa.setObjectName("mui_nomempresa")
        self.gridlayout.addWidget(self.mui_nomempresa,0,1,1,1)

        self.label_2 = QtGui.QLabel(NuevaContabilidadBase)
        self.label_2.setObjectName("label_2")
        self.gridlayout.addWidget(self.label_2,0,2,1,2)

        self.mui_nomdb = QtGui.QLineEdit(NuevaContabilidadBase)
        self.mui_nomdb.setObjectName("mui_nomdb")
        self.gridlayout.addWidget(self.mui_nomdb,0,4,1,1)

        self.label_4 = QtGui.QLabel(NuevaContabilidadBase)
        self.label_4.setObjectName("label_4")
        self.gridlayout.addWidget(self.label_4,1,0,1,5)

        self.mui_plugins = QtGui.QTableWidget(NuevaContabilidadBase)
        self.mui_plugins.setAlternatingRowColors(True)
        self.mui_plugins.setTextElideMode(QtCore.Qt.ElideNone)
        self.mui_plugins.setWordWrap(False)
        self.mui_plugins.setObjectName("mui_plugins")
        self.gridlayout.addWidget(self.mui_plugins,2,0,1,5)

        self.mui_checkbox = QtGui.QCheckBox(NuevaContabilidadBase)
        self.mui_checkbox.setChecked(True)
        self.mui_checkbox.setObjectName("mui_checkbox")
        self.gridlayout.addWidget(self.mui_checkbox,3,0,1,5)

        self.mui_textBrowser = QtGui.QTextBrowser(NuevaContabilidadBase)
        self.mui_textBrowser.setFocusPolicy(QtCore.Qt.NoFocus)
        self.mui_textBrowser.setObjectName("mui_textBrowser")
        self.gridlayout.addWidget(self.mui_textBrowser,4,0,1,5)

        self.mui_aceptar = QtGui.QPushButton(NuevaContabilidadBase)
        self.mui_aceptar.setObjectName("mui_aceptar")
        self.gridlayout.addWidget(self.mui_aceptar,5,2,1,1)

        self.mui_cancelar = QtGui.QPushButton(NuevaContabilidadBase)
        self.mui_cancelar.setObjectName("mui_cancelar")
        self.gridlayout.addWidget(self.mui_cancelar,5,3,1,2)
        self.label.setBuddy(self.mui_nomempresa)
        self.label_2.setBuddy(self.mui_nomdb)

        self.retranslateUi(NuevaContabilidadBase)
        QtCore.QObject.connect(self.mui_cancelar,QtCore.SIGNAL("released()"),NuevaContabilidadBase.reject)
        QtCore.QObject.connect(self.mui_checkbox,QtCore.SIGNAL("toggled(bool)"),self.mui_textBrowser.setVisible)
        QtCore.QMetaObject.connectSlotsByName(NuevaContabilidadBase)
        NuevaContabilidadBase.setTabOrder(self.mui_nomempresa,self.mui_nomdb)
        NuevaContabilidadBase.setTabOrder(self.mui_nomdb,self.mui_aceptar)
        NuevaContabilidadBase.setTabOrder(self.mui_aceptar,self.mui_cancelar)
        NuevaContabilidadBase.setTabOrder(self.mui_cancelar,self.mui_textBrowser)

    def retranslateUi(self, NuevaContabilidadBase):
        NuevaContabilidadBase.setWindowTitle(QtGui.QApplication.translate("NuevaContabilidadBase", "Nueva Contabilidad", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setWhatsThis(QtGui.QApplication.translate("NuevaContabilidadBase", "Nombre Fiscal de la Empresa", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Nombre Empresa", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setWhatsThis(QtGui.QApplication.translate("NuevaContabilidadBase", "Nombre de la Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Nombre Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.label_4.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Plugins Displonibles", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.clear()
        self.mui_plugins.setColumnCount(2)
        self.mui_plugins.setRowCount(0)

        headerItem = QtGui.QTableWidgetItem()
        headerItem.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Plugin", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(0,headerItem)

        headerItem1 = QtGui.QTableWidgetItem()
        headerItem1.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Descripcion", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(1,headerItem1)
        self.mui_checkbox.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Ver Consola", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_aceptar.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Generar", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_cancelar.setText(QtGui.QApplication.translate("NuevaContabilidadBase", "Cerrar", None, QtGui.QApplication.UnicodeUTF8))

