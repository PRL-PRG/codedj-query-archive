"""
 by Antoni Aloy López

 Definición de las clases básicas para el manejo de cuentas contables
 Versión : 0.1
 Fecha : 04/08/2001

 Licencia : GNU

"""

class Persistencia:
	"""Esta clase debe implementar todo lo relacionado
	con la persistencia: guardar, obtener, eliminar
	"""
	def __init__(self):
		pass
	def save(self):
		pass
	def load(self):
		pass
	def delete(self):
		pass
	def buscar(self,clave):
		return 1
		
class ItemConta(Persistencia):
	"""Clase básica para el manejo de la mayoría de los objetos
	contables que constan de código y descripción
	"""
	def __init__(self,codigo, descripcion=''):
		Persistencia.__init__(self)
		self.codigo=codigo
		self.descripcion=descripcion
		
	def __repr__(self):
		return self.codigo+'\t'+self.descripcion

	

class Cuenta(ItemConta):
	"""Cuenta
	Definición de la clase para manejar una cuenta contable
	"""
	def __init__(self,codigo,descripcion=''):
		ItemConta.__init__(self,codigo,descripcion)
		self.activa = 1			# 1 ó 0
		self.seguridad = 0		# nivel de seguridad
		self.padre = None		# nivel anterior de cta
		self.grupo = None
		
	def __repr__(self):
		return self.codigo+'\t'+self.descripcion

class Diario(ItemConta):
	""" Diario
	Definición de la clase diario
	"""
	def __init__(self,codigo,descripcion):
		ItemConta.__init__(self,codigo,descripcion)

class Impuestos(ItemConta):
	"""Impuestos
	Clase para el manejo de los distintos tipos impositivos
	"""
	def __init__(self,codigo,descripcion):
		ItemConta.__init__(self,codigo,descripcion)
		self.tipo = 0.0  # %
	def __repr__(self):
		return ItemConta.__repr__(self)+'\t'+`self.tipo` + "%"
	
	def obtenerDescripcion(codigo):
		if self.buscar(codigo) :
			# Actualizar la información
			return self.descripcion
		else:
			raise "Código no encontrado", self.codigo

class ConceptosContables(ItemConta):
	"""Conceptos Contables
	Clase para el manejo de los conceptos contables
	"""	
	def __init__(self,codigo,descripcion):
				ItemConta.__init__(self,codigo,descripcion)
	
class Borrador(ItemConta):
	"""Borrador
	Clase que mantiene información sobre los borradores
	"""
	def __init__(self,codigo,descripcion,numero,bloqueado='NO'):
		ItemConta.__init__(self,codigo,descripcion)
		self.numero = numero
		self.bloqueado=bloqueado
		self.usuario = '' # Por si queremos info usuario
		
	def __repr__(self):
		return ItemConta.__repr__(self)+'\t'+`self.numero` + "\t"+self.bloqueado

	def __desbloquear(self):
		#Debe implementarse a nivel de tabla		
		pass
	def __guardar(self):
		#Debe implementarse a nivel de tabla
		pass
	def __borrar(self):
		#Debe implementarse a nivel de tabla
		pass
		
	def __bloquear(self):
		#A implementar
		pass
		
	def desbloquear(self):
		# Desbloquea un borrador para su uso
		self.__desbloquear()
		self.bloqueado='NO'
		
	def bloquear(self):
		# Bloquea un borrador para evitar su uso
		self.__bloquear()
		self.bloqueado ='SI'
		
	def guardar(self):
		# Guarda el borrador
		self.__guardar()
		self.numero=self.numero+1
		
	def borrar(self):
		# Elimina el borrador
		self.__borrar()
		

class Movimiento(Persistencia):
	"""Definición de un movimiento contable
	Un conjunto de movimientos cuadrados forman un apunte"""
	
	def __init__(self, num, fecha, cuenta, diario, concepto, debe, haber, contrapartida,numdoc, documento, comentario, descripcion, vencimiento, impuestos, conciliado):
		Persistencia.__init__(self)
		self.num = num  		# id del movimiento
		self.fecha = fecha 		# fecha del mvto
		self.cuenta = cuenta		# cuenta a la que se imputa
		self.diario = diario		# diario
		self.concepto = concepto	# concepto contable
		self.debe = debe		# importe debe
		self.haber = haber		# importe haber (sólo uno de los dos puede ser <>0)
		self.contrapartida = contrapartida
		self.numdoc = numdoc		# num doc asociado (fra, por ejemplo)
		self.documento = documento	# localización del documento
		self.comentario = comentario	# comentario corto del mvto (para conciliación)
		self.descripcion = descripcion	# descripcion ampliada
		self.vencimiento= vencimiento	# fecha de vencimiento asociada
		self.impuestos	= impuestos	# impuestos asociados
		self.conciliado = conciliado 	# marca de conciliación
	def __repr__(self):
		if self.debe == 0:
			importe = self.haber
			tipo = 'H'
		else:
			importe = self.debe
			tipo = 'D'
		return `self.num` +'\t'+ self.fecha+'\t'+self.cuenta+'\t'+`importe`+tipo+'\t'+self.comentario
			

if __name__=='__main__':
	plan = {}
	cta = Cuenta('4300','Clientes')
	plan[cta.codigo]=cta
	cta = Cuenta('4100','Acreedores')
	plan[cta.codigo]=cta
	cta = Cuenta('4000','Proveedores')
	plan[cta.codigo]=cta
	cta = Cuenta('4300999','Clientes Varios')
	plan[cta.codigo]=cta

	cuentas = plan.keys()
	cuentas.sort()
	print "Plan de cuentas muy corto"
	for cta in cuentas:
		print cta
	
	print "I ahora algo completamente diferente ...!"
	impto = Impuestos('IVA7','Iva al 7%')
	impto.tipo = 7.0
	print impto

	print "Borradores"
	borrador = Borrador('AAL','Borrador de toni',100)
	print borrador

	print "Movimiento"
	mvto = Movimiento(1,'10/10/2001','43000','001','0033',1222,0,'57200', 0, 0, 'NO COMENT', 'XXX','25/10/2003',0,'NO')
	print mvto
