# encoding: utf-8
from PyQt4 import QtGui, QtDesigner
from widget import QLineEditWErrState

class QLineEditWErrStatePlugin(QtDesigner.QPyDesignerCustomWidgetPlugin):
    """
    QLineEditWErrStatePlugin(QtDesigner.QPyDesignerCustomWidgetPlugin)
    
    Provides a Python custom plugin for Qt Designer by implementing the
    QDesignerCustomWidgetPlugin via a PyQt-specific custom plugin class.
    """
    
    def __init__(self, parent=None):
        QtDesigner.QPyDesignerCustomWidgetPlugin.__init__(self, parent)
        self.initialized = True

    def createWidget(self, parent):
        # метод должен вернуть экземпляр класса нашего виджета
        # вот тут и пригодилось согласование с принятым в Qt4 API
        return QLineEditWErrState(parent)

    def name(self):
        # этод метод должен вернуть имя класса виджета
        return "QLineEditWErrState"

    def group(self):
        # имя группы виджета
        return "PyQt custom widgets"
    
    def icon(self):
        # иконка виджета
        return QtGui.QIcon()

    def toolTip(self):
        # всплывающая подсказка
        return "QLineEdit with error state"

    def whatsThis(self):
        # краткое описание
        return "Custom widget QLineEditWErrState - QLineEdit with error state"

    def isContainer(self):
        # True, если виджет может служить контейнером других виджетов,
        # при этом требуется реализация QDesignerContainerExtension
        # False в противном случае
        return False

    def domXml(self):
        # должен вернуть XML-описание виджета и параметры его свойств.
        # минимально -- класс и имя экземпляра класса
        # вставляется в .ui
        return '<widget class="QLineEditWErrState" name=\"errStateLineEdit\" />\n'

    def includeFile(self):
        # возвращает имя модуля, в котором хранится наш виджет
        # вставляется как `import <includeFile>` в генеренном из .ui Python-коде
        return "widget"
