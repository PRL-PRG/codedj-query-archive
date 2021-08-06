#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Example of handmade UI widget
"""

from PyQt4 import QtCore, QtGui
    
class HandmadeWidget(QtGui.QWidget):
        
    def __init__(self, parent=None):
        super(HandmadeWidget, self).__init__(parent)
        self.setupUi()
        
    def setupUi(self):
        # располагаем под-виджеты внутри нашего виджета по таблице
        self.layout = QtGui.QGridLayout(self)
        # поскольку поля ввода однотипны, то и делаем их в цикле
        for pos, name in enumerate(('series', 'number')):
            # ярлык к полю ввода
            label = QtGui.QLabel(self)
            # само поле ввода
            edit = QtGui.QLineEdit(self)
            # поставить в соответствие ярлык <-> поле ввода
            label.setBuddy(edit)
            # текст ярлыка
            label.setText('&%s' % name.title())
            # размещаем ярлык и поле ввода по ячейкам нашей вирт.таблицы
            self.layout.addWidget(label, pos, 0, 1, 1)
            self.layout.addWidget(edit, pos, 1, 1, 1)
            # сохраняем ярлык и поле ввода как отдельные атрибуты виджета
            setattr(self, '%sLabel' % name, label)
            setattr(self, '%sEdit' % name, edit)
        # кнопка
        self.goButton = QtGui.QPushButton(self)
        self.goButton.setText("Go")
        # для выравнивая кнопки по правому краю
        # помещаем ее в горизонтальный бокс
        # вместе с "пружиной"-spacer
        hbox = QtGui.QHBoxLayout()
        spacer = QtGui.QSpacerItem(40, 20, 
                                   QtGui.QSizePolicy.Expanding, 
                                   QtGui.QSizePolicy.Minimum)
        hbox.addItem(spacer)
        hbox.addWidget(self.goButton)
        # добавляем в layout горизонтальный бокс
        self.layout.addLayout(hbox, pos+1, 0, 1, 2)

if __name__ == '__main__':
    import sys
    app = QtGui.QApplication(sys.argv)
    widget = HandmadeWidget()
    widget.show()
    sys.exit(app.exec_())

