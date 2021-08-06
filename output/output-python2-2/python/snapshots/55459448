import sys
import os
from PyQt4.QtGui import *
from PyQt4.QtCore import *
from nuevafacturacionbase import *



class NuevaFacturacion(QtGui.QDialog, Ui_NuevaFacturacionBase):
    def __init__(self, parent = None):
        QtGui.QDialog.__init__(self,parent)
	self.setupUi(self)
	
	self.process = QtCore.QProcess()
	self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self.readOutput)
	self.connect(self.process, SIGNAL("readyReadStandardError()"), self.readErrors)
	self.connect(self.process, SIGNAL("finished()"), self.finished)
	self.connect(self.process, SIGNAL("started()"), self.started)
	
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
	
	self.hayplugins = 0
	self.nomdb = self.mui_nomdb.text()
	
	# Creamos la base de datos
	self.command = 'su postgres -c "createdb -E UNICODE ' + self.nomdb +'"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)
	
	# Cargamos la esquematica de la base de datos
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_schema.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cargamos los datos minimos
	self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/crear/bulmafact/bulmafact_data.sql"'
	self.writecommand(self.command)
	self.process.start(self.command)
	self.process.waitForFinished(-1)

	# Cambiamos el nombre de la empresa
	self.nomempresa = self.mui_nomempresa.text()
	self.subcomand = 'UPDATE configuracion set valor=\'\"\'' +self.nomempresa +'\'\"\' WHERE nombre = \'\"\'NombreEmpresa\'\"\';'
	self.command = 'su postgres -c \'psql ' + self.nomdb + ' -c \"' +self.subcomand+ '\"\''
	self.writecommand(self.command)
        os.system(self.command.toAscii().data())

	# Si esta seleccionado el checkbox de contratos cargamos el parche de contratos
	if (self.mui_contratos.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-contratos.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1
	
	if (self.mui_plugindocked.isChecked()):
		self.hayplugins = 1
	
	if (self.mui_pluginimpers.isChecked()):
		self.hayplugins = 1
		
	if (self.mui_pluginmail.isChecked()):
		self.hayplugins = 1

	if (self.mui_comercialbf.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-comercialbf.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_plugincatalogo.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginpreciocoste.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-pluginpreciocoste.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_pluginalmacen.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugincuadrante.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-plugincuadrante.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_pluginpromedios.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginasterisk.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-pluginasterisk.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_pluginimpresionesmultiples.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginq19.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginbarcodeopen.isChecked()):
		self.hayplugins = 1
		
	if (self.mui_plugininformeclientes.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugintarifas.isChecked()):
		self.hayplugins = 1

#	if (self.mui_pluginivainc.isChecked()):
#		self.hayplugins = 1

	if (self.mui_pluginticket.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginvehiculosbf.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-pluginvehiculosbf.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_pluginclipboardbf.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugininventario.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-plugininventario.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_plugindebugbf.isChecked()):
		self.hayplugins = 1

	if (self.mui_pluginsubformsxc.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugintrazabilidad.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-plugintrazabilidad.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_plugintipostrabajo.isChecked()):
		self.command = 'su postgres -c "psql ' + self.nomdb + ' < /usr/share/bulmages/dbmodels/plugins/revf-plugintipostrabajo.sql"'
		self.writecommand(self.command)
		self.process.start(self.command)
		self.process.waitForFinished(-1)
		self.hayplugins = 1

	if (self.mui_pluginsubformods.isChecked()):
		self.hayplugins = 1

	if (self.mui_plugintpv.isChecked()):
		self.hayplugins = 1

	# Si hay plugins seleccionados escribimos la configuracion para esta empresa
	if (self.hayplugins == 1):
		self.writecommand("Escribiendo configuracion en /etc/bulmages")
		self.file = QFile("/etc/bulmages/bulmafact_" + self.nomdb + ".conf");
		if not(self.file.open(QIODevice.WriteOnly | QIODevice.Text)):
			return;
		self.out = QTextStream(self.file)
		self.terminador = ""
		self.out << "CONF_PLUGINS_BULMAFACT   "
		
		# Ponemos la configuracion del plugin docked
		if (self.mui_plugindocked.isChecked()):
			self.out << self.terminador << "libplugindocked.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin impresiones personalizadas
		if (self.mui_pluginimpers.isChecked()):
			self.out << self.terminador << "libpluginimpers.so"
			self.terminador = "; \\\n";
			
		# Ponemos la configuracion del plugin de e-mail
		if (self.mui_pluginmail.isChecked()):
			self.out << self.terminador << "libpluginmail.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion de comerciales
		if (self.mui_comercialbf.isChecked()):
			self.out << self.terminador << "libcomercialbf.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de catalogo
		if (self.mui_plugincatalogo.isChecked()):
			self.out << self.terminador << "libplugincatalogo.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de precios de coste
		if (self.mui_pluginpreciocoste.isChecked()):
			self.out << self.terminador << "libpluginpreciocoste.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de almacen
		if (self.mui_pluginalmacen.isChecked()):
			self.out << self.terminador << "libpluginalmacen.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de cuadrante
		if (self.mui_plugincuadrante.isChecked()):
			self.out << self.terminador << "libplugincuadrante.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de cuadrante
		if (self.mui_pluginpromedios.isChecked()):
			self.out << self.terminador << "libpluginpromedios.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de asterisk
		if (self.mui_pluginasterisk.isChecked()):
			self.out << self.terminador << "libpluginasterisk.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de impresiones multiples
		if (self.mui_pluginimpresionesmultiples.isChecked()):
			self.out << self.terminador << "libpluginimpresionesmultiples.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de Q19
		if (self.mui_pluginq19.isChecked()):
			self.out << self.terminador << "libpluginq19.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de barcode
		if (self.mui_pluginbarcodeopen.isChecked()):
			self.out << self.terminador << "libpluginbarcodeopen.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de informe de clientes
		if (self.mui_plugininformeclientes.isChecked()):
			self.out << self.terminador << "libplugininformeclientes.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de tarifas
		if (self.mui_plugintarifas.isChecked()):
			self.out << self.terminador << "libplugintarifas.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion de contratos
		if (self.mui_contratos.isChecked()):
			self.out << self.terminador << "libplugincontratos.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion de iva incluido
#		if (self.mui_pluginivainc.isChecked()):
#			self.out << self.terminador << "libpluginivainc.so"
#			self.terminador = "; \\\n";

		# Ponemos la configuracion de tickets
		if (self.mui_pluginticket.isChecked()):
			self.out << self.terminador << "libpluginticket.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de llamadas
		if (self.mui_llamadas.isChecked()):
			self.out << self.terminador << "libpluginllamadas.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de vehiculos
		if (self.mui_pluginvehiculosbf.isChecked()):
			self.out << self.terminador << "libpluginvehiculosbf.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de clipboard
		if (self.mui_pluginclipboardbf.isChecked()):
			self.out << self.terminador << "libpluginclipboardbf.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de inventario
		if (self.mui_plugininventario.isChecked()):
			self.out << self.terminador << "libplugininventario.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de debug
		if (self.mui_plugindebugbf.isChecked()):
			self.out << self.terminador << "libplugindebugbf.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de subformularios
		if (self.mui_pluginsubformsxc.isChecked()):
			self.out << self.terminador << "libpluginsubformsxc.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de trazabilidad
		if (self.mui_plugintrazabilidad.isChecked()):
			self.out << self.terminador << "libplugintrazabilidad.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin de tipos de trabajo
		if (self.mui_plugintipostrabajo.isChecked()):
			self.out << self.terminador << "libplugintipostrabajo.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin subformods
		if (self.mui_pluginsubformods.isChecked()):
			self.out << self.terminador << "libpluginsubformods.so"
			self.terminador = "; \\\n";

		# Ponemos la configuracion del plugin tpv
		if (self.mui_plugintpv.isChecked()):
			self.out << self.terminador << "libplugintpv.so"
			self.terminador = "; \\\n";

		self.out << "\n"
		self.file.close()
	self.mui_textBrowser.append("Done.")




def main(args):
    app=QtGui.QApplication(args)
    win=NuevaFacturacion()
    win.exec_()
    sys.exit(app.exec_())

if __name__=="__main__":
    main(sys.argv)
