import smtplib
import os
import sys
from email.MIMEMultipart import MIMEMultipart
from email.MIMEBase import MIMEBase
from email.MIMEText import MIMEText
from email.Utils import COMMASPACE, formatdate
from email import Encoders

def enviaEmail(origen, destino, asunto, text, archivo, server="localhost"):

	msg = MIMEMultipart()
	msg['From'] = origen
	msg['To'] = COMMASPACE.join(destino)
	msg['Date'] = formatdate(localtime=True)
	msg['Subject'] = asunto
	
	msg.attach( MIMEText(text) )
	
	part = MIMEBase('application', "octet-stream")
	part.set_payload( open(archivo,"rb").read() )
	Encoders.encode_base64(part)
	part.add_header('Content-Disposition', 'attachment; filename="%s"' % os.path.basename(archivo))
	msg.attach(part)
	
	smtp = smtplib.SMTP(server)
	smtp.sendmail(origen, destino, msg.as_string())
	smtp.close()

enviaEmail (
	sys.argv[1], # Origen
	sys.argv[2], # Destino
	sys.argv[3], # Asunto, pasado como argumento entre comillas dobles
	sys.argv[4], # Mensaje, pasado como argumento entre comillas dobles
	sys.argv[5]  # Archivo adjunto
)
