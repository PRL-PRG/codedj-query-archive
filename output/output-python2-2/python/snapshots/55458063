import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from modificarfacturacionbase import *
from plugins import PluginsBulmaSetup


class ModificarFacturacion(QtGui.QDialog, Ui_ModificarFacturacionBase, PluginsBulmaSetup):
    def __init__(self, database, parent = None):
        QtGui.QDialog.__init__(self,parent)
	PluginsBulmaSetup.__init__(self)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
#	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	self.mui_nomdb.setText(database)
	# Establecemos cual va a ser la base de datos con la que trabajaremos todo el rato
	self.database = database
	self.nombre = self.execQuery('SELECT valor FROM configuracion where nombre =\'NombreEmpresa\';').replace('\n', '')
	self.databaserevision = self.execQuery('SELECT valor FROM configuracion where nombre =\'DatabaseRevision\';').replace('\n', '')
	self.mui_nomempresa.setText(self.nombre.replace('\n', ''))
	self.mui_databaserevision.setText(self.databaserevision.replace('\n', ''))
	
	# Buscamos los Plugins
	self.buscaPlugins()
	# Ajustamos la presentacion
	self.mui_plugins.resizeColumnsToContents()
	self.mui_checkbox.setCheckState(Qt.Unchecked)

    def readOutput(self):
	self.mui_textBrowser.append(QString(self.process.readAllStandardOutput()))

    def readErrors(self):
	self.mui_textBrowser.append("<font color =\"#FF0000\">error: " + QString(self.process.readAllStandardError()) + "</font>")

    def finished(self):
	self.mui_textBrowser.append("<font color =\"#00FF00\">Done.</font>")

    def started(self):
	self.mui_textBrowser.append("<font color =\"#00FF00\">Start.</font>")

    def writecommand(self, comm):
	self.mui_textBrowser.append("<font color =\"#0000FF\">"+comm+"</font>")


    def on_mui_aceptar_released(self):
	self.mui_textBrowser.clear()

    def guardaQuery(self, query):
	self.query = query
	self.fileHandle = open ( '/tmp/query.sql', 'w' )
	self.fileHandle.write ( query )
	self.fileHandle.close()

    def execQuery(self, query):
	self.writecommand(QString("----") + self.database + QString("----"))
	self.subcomand = query
	self.guardaQuery(self.subcomand)
	self.command = 'su postgres -c \"psql -t -f /tmp/query.sql ' + self.database + '\"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	return QString(self.process.readAllStandardOutput())

    def on_mui_actualizardatabase_released(self):
	self.revisiones = ["revf-0.5.9.sql","revf-0.9.1.sql", "revf-0.9.3.sql", "revf-0.10.sql"]
	#Parcheamos todo lo que hay que parchear
	for self.parche in self.revisiones:
		self.command = 'su postgres -c \"psql -t -f  /usr/share/bulmages/dbmodels/actualizar/' + self.parche + ' ' + self.database  + '\"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.writecommand(self.process.readAllStandardOutput())
		self.actualizarPlugins()

    def on_mui_hacerbackup_released(self):
	self.writecommand("Backup")
	
	self.savefile = QFileDialog.getSaveFileName(self,  QString("Guardar  Elija archivo destino"), QString("/home"), QString("SQL (*.sql *.pgdump)") )
	self.command = 'su postgres -c \"pg_dump -f ' + self.savefile + ' ' + self.database  + '\"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	self.writecommand(self.process.readAllStandardOutput())

    def buscaPlugins1(self):
	self.plugins = self.execQuery('SELECT nombre, valor FROM configuracion WHERE nombre LIKE \'DBRev-%\'')
	self.writecommand(self.plugins)
	print self.plugins
	self.arrplugins = self.plugins.split(QString("\n"))
	self.mui_plugins.setRowCount(self.arrplugins.count() -3)
	self.i = 0
	while (self.i < self.arrplugins.count() ):
		self.writecommand(self.arrplugins[self.i])
		self.valores = self.arrplugins[self.i].split(QString("|"))
		if (self.valores.count() >= 2):
			self.mui_plugins.setItem(self.i-1 , 1 , QTableWidgetItem(self.valores[1].replace('\n', '')))
			self.mui_plugins.setItem(self.i-1 , 0 , QTableWidgetItem(self.valores[0].replace('\n', '')))

		self.i = self.i + 1
	
    def buscaPluginInstalado(self, plugin, libreria):
	self.version = self.execQuery('SELECT valor FROM configuracion WHERE nombre = \'' + plugin +'\'').replace('\n','').replace(' ','')
	if (self.version != ''):
		return self.version
	self.command = 'grep '+libreria+' /etc/bulmages/bulmafact_' + self.database + '.conf'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	self.version = self.process.readAllStandardOutput()
	if (self.version != ''):
		self.version = '0.11'
	return QString(self.version)
	
		
    def buscaPlugins(self):
	self.writecommand("Buscando Pluggins")

	self.mui_plugins.setRowCount(len(self.pluginsbulmafact))
	self.i = 0
	while (self.i < len(self.pluginsbulmafact)):
		self.versioninst = self.buscaPluginInstalado(self.pluginsbulmafact[self.i][3], self.pluginsbulmafact[self.i][1])
		self.check = QTableWidgetItem(QString(self.pluginsbulmafact[self.i][0]))
		self.check.setFlags(Qt.ItemIsUserCheckable | Qt.ItemIsEnabled)
		self.check.setCheckState(Qt.Unchecked)
		if (self.versioninst != ''):
			self.check.setCheckState(Qt.Checked)
		self.mui_plugins.setItem(self.i, 0, self.check)
		self.mui_plugins.setItem(self.i, 2, QTableWidgetItem(self.versioninst))
		self.mui_plugins.setItem(self.i , 1 , QTableWidgetItem(self.pluginsbulmafact[self.i][2]))
		self.mui_plugins.setRowHeight(self.i, 50)
		self.i = self.i + 1
	
	
    def actualizarPlugins(self):
	self.writecommand('ACTUALIZANDO PLUGINS')
	self.i = 0
	while (self.i < self.mui_plugins.rowCount()):
		self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
		if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
			self.writecommand('Ha que actualizar ' + self.pluginsbulmafact[self.i][0])
			self.command = 'su postgres -c \"psql -t -f  /usr/share/bulmages/dbmodels/actualizar/' + self.pluginsbulmafact[self.i][4] + '\"'
			self.writecommand(self.command)
			self.process.start(self.command)
			self.process.waitForFinished(-1)
			self.writecommand(self.process.readAllStandardOutput())
		self.i = self.i +1


    def on_mui_aceptar_released(self):
	self.writecommand('ESCRIBIENDO CONFIGURACION')
	self.writecommand("Escribiendo configuracion en /etc/bulmages")
	self.file = QFile("/etc/bulmages/bulmafact_" + self.database + ".conf");
	if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
		return;
	self.out = QTextStream(self.file)
	self.terminador = ""
	self.out << "CONF_PLUGINS_BULMAFACT   "
	
	
	self.i = 0
	while (self.i < self.mui_plugins.rowCount()):
		self.writecommand('Tratando ' + self.pluginsbulmafact[self.i][0])
		if (self.mui_plugins.item(self.i, 0).checkState() == Qt.Checked):
			self.writecommand('Ha que actualizar ' + self.pluginsbulmafact[self.i][0])
			self.out << self.terminador << self.pluginsbulmafact[self.i][1]
			self.terminador = "; \\\n";
		self.i = self.i +1
	self.out << "\n"
	self.file.close()

def main(args):
    app=QtGui.QApplication(args)
    win=ModificarFacturacion('bulmafact')
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
