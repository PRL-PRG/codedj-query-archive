import  pygtk
pygtk.require('2.0')
import gtk
from gtk import gdk
import os

from Cursors import Cursores
from Botao import Botao
from Area import Area

DRAW_WIDTH = 1200
DRAW_HEIGHT = 800

class Oficina:
	def __init__(self):
		"""Initialize the Oficina object.

		Keyword arguments:
		self -- Oficina.Oficina instance

		"""
		#self.window = gtk.Window()

		#self.areaFixa = gtk.Fixed()
		#self.areaFixa.set_size_request(DRAW_WIDTH, DRAW_HEIGHT)

		# cor de fundo da janela
		#color = gtk.gdk.color_parse("white")
		#self.window.modify_bg(gtk.STATE_NORMAL, color)
		
		# imagem de fundo
		#self.fundo = gtk.Image()
		#self.fundo.set_from_file('fundo.png')
		#self.areaFixa.put(self.fundo, 0, 0)
		
		# cursores
		self.cursorLapis = Cursores('lapis_cursor.png')
		self.cursorCirculo = Cursores('circulo_cursor.png')
		self.cursorBorracha = Cursores('borracha_cursor.png')
		self.cursorQuadrado = Cursores('quadrado_cursor.png')
		self.cursorLinha = Cursores('linha_cursor.png')
		self.cursorLetra = Cursores('letra_cursor.png')		
		self.cursorSelecao = Cursores('selecao_cursor.png')	
		self.cursorPoligono = Cursores('poligono_cursor.png')	
		self.cursorMove = Cursores('move_cursor.png')
		
		self.area = Area(self)
		self.area.ferramenta = 2
		#self.areaFixa.put(self.area,0,0)

		# botoes de evento
		# ferramentas
		
		botao = Botao(self.areaFixa)
		botao.adicionaBotao('corlinha.png',-1,15,10,self.mousedown, "Cor Linha")
		botao.adicionaBotao('balde.png',-2,15,40,self.mousedown, "Balde")
		
		botao.adicionaBotao('linha.png',1,15,100,self.mousedown, "Linha")
		botao.adicionaBotao('lapis.png',2,130,110,self.mousedown, "Lapis")		
		botao.adicionaBotao('borracha.png',3,90,180,self.mousedown, "Borracha")
		botao.adicionaBotao('letra.png',4,50,220,self.mousedown, "Letra")
		botao.adicionaBotao('circulo.png',5,25,270,self.mousedown, "Circulo")
		botao.adicionaBotao('quadrado.png',6,20,320,self.mousedown, "Quadrado")
		botao.adicionaBotao('vassoura.png',7,20,360,self.mousedown, "Vassoura")
		# cor preenchimento
		# deprecated
		botao.adicionaBotao('roxo.png',8,210,42,self.mousedown, "Preenchimento Roxo")
		botao.adicionaBotao('amarelo.png',9,54,40,self.mousedown, "Preenchimento Amarelo")
		botao.adicionaBotao('preto.png',10,90,42,self.mousedown,  "Preenchimento Preto")
		botao.adicionaBotao('azul.png',11,120,40,self.mousedown, "Preenchimento Azul")
		botao.adicionaBotao('verde.png',12,150,42,self.mousedown,  "Preenchimento Verde")
		botao.adicionaBotao('vermelho.png',13,180,40,self.mousedown, "Preenchimento Vermelho")
		botao.adicionaBotao('laranja.png',14,280,42,self.mousedown, "Preenchimento Laranja")
		botao.adicionaBotao('branco.png',15,240,40,self.mousedown, "Preenchimento Branco")
		
		# cor linha
		# deprecated
		botao.adicionaBotao('roxo.png',16,210,12,self.mousedown, "Linha Roxa")
		botao.adicionaBotao('amarelo.png',17,54,10,self.mousedown, "Linha Amarela")
		botao.adicionaBotao('preto.png',18,90,12,self.mousedown, "Linha Preta")
		botao.adicionaBotao('azul.png',19,120,10,self.mousedown, "Linha Azul")
		botao.adicionaBotao('verde.png',20,150,12,self.mousedown, "Linha Verde")
		botao.adicionaBotao('vermelho.png',21,180,10,self.mousedown, "Linha Vermelha")
		botao.adicionaBotao('laranja.png',22,280,12,self.mousedown, "Linha Laranja")
		botao.adicionaBotao('branco.png',23,240,10,self.mousedown, "Linha Branca")		
		
		botao.adicionaBotao('abrir.png',24,660,10,self.mousedown, "Abrir")
		botao.adicionaBotao('salvar.png',25,600,10,self.mousedown, "Salvar")
		botao.adicionaBotao('selecao.png',26,550,10,self.mousedown, "Selecao")
		botao.adicionaBotao('poligono.png',27,70,95,self.mousedown, "Poligono")
		
		self.window.add(self.areaFixa)
		self.window.connect("destroy", gtk.main_quit)		
		# desenho de texto
		
		self.entrada = gtk.Entry(max=50)
		self.areaFixa.put(self.entrada,100,100)
		
		self.window.show_all()		
		self.entrada.hide()
		
		#self.area.show()
		#

	def mousedown(self,widget,event, ferramenta):
		"""Verify what event was called when the mouse pressed a button.

		Keyword arguments:
		self -- Oficina.Oficina instance
		widget -- gtk.EventBox
		event -- GdkEvent
		ferramenta -- integer "enum"

		"""
		self.entrada.hide()
		if ferramenta == 7:			
			self.area.d.limpatudo()	#vassoura	
		elif ferramenta == 8:
			self.area.mudacor(0) #roxo	
		elif ferramenta == 9:
			self.area.mudacor(1) #amarelo
		elif ferramenta == 10:
			self.area.mudacor(2) #preto
		elif ferramenta == 11:
			self.area.mudacor(3) #azul
		elif ferramenta == 12:
			self.area.mudacor(4) #verde
		elif ferramenta == 13:
			self.area.mudacor(5) #vermelho
		elif ferramenta == 14:
			self.area.mudacor(6) #laranja
		elif ferramenta == 15:
			self.area.mudacor(7) #branco			
		elif ferramenta == 16:
			self.area.mudacorlinha(0)
		elif ferramenta == 17:
			self.area.mudacorlinha(1)
		elif ferramenta == 18:
			self.area.mudacorlinha(2)		
		elif ferramenta == 19:
			self.area.mudacorlinha(3)	
		elif ferramenta == 20:
			self.area.mudacorlinha(4)
		elif ferramenta == 21:
			self.area.mudacorlinha(5)		
		elif ferramenta == 22:
			self.area.mudacorlinha(6) 
		elif ferramenta == 23:
			self.area.mudacorlinha(7)		
		elif ferramenta == 24:	
			dialog = gtk.FileChooserDialog(title=('Abrir Arquivo...'),   
                                  action=gtk.FILE_CHOOSER_ACTION_OPEN,   
                                  buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL,   
                                  gtk.STOCK_SAVE, gtk.RESPONSE_OK)) 
			dialog.show_all()			
			response = dialog.run()
			if response == gtk.RESPONSE_OK:
				print dialog.get_filename(), 'selected'
				gtk28 = False 
				file_path = dialog.get_filename()
				file_path = self.decode_path((file_path,))[0]  
				self.open(file_path)
			elif response == gtk.RESPONSE_CANCEL:
				print 'Closed, no files selected'
			dialog.destroy()
			
		elif ferramenta == 25:
			dialog = gtk.FileChooserDialog(title=('Salvar Arquivo como...'),   
                                  action=gtk.FILE_CHOOSER_ACTION_SAVE,   
                                  buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL,   
                                  gtk.STOCK_SAVE, gtk.RESPONSE_OK))  

			dialog.show_all()			
			response = dialog.run()
			if response == gtk.RESPONSE_OK:
				print dialog.get_filename(), 'selected'
				gtk28 = False 
				file_path = dialog.get_filename()
				file_path = self.decode_path((file_path,))[0]  
				self.save(file_path)
			elif response == gtk.RESPONSE_CANCEL:
				print 'Closed, no files selected'
			dialog.destroy()
		
		else:						
			if ferramenta == 1:
				print "linha"
				self.area.window.set_cursor(self.cursorLinha.cursor())
			elif ferramenta == 2:
				print "lapis"
				self.area.window.set_cursor(self.cursorLapis.cursor())			
			elif ferramenta == 3:
				print "borracha"
				self.area.window.set_cursor(self.cursorBorracha.cursor())
			elif ferramenta == 4:
				print "letra"
				self.area.window.set_cursor(self.cursorLetra.cursor())
			elif ferramenta == 5:
				print "circulo"
				self.area.window.set_cursor(self.cursorCirculo.cursor())
			elif ferramenta == 6:
				print "quadrado"
				self.area.window.set_cursor(self.cursorQuadrado.cursor())		
			elif ferramenta == 26:
				self.area.window.set_cursor(self.cursorSelecao.cursor())			
			elif ferramenta == 27:
				self.area.window.set_cursor(self.cursorPoligono.cursor())	
				self.area.primeira = 1
			
			self.area.ferramenta = ferramenta

	def open(self, name):
		self.area.d.limpatudo()
		self.area.d.loadImage(name)
			
	def save(self, name):
		"""Save the drawing.

		Keyword arguments:
		self -- Oficina.Oficina instance
		name -- string (path where the file will be saved)

		"""
		pixbuf = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, DRAW_WIDTH, DRAW_HEIGHT)
		pixbuf.get_from_drawable(self.area.pixmap, gtk.gdk.colormap_get_system(), 0, 0, 0, 0, -1, -1)
		pixbuf.save(name + ".png", "png", {})	
		
	def decode_path(self, file_paths):
		"""

		Keyword arguments:
		self -- Oficina.Oficina instance
		file_paths -- tuple with the string of the path

		"""
		file_paths_list = list()
		if os.name == 'nt': #  Windows 
			for file_path in file_paths: 
				file_path = file_path.decode('utf8') 
				file_paths_list.append(file_path) 
				print "file_path", file_path
				
		else: 
			for file_path in file_paths: 
				try: 
					file_path = file_path.decode(sys.getfilesystemencoding()) 
				except: 
					try: 
						file_path = file_path.decode('utf-8') 
					except: 
						pass 
					file_paths_list.append(file_path) 

		return file_paths_list 
