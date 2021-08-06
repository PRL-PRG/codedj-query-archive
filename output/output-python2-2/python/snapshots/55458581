# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'modificarcontabilidadbase.ui'
#
# Created: Mon Mar 10 22:06:56 2008
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_ModificarContabilidadBase(object):
    def setupUi(self, ModificarContabilidadBase):
        ModificarContabilidadBase.setObjectName("ModificarContabilidadBase")
        ModificarContabilidadBase.resize(QtCore.QSize(QtCore.QRect(0,0,800,608).size()).expandedTo(ModificarContabilidadBase.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(ModificarContabilidadBase.sizePolicy().hasHeightForWidth())
        ModificarContabilidadBase.setSizePolicy(sizePolicy)
        ModificarContabilidadBase.setMaximumSize(QtCore.QSize(800,608))
        ModificarContabilidadBase.setWindowIcon(QtGui.QIcon("../../../../.designer/images/png/bulmafact.png"))

        self.gridlayout = QtGui.QGridLayout(ModificarContabilidadBase)
        self.gridlayout.setObjectName("gridlayout")

        self.label = QtGui.QLabel(ModificarContabilidadBase)
        self.label.setObjectName("label")
        self.gridlayout.addWidget(self.label,0,0,1,1)

        self.mui_nomempresa = QtGui.QLineEdit(ModificarContabilidadBase)
        self.mui_nomempresa.setObjectName("mui_nomempresa")
        self.gridlayout.addWidget(self.mui_nomempresa,0,1,1,2)

        self.label_2 = QtGui.QLabel(ModificarContabilidadBase)
        self.label_2.setObjectName("label_2")
        self.gridlayout.addWidget(self.label_2,0,3,1,1)

        self.mui_nomdb = QtGui.QLineEdit(ModificarContabilidadBase)
        self.mui_nomdb.setReadOnly(True)
        self.mui_nomdb.setObjectName("mui_nomdb")
        self.gridlayout.addWidget(self.mui_nomdb,0,4,1,2)

        self.label_3 = QtGui.QLabel(ModificarContabilidadBase)
        self.label_3.setObjectName("label_3")
        self.gridlayout.addWidget(self.label_3,1,0,1,2)

        self.mui_databaserevision = QtGui.QLineEdit(ModificarContabilidadBase)
        self.mui_databaserevision.setReadOnly(True)
        self.mui_databaserevision.setObjectName("mui_databaserevision")
        self.gridlayout.addWidget(self.mui_databaserevision,1,2,1,1)

        self.label_4 = QtGui.QLabel(ModificarContabilidadBase)
        self.label_4.setObjectName("label_4")
        self.gridlayout.addWidget(self.label_4,2,0,1,3)

        self.mui_plugins = QtGui.QTableWidget(ModificarContabilidadBase)
        self.mui_plugins.setAlternatingRowColors(True)
        self.mui_plugins.setObjectName("mui_plugins")
        self.gridlayout.addWidget(self.mui_plugins,3,0,1,6)

        self.mui_checkbox = QtGui.QCheckBox(ModificarContabilidadBase)
        self.mui_checkbox.setChecked(True)
        self.mui_checkbox.setObjectName("mui_checkbox")
        self.gridlayout.addWidget(self.mui_checkbox,4,0,1,3)

        self.mui_textBrowser = QtGui.QTextBrowser(ModificarContabilidadBase)
        self.mui_textBrowser.setObjectName("mui_textBrowser")
        self.gridlayout.addWidget(self.mui_textBrowser,5,0,1,6)

        self.mui_actualizardatabase = QtGui.QPushButton(ModificarContabilidadBase)
        self.mui_actualizardatabase.setObjectName("mui_actualizardatabase")
        self.gridlayout.addWidget(self.mui_actualizardatabase,6,0,1,2)

        self.mui_hacerbackup = QtGui.QPushButton(ModificarContabilidadBase)
        self.mui_hacerbackup.setObjectName("mui_hacerbackup")
        self.gridlayout.addWidget(self.mui_hacerbackup,6,2,1,1)

        self.mui_aceptar = QtGui.QPushButton(ModificarContabilidadBase)
        self.mui_aceptar.setObjectName("mui_aceptar")
        self.gridlayout.addWidget(self.mui_aceptar,6,3,1,2)

        self.mui_cancelar = QtGui.QPushButton(ModificarContabilidadBase)
        self.mui_cancelar.setObjectName("mui_cancelar")
        self.gridlayout.addWidget(self.mui_cancelar,6,5,1,1)
        self.label.setBuddy(self.mui_nomempresa)
        self.label_2.setBuddy(self.mui_nomdb)

        self.retranslateUi(ModificarContabilidadBase)
        QtCore.QObject.connect(self.mui_cancelar,QtCore.SIGNAL("released()"),ModificarContabilidadBase.reject)
        QtCore.QObject.connect(self.mui_checkbox,QtCore.SIGNAL("toggled(bool)"),self.mui_textBrowser.setVisible)
        QtCore.QMetaObject.connectSlotsByName(ModificarContabilidadBase)

    def retranslateUi(self, ModificarContabilidadBase):
        ModificarContabilidadBase.setWindowTitle(QtGui.QApplication.translate("ModificarContabilidadBase", "Modificar Contabilidad", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Nombre Empresa", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Nombre Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.label_3.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Version de la Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.label_4.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Plugins Instalados", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.clear()
        self.mui_plugins.setColumnCount(3)
        self.mui_plugins.setRowCount(0)

        headerItem = QtGui.QTableWidgetItem()
        headerItem.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Plugin", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(0,headerItem)

        headerItem1 = QtGui.QTableWidgetItem()
        headerItem1.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Descripcion", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(1,headerItem1)

        headerItem2 = QtGui.QTableWidgetItem()
        headerItem2.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Version Instalada", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_plugins.setHorizontalHeaderItem(2,headerItem2)
        self.mui_checkbox.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Ver Consola", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_actualizardatabase.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Actualizar Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_hacerbackup.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Hacer copia de Seguridad", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_aceptar.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Generar .conf", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_cancelar.setText(QtGui.QApplication.translate("ModificarContabilidadBase", "Cerrar", None, QtGui.QApplication.UnicodeUTF8))

