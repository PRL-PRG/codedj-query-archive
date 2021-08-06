import os
import sys
import email
import errno
import mimetypes
import poplib

def desempaquetaEmail(archivo, directorio):
	
	if not directorio:
		print "Error, no se ha pasado como parametro el nombre del directorio donde se desempaquetaran los correos"
		sys.exit(1)
	
	if not archivo:
		print "Error, no se ha pasado como parametro el nombre del archivo que contiene el e-mail a desempaquetar"
		sys.exit(1)

	try:
		os.mkdir(directorio)
	except OSError, e:
		# Ignoramos el error de directorio ya existente
		if e.errno <> errno.EEXIST:
			raise
	
	fp = open(archivo)
	msg = email.message_from_file(fp)
	fp.close()
	
	contador = 1
	for part in msg.walk():
		if part.get_content_maintype() == 'multipart':
			continue
		
		nomarchivo = part.get_filename()
		extension = mimetypes.guess_extension(part.get_content_type())
		
		if not extension:
			extension = '.email'

		if not nomarchivo:
			nomarchivo = 'part-%03d%s' % (contador, extension)
		
		contador += 1
		fp = open(os.path.join(directorio, nomarchivo), 'wb')
		fp.write(part.get_payload(decode=True))
		fp.close()

def recibeEmail(servidor, usuario, password, directorio="diremail", archivo="email"):

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

		# Dividimos el archivo en mensajes y adjunto/s
		desempaquetaEmail(archivo+numero, directorio+numero)

		# Obtenemos y decimos que se borre el mensaje al cerrar la conexion ( M.quit() )
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
