import sys
import getopt



class Mensaje:
	"Representa la llegada de un mensaje"
	def __init__(self):
		self.m_mensaje = ""

class Metodo:
	"Representa un metodo invocado"
	def __init__(self):
		self.m_nombreMetodo = ""
		self.m_invocacionesInit = 0
		self.m_invocacionesEnd = 0
	
	def imprimir(self):
		print "      " + self.m_nombreMetodo + "  %s " % self.m_invocacionesInit

class Clase:
	"Representa una clase"
        def __init__(self):
                self.m_nombreClase = ""
		self.m_invocaciones = 0
		self. m_listaMetodos = {}

	def procesaMensaje(self, mens):
		mensaje = mens
		if mensaje.startswith("END "):
			mensaje = mensaje[4:]
		lmensaje = mensaje.split("::")
		submens = lmensaje[1].split(" ")
		lmensaje[1] = submens[0]

		if lmensaje[1] in self.m_listaMetodos:
			a = self.m_listaMetodos[lmensaje[1]]
			a.m_invocacionesInit = a.m_invocacionesInit + 1
			self.m_listaMetodos[lmensaje[1]] = a
		else:
			a = Metodo()
			a.m_nombreMetodo = lmensaje[1]
			a.m_invocacionesInit = 1
			self.m_listaMetodos[lmensaje[1]] = a

	def imprimir(self):
		print self.m_nombreClase + "  %s" % self.m_invocaciones
		for x in self.m_listaMetodos:
			self.m_listaMetodos[x].imprimir()

class ListaClases:
	"Listado de clases"
	def __init__(self):
		self.m_listaClases = {}

	def procesaMensaje(self, mens):
		# Buscamos la clase del mensaje.
		mensaje = mens
		if mensaje.startswith("END "):
			mensaje = mensaje[4:]
		lmensaje = mensaje.split("::")
		if len(lmensaje) < 2:
			return

		if lmensaje[0] in self.m_listaClases:
			a = self.m_listaClases[lmensaje[0]]
			a.m_invocaciones = a.m_invocaciones +1
			a.procesaMensaje(mens)
			self.m_listaClases[lmensaje[0]] = a
		else:
			a = Clase()
			a.m_nombreClase = lmensaje[0]
			a.m_invocaciones = 1
			a.procesaMensaje(mens)
			self.m_listaClases[lmensaje[0]] = a

	def imprimir(self):
		for x in self.m_listaClases:
			self.m_listaClases[x].imprimir()


if (__name__=='__main__'):
	lmen = ListaClases()
		
	f = open("~/.bulmages/bulmagesout.txt", 'r')
	f.seek(0,2)
	while 1:
		line = f.readline(300)
		if line != "":
			lmen.procesaMensaje(line.replace("\n", ""))
			print line

	lmen.imprimir()
	