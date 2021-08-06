# -*- coding: utf-8 -*-

import sys
import os

global pathdbbulmafact
global pathdbbulmacont
global pathdbparches
global configfiles


class PluginsBulmaSetup:
    def __init__(self):
	self.pluginsbulmafact = [
	['Comerciales','libcomercialbf.so','Tratamiento de Comerciales','DBRev-ComercialBF','revf-comercialbf.sql'],
	['E-Mail','libpluginmail.so','Envio de documentos \n Por correo electrónico \n Utiliza kmail.','',''],
	['Contratos','libplugincontratos.so','Gestiona los contratos \n de servicios con clientes','DBRev-Contratos','revf-plugincontratos.sql'],
	['Imprmir tickets','libpluginticket.so','Impresion de Tickets \n con impresoras ESC / POS','',''],
#	['Factura Electronica','libefacturabf.so','Emision de Factura Electronica','',''],
	['Domiciliaciones Bancarias','libpluginnq19.so','Permite controlar los archivos de domiciliaciones enviados al banco','DBRev-PluginnQ19','revf-pluginnq19.sql'],
	['Corrector de la Factuarcion','libplugincorrectorbf.so','Efectua tests de verificación sobre la facturacion','',''],
	['Tipos de Trabajo','libplugintipostrabajo.so','Tipificar los Trabajadores segun \n su actividad','',''],
	['Impresiones Multiples','libpluginimpresionesmultiples.so','Permite operar con multiples documentos \n previamente seleccionados','',''],
	['libpluginpreciocoste.so','libpluginpreciocoste.so','libpluginpreciocoste.so','libpluginpreciocoste.so','libpluginpreciocoste.so'],
	['Cuadrantes','libplugincuadrante.so','Permite llevar un cuadrante de trabajadores','DBRev-Cuadrante','revf-plugincuadrante.sql'],
	['TPV','libplugintpv.so','Permite ver en BulmaFact \n Datos de los Puntos de Venta','',''],
	['Informe de Clientes','libplugininformeclientes.so','Generacion de un informe detallado de clientes','',''],
	['Almacenes Extendidos','libpluginalmacen.so','Amplia la Gestion de Almacenes \n Permite mayor numero de datos','',''],
	['Promedios','libpluginpromedios.so','Muestra los promedios \n de ventas y compras por \n articulo','',''],
	['Modo Debug','libplugindebugbf.so','Habilita el modo de depuracion \n en tiempo de ejecucion','',''],
	['Trazabilidad','libplugintrazabilidad.so','Habilita el modo de trazabilidad de \n productos','',''],
#	['Informes SXC','libplugininformeclientessxc.so','Permite la generacion \n de informes en formato \n SXC','',''],
	['Validaciones con Asterisk','libpluginasterisk.so','Realiza validaciones de usuarios con centralita telefonica Asterisk','DBRev-ValAsterisk','revf-pluginasterisk.sql'],
	['Domiciliaciones Bancarias','libpluginq19.so','Permite exportar facturas \n al formato Q19','',''],
	['Etiquetas','libpluginetiquetado.so','Genera Codigos de Barras \n de los articulos','',''],
#	['Vehiculos','libpluginvehiculosbf.so','Permite controlar la Flota de Vehiculos','',''],
	['Inventarios','libplugininventario.so','Permite llevar un control de inventarios realizados','DBRev-Inventario','revf-plugininventario.sql'],
	['Tarifas','libplugintarifas.so','Tratamiento de las Tarifas','',''],
	['Apertura rapida','libpluginbarcodeopen.so','Permite abrir documentos \n con codigo de barras a traves del \n scaner','',''],
	['Etiquetas','libpluginetiquetas.so','Imprime etiquetas de los productos','',''],
	['Llamadas','libpluginllamadas.so','Permite llevar un registro de llamadas de clientes','DBRev-PluginLlamadas','revf-pluginllamadas.sql'],
	['Catalogo de Productos','libplugincatalogo.so','Permite Generar \n un catalogo de productos','',''],
#	['Tallas y Colores','libplugin_tc_articulos.so','Agrega la funcionalidad de Tallas y Colores','',''],
	['Facturacion Automatica','libpluginfacturar.so','Facturar de forma automatica \n a partir de los albaranes','',''],
# PLUGINS DE BULMALIB
	['Pegado desde Clipboard', 'libpluginclipboardbf.so', 'Copiando un area desde una hoja de calculo. Esta se puede insertar en un subformulario de BulmaG&eacute;s','',''],
	['Impresiones Personalizadas', 'libpluginimpers.so', 'Para tener multiples modelos de impresion para cada uno de los documentos.\n Genere tantas plantillas RML como desee para una factura, presupuesto u otros y utilicela cuando la precise. Funciona como un menu contextaual en las fichas que muestra todas las plantillas que contengan en su nombre el nombre de la entidad a la que la ficha representa.','',''],
	['Pegado desde Clipboard', 'libpluginclipboardbf.so', 'Copiando un area desde una hoja de calculo. Esta se puede\n insertar en un subformulario de BulmaGes','',''],
	['Exportar subforms a SXC', 'libpluginsubformsxc.so', 'Permite, mediante un menu contextual exportar cualquier\n subformulario a formato SXC y abrirlo con KSpread o OpenOffice','',''],
	['Embeber aplicaciones', 'libplugindocked.so', 'Embeba cualquier aplicacion X en el entorno de BulmaFact para\n tener las aplicaciones mas a mano','',''],
	['Exportar subforms a ODS', 'libpluginsubformods.so', 'Permite, mediante un menu contextual exportar cualquier\n subformulario a formato SXC y abrirlo con KSpread o OpenOffice','','']

	]


	self.pluginsbulmacont = [
	['Registro de IVA','libpluginregistroiva.so','Registro de Facturas y IVAS','',''],
	['Corrector Contable','libplugincorrector.so','Realiza tests sobre la contabilidad. \n Avisa de los posibles errores encontrados.','',''],
	['Modo Debug','libplugindebugbc.so','Permite poner el modo debug en tiempo de ejecucion \n mediante una opcion en el menu Herramientas.','',''],
	['Proyectos','libpluginproyectos.so','Gestion de Proyectos como centros de coste.','',''],
	['Balance Jerarquico','libpluginbalancetree.so','Generacion de balances de situacion con presentacion jerarquica.','',''],
	['Balance de situacion','libpluginbalance1.so',"Generación de balances de situacion.",'',''],
	['Balance de situacion','libpluginbalance.so','Version anterior de la generacion de balances.','',''],
	['Cuentas Anuales ODS','libplugincanualesods.so','Generacion de las cuentas anuales en formato ODS.','',''],
# PLUGINS DE BULMALIB
	['Pegado desde Clipboard', 'libpluginclipboardbf.so', 'Copiando un area desde una hoja de calculo. Esta se puede insertar en un subformulario de BulmaG&eacute;s','',''],
	['Impresiones Personalizadas', 'libpluginimpers.so', 'Para tener multiples modelos de impresion para cada uno de los documentos.\n Genere tantas plantillas RML como desee para una factura, presupuesto u otros y utilicela cuando la precise. Funciona como un menu contextaual en las fichas que muestra todas las plantillas que contengan en su nombre el nombre de la entidad a la que la ficha representa.','',''],
	['Pegado desde Clipboard', 'libpluginclipboardbf.so', 'Copiando un area desde una hoja de calculo. Esta se puede\n insertar en un subformulario de BulmaGes','',''],
	['Exportar subforms a SXC', 'libpluginsubformsxc.so', 'Permite, mediante un menu contextual exportar cualquier\n subformulario a formato SXC y abrirlo con KSpread o OpenOffice','',''],
	['Embeber aplicaciones', 'libplugindocked.so', 'Embeba cualquier aplicacion X en el entorno de BulmaFact para\n tener las aplicaciones mas a mano','',''],
	['Exportar subforms a ODS', 'libpluginsubformods.so', 'Permite, mediante un menu contextual exportar cualquier\n subformulario a formato SXC y abrirlo con KSpread o OpenOffice','','']

	]
	
	self.pluginsbulmatpv = [
	['Abreviaciones','libpluginabrevs.so','Incluye metodos rapidos para pantallas tactiles.','',''],
	['Articulos Graficos','libpluginartgraficos.so','Permite la introduccion de articulos con rejilla','',''],
	['Teclado numerico','libplugintecladonumerico.so','Permite el control de un teclado numerico en pantallas tactiles','',''],
	['Totales','libplugintotal.so','Muestra los totales de un ticket','',''],
	['Administracion','libpluginadmin.so','Funcionalidades de administracion X y Z','',''],
	['Cobro','libplugincobrar.so','Introduce la funcionalidad del cobro de tickets','',''],
	['Ticket Basico','libpluginticketbasico.so','Permite el manejo basico de un Ticket','','']
	]
	


	