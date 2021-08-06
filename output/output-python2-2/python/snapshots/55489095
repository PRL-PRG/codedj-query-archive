import os
import sys
import email
import errno
import mimetypes
import poplib
import md5
import string

def desempaquetaEmail(archivo):
	
	if not archivo:
		print "Error, no se ha pasado como parametro el nombre del archivo que contiene el e-mail a desempaquetar"
		sys.exit(1)
	
	# Abrimos el archivo y lo obtenemos como mensaje de correo electronico dentro de msg
	fp = open(archivo)
	msg = email.message_from_file(fp)
	fp.close()
	
	# Instancia del objeto MD5 que nos servira para crear los nombres de los directorios con el HASH del XML
	m = md5.new()
	
	# Empezamos a desempaquetar el mensaje. Ignoramos todo lo que no sean XMLs
	contador = 1
	for part in msg.walk():
		if part.get_content_maintype() == 'multipart':
			continue
		
		nomarchivo = part.get_filename()
		extension = mimetypes.guess_extension(part.get_content_type())
		
		#if not extension:
			#extension = '.email'

		# Para el mensaje del email, que no devuelve nombre de archivo y hace que
		# esto pete en el if para encontrar la cadena '.xml'
		if not nomarchivo:
			nomarchivo = 'part-%03d%s' % (contador, extension)
		
		contador += 1
		
		# Si es un XML suponemos que es una e-factura y generamos un HASH MD5 con el contenido del archivo.
		# Este HASH sera el nombre del directorio donde alojemos finalmente el XML. Asi evitamos el problema
		# de como nombrar a un directorio una vez obtenida la factura.
		# Usamos esto y no damos lugar a la imaginacion, ya nos da una manera segura para no repetir nombres.
		
		
		#encontrado = nomarchivo.find('.xml')
		
		if nomarchivo.find('.xml') <> -1:
			# Aqui se crea el HASH MD5 con el contenido del fichero XML
			m.update(part.get_payload(decode=True))
			
			try:
				# Creamos el directorio usando el HASH como nombre
				os.mkdir(m.hexdigest())
			except OSError, e:
				# Ignoramos el error de directorio ya existente (nos mandan la misma efactura varias veces)
				if e.errno <> errno.EEXIST:
					raise
			
			# Vamos al directorio que acabamos de crear para escribir ahi el fichero XML
			os.chdir(m.hexdigest())
			
			fp = open(nomarchivo, 'wb')
			fp.write(part.get_payload(decode=True))
			fp.close()
			
			# Volvemos al anterior directorio de trabajo para seguir con mas efacturas o acabar :)
			os.chdir('../');

def recibeEmail(servidor, usuario, password, archivo="email"):

	# El parametro archivo solo es el prefijo con el que va el archivo temporal usado para desempaquetar

	M = poplib.POP3(servidor)
	M.user(usuario)
	M.pass_(password)

	numMessages = len(M.list()[1])
	estado = M.stat();

	# Recorremos mensajes nuevos
	for i in range(numMessages):

		numero = str(i+1)

		f=open(archivo+numero,"wb")

		string = '';

		# Recorremos lineas de mensaje
		for j in M.retr(i+1)[1]:
			# Aqui hay que ir guardando las lineas en un string y
			# guardarlo como fichero. Luego, de ese fichero, obtener
			# el/los archivo/s adjunto/s
			string += j+"\n";

		# Guardamos el archivo y cerramos el descriptor de fichero
		f.write(string)
		f.close()

		# Obtenemos el XML de la efactura
		desempaquetaEmail(archivo+numero)

		# Ahora toca borrar el fichero del cual hemos desempaquetado el XML
		os.remove(archivo+numero)

		# Indicamos que se borre el mensaje del servidor al cerrar la conexion ( con M.quit() )
		M.dele(i+1)

	#print "Mensajes nuevos:"
	#print estado[0]

	# Commit de los cambios en el buzon y se cierra la conexion
	M.quit()

recibeEmail(
	sys.argv[1], # Servidor
	sys.argv[2], # Usuario
	sys.argv[3], # Password
)
