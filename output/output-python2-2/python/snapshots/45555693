# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'demo.ui'
#
# Created: Sun Sep 28 18:26:36 2008
#      by: PyQt4 UI code generator 4.3.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_Form(object):
    def setupUi(self, Form):
        Form.setObjectName("Form")
        Form.resize(QtCore.QSize(QtCore.QRect(0,0,353,101).size()).expandedTo(Form.minimumSizeHint()))

        self.gridlayout = QtGui.QGridLayout(Form)
        self.gridlayout.setObjectName("gridlayout")

        self.gridlayout1 = QtGui.QGridLayout()
        self.gridlayout1.setHorizontalSpacing(12)
        self.gridlayout1.setVerticalSpacing(25)
        self.gridlayout1.setObjectName("gridlayout1")

        self.label = QtGui.QLabel(Form)
        self.label.setObjectName("label")
        self.gridlayout1.addWidget(self.label,0,0,1,1)

        self.lineEdit = QtGui.QLineEdit(Form)
        self.lineEdit.setObjectName("lineEdit")
        self.gridlayout1.addWidget(self.lineEdit,0,1,1,1)

        self.label_2 = QtGui.QLabel(Form)
        self.label_2.setObjectName("label_2")
        self.gridlayout1.addWidget(self.label_2,1,0,1,1)

        self.errStateLineEdit = QLineEditWErrState(Form)
        self.errStateLineEdit.setReadOnly(True)
        self.errStateLineEdit.setObjectName("errStateLineEdit")
        self.gridlayout1.addWidget(self.errStateLineEdit,1,1,1,1)
        self.gridlayout.addLayout(self.gridlayout1,0,0,1,1)

        self.retranslateUi(Form)
        QtCore.QObject.connect(self.lineEdit,QtCore.SIGNAL("textChanged(QString)"),self.errStateLineEdit.setText)
        QtCore.QObject.connect(self.lineEdit,QtCore.SIGNAL("textChanged(QString)"),self.errStateLineEdit.setErrorState)
        QtCore.QMetaObject.connectSlotsByName(Form)

    def retranslateUi(self, Form):
        Form.setWindowTitle(QtGui.QApplication.translate("Form", "Form", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("Form", "Your input (regular QLineEdit)", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("Form", "Mirroring (custom QLineEditWErrState)", None, QtGui.QApplication.UnicodeUTF8))

from widget import QLineEditWErrState


if __name__ == "__main__":
    import sys
    app = QtGui.QApplication(sys.argv)
    Form = QtGui.QWidget()
    ui = Ui_Form()
    ui.setupUi(Form)
    Form.show()
    sys.exit(app.exec_())
