# Conozco Uruguay
# Copyright (C) 2008 Gabriel Eirea
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Contact information:
# Gabriel Eirea geirea@gmail.com
# Ceibal Jam http://wiki.laptop.org/go/Ceibal_Jam

import sys, pygame, random, os
#import datetime, time
from sugar.activity import activity

RADIO = 10
RADIO2 = RADIO**2
XMAPAMAX = 786
DXPANEL = 414
XCENTROPANEL = 1002
YGLOBITO = 310
DXBICHO = 218
DYBICHO = 268
XBICHO = 1200-DXBICHO
YBICHO = 900-DYBICHO
XNAVE = 800
YNAVE = 650
DXNAVE = 100
DYNAVE = 200
CAMINODATOS = "datos"
ARCHIVODEPTOS = "departamentos.txt"
ARCHIVOLUGARES = "ciudades.txt"
ARCHIVONIVELES = "niveles.txt"
ARCHIVOEXPLORACIONES = "exploraciones.txt"
ARCHIVORIOS = "rios.txt"
ARCHIVOCREDITOS = "creditos.txt"
CAMINOIMAGENES = "imagenes"
COLORNOMBREDEPTO = (200,60,60)
COLORNOMBRECAPITAL = (10,10,10)
COLORNOMBRERIO = (100,100,20)
COLORPREGUNTAS = (80,80,155)
COLORPANEL = (156,158,172)
TOTALAVANCE = 7
EVENTORESPUESTA = pygame.USEREVENT+1
TIEMPORESPUESTA = 2300
EVENTODESPEGUE = EVENTORESPUESTA+1
TIEMPODESPEGUE = 40

class Punto():

    def __init__(self,nombre,tipo,simbolo,posicion,postexto):
        self.nombre = nombre
        self.tipo = int(tipo)
        self.posicion = (int(posicion[0]),int(posicion[1]))
        self.postexto = (int(postexto[0])+self.posicion[0],
                         int(postexto[1])+self.posicion[1])
        self.simbolo = simbolo

    def estaAca(self,pos):
        """Devuelve un booleano indicando si esta en la coordenada pos,
        la precision viene dada por la constante global RADIO"""
        if (pos[0]-self.posicion[0])**2+(pos[1]-self.posicion[1])**2 < RADIO2:
            return True
        else:
            return False

    def dibujar(self,pantalla,flipAhora):
        """Dibuja un punto en su posicion"""
        pantalla.blit(self.simbolo, (self.posicion[0]-8,self.posicion[1]-8))
        if flipAhora:
            pygame.display.flip()

    def mostrarNombre(self,pantalla,fuente,color,flipAhora):
        """Escribe el nombre del punto en su posicion"""
        text = fuente.render(self.nombre, 1, color)
        textrect = text.get_rect()
        textrect.center = (self.postexto[0],self.postexto[1])
        pantalla.blit(text, textrect)
	if flipAhora:
            pygame.display.flip()


class Zona():

    def __init__(self,mapa,nombre,claveColor,tipo,posicion,rotacion):
        self.mapa = mapa # esto hace una copia en memoria o no????
        self.nombre = nombre
        self.claveColor = int(claveColor)
        self.tipo = int(tipo)
        self.posicion = (int(posicion[0]),int(posicion[1]))
        self.rotacion = int(rotacion)

    def estaAca(self,pos):
        if pos[0] < XMAPAMAX:
            colorAca = self.mapa.get_at((pos[0], pos[1]))
            if colorAca[0] == self.claveColor:
                return True
            else:
                return False
        else:
            return False

    def mostrarNombre(self,pantalla,fuente,color,flipAhora):
        """Escribe el nombre de la zona en su posicion"""
        text = fuente.render(self.nombre, 1, color)
        textrot = pygame.transform.rotate(text,self.rotacion)
        textrect = textrot.get_rect()
        #print self.posicion[0] + " " + self.posicion[1]
        textrect.center = (self.posicion[0],self.posicion[1])
        pantalla.blit(textrot, textrect)
	if flipAhora:
            pygame.display.flip()


class Nivel():

    def __init__(self,nombre):
        self.nombre = nombre
        self.dibujoInicial = list()
        self.nombreInicial = list()
        self.preguntas = list()
        self.indicePreguntaActual = 0
        self.elementosActivos = list()

    def prepararPreguntas(self):
        """Este metodo sirve para preparar la lista de preguntas al azar."""
        random.shuffle(self.preguntas)

    def siguientePregunta(self,listaSufijos,listaPrefijos):
        self.preguntaActual = self.preguntas[self.indicePreguntaActual]
        self.sufijoActual = random.randint(1,len(listaSufijos))-1
        self.prefijoActual = random.randint(1,len(listaPrefijos))-1
        lineas = listaPrefijos[self.prefijoActual].split("\\")
        lineas.extend(self.preguntaActual[0].split("\\"))
        lineas.extend(listaSufijos[self.sufijoActual].split("\\"))
        self.indicePreguntaActual = self.indicePreguntaActual+1
        if self.indicePreguntaActual == len(self.preguntas):
            self.indicePreguntaActual = 0
        return lineas

    def mostrarPregunta(self,pantalla,fuente,sufijo,prefijo):
        self.preguntaActual = self.preguntas[self.indicePreguntaActual]
        lineas = prefijo.split("\\")
        lineas.extend(self.preguntaActual[0].split("\\"))
        lineas.extend(sufijo.split("\\"))
        yLinea = 100
        for l in lineas:
            text = fuente.render(l, 1, COLORPREGUNTAS)
            textrect = text.get_rect()
            textrect.center = (XCENTROPANEL,yLinea)
            pantalla.blit(text, textrect)
            yLinea = yLinea + fuente.get_height()
	pygame.display.flip()

    def esCorrecta(self,listaDeptos,listaLugares,listaRios,pos):
        respCorrecta = self.preguntaActual[2]
        # primero averiguar tipo
        if self.preguntaActual[1] == 1: # DEPTO
            # buscar depto correcto
            encontrado = False
            for d in listaDeptos:
                if d.nombre.startswith(respCorrecta):
                    encontrado = True
                    break
            if not encontrado:
                print "Error: no se encontro respuesta depto "+respCorrecta
                return False
            if d.estaAca(pos):
                return True
            else:
                return False
        elif self.preguntaActual[1] == 2: #CAPITAL
            # buscar lugar correcto
            encontrado = False
            for l in listaLugares:
                if l.nombre.startswith(respCorrecta):
                    encontrado = True
                    break
            if not encontrado:
                print "Error: no se encontro respuesta capital "+respCorrecta
                return False
            if l.estaAca(pos):
                return True
            else:
                return False
        if self.preguntaActual[1] == 3: # RIO
            # buscar rio correcto
            encontrado = False
            for d in listaRios:
                if d.nombre.startswith(respCorrecta):
                    encontrado = True
                    break
            if not encontrado:
                print "Error: no se encontro respuesta rio "+respCorrecta
                return False
            if d.estaAca(pos):
                return True
            else:
                return False

#class ConozcoUy(activity.Activity):
class ConozcoUy():

    def mostrarTexto(self,texto,fuente,posicion,color):
        """Muestra texto en una determinada posicion"""
        text = fuente.render(texto, 1, color)
        textrect = text.get_rect()
        textrect.center = posicion
        self.pantalla.blit(text, textrect)

    def cargarDepartamentos(self):
        """Carga las imagenes y los datos de los departamentos"""
        self.deptos = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"deptos.png"))
        self.deptosLineas = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"deptosLineas.png"))
        self.listaDeptos = list()
        # falta sanitizar manejo de archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVODEPTOS),"r")
        linea = f.readline()
        while linea:
            if linea[0] == "#":
                linea = f.readline()
                continue
            [nombreDepto,claveColor,posx,posy,rotacion]=linea.strip().split("|")
            nuevoDepto = Zona(self.deptos,unicode(nombreDepto,'iso-8859-1'),
                              claveColor,1,(posx,posy),rotacion)
            self.listaDeptos.append(nuevoDepto)
            linea = f.readline()
        f.close()

    def cargarRios(self):
        """Carga las imagenes y los datos de los rios"""
        self.rios = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"rios.png"))
        self.riosDetectar = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"riosDetectar.png"))
        self.listaRios = list()
        # falta sanitizar manejo de archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVORIOS),"r")
        linea = f.readline()
        while linea:
            if linea[0] == "#":
                linea = f.readline()
                continue
            [nombreRio,claveColor,posx,posy,rotacion]=linea.strip().split("|")
            nuevoRio = Zona(self.riosDetectar,unicode(nombreRio,'iso-8859-1'),
                              claveColor,1,(posx,posy),rotacion)
            self.listaRios.append(nuevoRio)
            linea = f.readline()
        f.close()

    def cargarLugares(self):
        """Carga los datos de las ciudades y otros puntos de interes"""
        self.simboloCapital = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"capital.png"))
        self.simboloCiudad = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"ciudad.png"))
        self.listaLugares = list()
        # falta sanitizar manejo de archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVOLUGARES),"r")
        linea = f.readline()
        while linea:
            if linea[0] == "#":
                linea = f.readline()
                continue
            [nombreLugar,posx,posy,tipo,incx,incy] = \
                linea.strip().split("|")
            if int(tipo) == 1:
                simbolo = self.simboloCapital
            elif int(tipo) == 2:
                simbolo = self.simboloCiudad
            else:
                simbolo = self.simboloCiudad
            nuevoLugar = Punto(unicode(nombreLugar,'iso-8859-1'),
                               int(tipo),simbolo,
                               (posx,posy),(incx,incy))
            self.listaLugares.append(nuevoLugar)
            linea = f.readline()
        f.close()

    def cargarNiveles(self):
        self.listaNiveles = list()
        self.listaPrefijos = list()
        self.listaSufijos = list()
        self.listaCorrecto = list()
        self.listaMal = list()
        self.listaDespedidas = list()
        # falta sanitizar manejo de archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVONIVELES),"r")
        linea = f.readline()
        while linea:
            if linea[0] == "#":
                linea = f.readline()
                continue
            if linea[0] == "[":
                # empieza nivel
                nombreNivel = linea.strip("[]\n")
                nuevoNivel = Nivel(nombreNivel)
                self.listaNiveles.append(nuevoNivel)
                linea = f.readline()
                continue
            if linea.find("=") == -1:
                linea = f.readline()
                continue         
            [var,valor] = linea.strip().split("=")
            if var.startswith("Prefijo"):
                self.listaPrefijos.append(unicode(valor.strip(),'iso-8859-1'))
            elif var.startswith("Sufijo"):
                self.listaSufijos.append(unicode(valor.strip(),'iso-8859-1'))
            elif var.startswith("Correcto"):
                self.listaCorrecto.append(unicode(valor.strip(),'iso-8859-1'))
            elif var.startswith("Mal"):
                self.listaMal.append(unicode(valor.strip(),'iso-8859-1'))
            elif var.startswith("Despedida"):
                self.listaDespedidas.append(unicode(valor.strip(),'iso-8859-1'))
            elif var.startswith("dibujoInicial"):
                listaDibujos = valor.split(",")
                for i in listaDibujos:
                    nuevoNivel.dibujoInicial.append(i.strip())
            elif var.startswith("nombreInicial"):
                listaNombres = valor.split(",")
                for i in listaNombres:
                    nuevoNivel.nombreInicial.append(i.strip())
            elif var.startswith("Pregunta"):
                [texto,tipo,respuesta] = valor.split("|")
                nuevoNivel.preguntas.append(
                    (unicode(texto.strip(),'iso-8859-1'),
                     int(tipo),
                     unicode(respuesta.strip(),'iso-8859-1')))
            linea = f.readline()
        f.close()
        self.indiceNivelActual = 0
        self.numeroNiveles = len(self.listaNiveles)
        self.numeroSufijos = len(self.listaSufijos)
        self.numeroPrefijos = len(self.listaPrefijos)
        self.numeroCorrecto = len(self.listaCorrecto)
        self.numeroMal = len(self.listaMal)
        self.numeroDespedidas = len(self.listaDespedidas)

    def cargarExploraciones(self):
        self.listaExploraciones = list()
        # falta sanitizar manejo de archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVOEXPLORACIONES),"r")
        linea = f.readline()
        while linea:
            if linea[0] == "#":
                linea = f.readline()
                continue
            if linea[0] == "[":
                # empieza nivel
                nombreNivel = linea.strip("[]\n")
                nuevoNivel = Nivel(nombreNivel)
                self.listaExploraciones.append(nuevoNivel)
                linea = f.readline()
                continue
            if linea.find("=") == -1:
                linea = f.readline()
                continue         
            [var,valor] = linea.strip().split("=")
            if var.startswith("dibujoInicial"):
                listaDibujos = valor.split(",")
                for i in listaDibujos:
                    nuevoNivel.dibujoInicial.append(i.strip())
            elif var.startswith("nombreInicial"):
                listaNombres = valor.split(",")
                for i in listaNombres:
                    nuevoNivel.nombreInicial.append(i.strip())
            elif var.startswith("elementosActivos"):
                listaNombres = valor.split(",")
                for i in listaNombres:
                    nuevoNivel.elementosActivos.append(i.strip())
            linea = f.readline()
        f.close()
        self.numeroExploraciones = len(self.listaExploraciones)

    def pantallaAcercaDe(self):
        self.pantallaTemp = pygame.Surface((1200,900))
        self.pantallaTemp.blit(self.pantalla,(0,0))
        self.pantalla.fill((0,0,0))
        self.mostrarTexto("Acerca de Conozco Uruguay",
                          self.fuente40,(600,100),(255,255,255))
        # falta sanitizar acceso a archivo
        f = open(os.path.join(CAMINODATOS,ARCHIVOCREDITOS),"r")
        yLinea = 200
        for linea in f:
            self.mostrarTexto(linea.strip(),
                          self.fuente32,(600,yLinea),(155,155,255))
            yLinea = yLinea + 40
        f.close()
        self.mostrarTexto("Presione cualquier tecla para volver.",
                          self.fuente32,(600,800),(255,155,155))
	pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    self.pantalla.blit(self.pantallaTemp,(0,0))
                    pygame.display.flip()
                    return

    def pantallaInicial(self):
        self.pantalla.fill((0,0,0))
        self.mostrarTexto("Conozco Uruguay",
                          self.fuente40,(600,100),(255,255,255))
        self.mostrarTexto("Juego",
                          self.fuente40,(300,200),(200,100,100))
        yLista = 300
        for n in self.listaNiveles:
            self.pantalla.fill((20,20,20),(10,yLista-24,590,48))
            self.mostrarTexto(n.nombre,
                              self.fuente40,(300,yLista),(200,100,100))
            yLista += 50
        self.mostrarTexto("Exploro",
                          self.fuente40,(900,200),(100,100,200))
        yLista = 300
        for n in self.listaExploraciones:
            self.pantalla.fill((20,20,20),(610,yLista-24,590,48))
            self.mostrarTexto(n.nombre,
                              self.fuente40,(900,yLista),(100,100,200))
            yLista += 50
        self.pantalla.fill((20,20,20),(10,801,590,48))
        self.mostrarTexto("Sobre este juego",
                          self.fuente40,(300,825),(100,200,100))
        self.pantalla.fill((20,20,20),(610,801,590,48))
        self.mostrarTexto("Salir",
                          self.fuente40,(900,825),(100,200,100))
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    if event.key == 27: # escape
                        sys.exit()
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    pos = event.pos
                    if pos[1] > 275:
                        if pos[0] < 600:
                            if pos[1] < 275+len(self.listaNiveles)*50:
                                self.indiceNivelActual = (pos[1]-275)//50
                                self.jugar = True
                                return
                            elif pos[1] > 800 and pos[1] < 850:
                                self.pantallaAcercaDe()
                        else:
                            if pos[1] < 275+len(self.listaExploraciones)*50:
                                self.indiceNivelActual = (pos[1]-275)//50
                                self.jugar = False
                                return
                            elif pos[1] > 800 and pos[1] < 850:
                                sys.exit()


#    def __init__(self,handle):
    def __init__(self):
        """Esta es la inicializacion del juego"""
#        activity.Activity.__init__(self,handle)
        pygame.init()
        # crear pantalla
        self.anchoPantalla, self.altoPantalla = 1200,900
        self.pantalla = pygame.display.set_mode((self.anchoPantalla,
                                                 self.altoPantalla))
        # cargar imagenes
        self.fondo = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"fondo.png"))
        self.globito = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"globito.png"))
        self.bicho = pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"bicho.png"))
        self.nave = list()
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave1.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave2.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave3.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave4.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave5.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave6.png")))
        self.nave.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"nave7.png")))
        self.fuego = list()
        self.fuego.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"fuego1.png")))
        self.fuego.append(pygame.image.load( \
            os.path.join(CAMINOIMAGENES,"fuego2.png")))
        # cargar fuentes, se puede hacer mejor esto??????
        self.fuente40 = pygame.font.Font(None, 40)
        self.fuente32 = pygame.font.Font(None, 32)
        self.fuente24 = pygame.font.Font(None,24)
        # cargar datos
        self.cargarDepartamentos()
        self.cargarRios()
        self.cargarLugares()
        self.cargarNiveles()
        self.cargarExploraciones()

    def mostrarTodo(self):
        """Muestra todos los nombres, solo de prueba."""
        for d in self.listaDeptos:
            d.mostrarNombre(self.pantalla,self.fuente32,
                            COLORNOMBREDEPTO,False)
        for l in self.listaLugares:
            l.dibujar(self.pantalla,False)
            l.mostrarNombre(self.pantalla,self.fuente24,
                            COLORNOMBRECAPITAL,False)
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    if event.key == 27: # escape
                        sys.exit()

    def descubrirNombres(self):
        """Este es un jueguito point-and-click, solo de prueba."""
        self.pantalla.blit(self.deptosLineas, (0, 0))
        for l in self.listaLugares:
            l.dibujar(self.pantalla,False)
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    if event.key == 27: # escape
                        sys.exit()
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    encontro = False
                    for l in self.listaLugares:
                        if l.estaAca(event.pos):
                            l.mostrarNombre(self.pantalla,self.fuente24,
                                            COLORNOMBRECAPITAL,True)
                            encontro = True
                            break
                    if not encontro:
                        for d in self.listaDeptos:
                            if d.estaAca(event.pos):
                                d.mostrarNombre(self.pantalla,self.fuente32,
                                                COLORNOMBREDEPTO,True)
                                break

    def mostrarGlobito(self,lineas):
        """Muestra texto en el globito"""
        self.pantalla.blit(self.globito,(XMAPAMAX,YGLOBITO))
        yLinea = YGLOBITO+self.fuente32.get_height()*3
        for l in lineas:
            text = self.fuente32.render(l, 1, COLORPREGUNTAS)
            textrect = text.get_rect()
            textrect.center = (XCENTROPANEL,yLinea)
            self.pantalla.blit(text, textrect)
            yLinea = yLinea + self.fuente32.get_height()+10
	pygame.display.flip()

    def borrarGlobito(self):
        """ Borra el globito"""
        self.pantalla.blit(self.globito,(XMAPAMAX,YGLOBITO))

    def correcto(self):
        self.pantalla.blit(self.nave[self.avanceNivel],(XNAVE,YNAVE))
        self.correctoActual = random.randint(1,self.numeroCorrecto)-1
        self.mostrarGlobito([self.listaCorrecto[self.correctoActual]])
        self.esCorrecto = True
        pygame.time.set_timer(EVENTORESPUESTA,TIEMPORESPUESTA)
        
    def mal(self):
        self.malActual = random.randint(1,self.numeroMal)-1
        self.mostrarGlobito([self.listaMal[self.malActual]])
        self.esCorrecto = False
        pygame.time.set_timer(EVENTORESPUESTA,TIEMPORESPUESTA)

    def explorarNombres(self):
        """Este es un jueguito point-and-click."""
        self.nivelActual = self.listaExploraciones[self.indiceNivelActual]
        # presentar nivel
        for i in self.nivelActual.dibujoInicial:
            if i.startswith("lineasDepto"):
                self.pantalla.blit(self.deptosLineas, (0, 0))
            elif i.startswith("rios"):
                self.pantalla.blit(self.rios, (0, 0))
            elif i.startswith("capitales"):
                for l in self.listaLugares:
                    if l.tipo == 1:
                        l.dibujar(self.pantalla,False)
            elif i.startswith("ciudades"):
                for l in self.listaLugares:
                    if l.tipo == 2:
                        l.dibujar(self.pantalla,False)
        for i in self.nivelActual.nombreInicial:
            if i.startswith("deptos"):
                for d in self.listaDeptos:
                    d.mostrarNombre(self.pantalla,self.fuente32,
                                    COLORNOMBREDEPTO,False)
            elif i.startswith("rios"):
                for d in self.listaRios:
                    d.mostrarNombre(self.pantalla,self.fuente32,
                                    COLORNOMBREDEPTO,False)
            elif i.startswith("capitales"):
                for l in self.listaLugares:
                    if l.tipo == 1:
                        l.mostrarNombre(self.pantalla,self.fuente24,
                                        COLORNOMBRECAPITAL,False)
            elif i.startswith("ciudades"):
                for l in self.listaLugares:
                    if l.tipo == 2:
                        l.mostrarNombre(self.pantalla,self.fuente24,
                                        COLORNOMBRECAPITAL,False)
        self.pantalla.fill((100,20,20),(975,26,200,48))
        self.mostrarTexto("Terminar",
                          self.fuente40,(1075,50),(255,155,155))
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    if event.key == 27: # escape
                        return
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if event.pos[0] < XMAPAMAX:
                        for i in self.nivelActual.elementosActivos:
                            if i.startswith("capitales"):
                                for l in self.listaLugares:
                                    if l.tipo == 1 and l.estaAca(event.pos):
                                        l.mostrarNombre(self.pantalla,
                                                        self.fuente24,
                                                        COLORNOMBRECAPITAL,
                                                        True)
                                        break
                            elif i.startswith("ciudades"):
                                for l in self.listaLugares:
                                    if l.tipo == 2 and l.estaAca(event.pos):
                                        l.mostrarNombre(self.pantalla,
                                                        self.fuente24,
                                                        COLORNOMBRECAPITAL,
                                                        True)
                                        break
                            elif i.startswith("rios"):
                                for d in self.listaRios:
                                    if d.estaAca(event.pos):
                                        d.mostrarNombre(self.pantalla,
                                                        self.fuente24,
                                                        COLORNOMBRERIO,
                                                        True)
                                        break
                            elif i.startswith("deptos"):
                                for d in self.listaDeptos:
                                    if d.estaAca(event.pos):
                                        d.mostrarNombre(self.pantalla,
                                                        self.fuente32,
                                                        COLORNOMBREDEPTO,
                                                        True)
                                        break
                    elif event.pos[0] > 975 and event.pos[0] < 1175 and \
                            event.pos[1] > 25 and event.pos[1] < 75:
                        return


    def jugarNivel(self):
        self.nivelActual = self.listaNiveles[self.indiceNivelActual]
        self.avanceNivel = 0
        self.nivelActual.prepararPreguntas()
        # presentar nivel
        for i in self.nivelActual.dibujoInicial:
            if i.startswith("lineasDepto"):
                self.pantalla.blit(self.deptosLineas, (0, 0))
            elif i.startswith("rios"):
                self.pantalla.blit(self.rios, (0, 0))
            elif i.startswith("capitales"):
                for l in self.listaLugares:
                    if l.tipo == 1:
                        l.dibujar(self.pantalla,False)
            elif i.startswith("ciudades"):
                for l in self.listaLugares:
                    if l.tipo == 2:
                        l.dibujar(self.pantalla,False)
        for i in self.nivelActual.nombreInicial:
            if i.startswith("deptos"):
                for d in self.listaDeptos:
                    d.mostrarNombre(self.pantalla,self.fuente32,
                                    COLORNOMBREDEPTO,False)
            elif i.startswith("capitales"):
                for l in self.listaLugares:
                    if l.tipo == 1:
                        l.mostrarNombre(self.pantalla,self.fuente24,
                                        COLORNOMBRECAPITAL,False)
            elif i.startswith("ciudades"):
                for l in self.listaLugares:
                    if l.tipo == 2:
                        l.mostrarNombre(self.pantalla,self.fuente24,
                                        COLORNOMBRECAPITAL,False)
        self.pantalla.fill((100,20,20),(975,26,200,48))
        self.mostrarTexto("Terminar",
                          self.fuente40,(1075,50),(255,155,155))
        pygame.display.flip()
        # presentar pregunta
        self.lineasPregunta = self.nivelActual.siguientePregunta(\
                self.listaSufijos,self.listaPrefijos)
        self.mostrarGlobito(self.lineasPregunta)
        # leer eventos y ver si la respuesta es correcta
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.KEYDOWN:
                    if event.key == 27: # escape
                        return
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if event.pos[0] < XMAPAMAX:
                        self.borrarGlobito()
                        if self.nivelActual.esCorrecta(self.listaDeptos,
                                                       self.listaLugares,
                                                       self.listaRios,
                                                       event.pos):
                            self.correcto()
                        else:
                            self.mal()
                    elif event.pos[0] > 975 and event.pos[0] < 1175 and \
                            event.pos[1] > 25 and event.pos[1] < 75:
                        return
                elif event.type == EVENTORESPUESTA:
                    pygame.time.set_timer(EVENTORESPUESTA,0)
                    if self.esCorrecto:
                        self.avanceNivel = self.avanceNivel + 1
                        if self.avanceNivel == TOTALAVANCE:
                            self.lineasPregunta =  self.listaDespedidas[\
                                random.randint(1,self.numeroDespedidas)-1]\
                                .split("\\")
                            self.mostrarGlobito(self.lineasPregunta)
                            self.yNave = YNAVE
                            self.fuego1 = True
                            pygame.time.set_timer(EVENTODESPEGUE,
                                                  TIEMPORESPUESTA*2)
                        else:
                            self.lineasPregunta = \
                                self.nivelActual.siguientePregunta(\
                                self.listaSufijos,self.listaPrefijos)
                            self.mostrarGlobito(self.lineasPregunta)
                    else:
                        self.mostrarGlobito(self.lineasPregunta)
                elif event.type == EVENTODESPEGUE:
                    if self.yNave == YNAVE:
                        self.pantalla.fill(COLORPANEL,
                                           (XBICHO,YBICHO,DXBICHO,DYBICHO))
                        self.pantalla.fill(COLORPANEL,
                                           (XMAPAMAX,0,DXPANEL,900))
                    self.pantalla.fill(COLORPANEL,
                                       (XNAVE,self.yNave,DXNAVE,DYNAVE+30))
                    self.yNave = self.yNave-8
                    if self.yNave<1:
                        pygame.time.set_timer(EVENTODESPEGUE,0)
                        return
                    else:
                        pygame.time.set_timer(EVENTODESPEGUE,TIEMPODESPEGUE)
                        self.pantalla.blit(self.nave[6],(XNAVE,self.yNave))
                        if self.fuego1:
                            self.pantalla.blit(self.fuego[0],
                                               (XNAVE+30,self.yNave+DYNAVE))
                        else:
                            self.pantalla.blit(self.fuego[1],
                                               (XNAVE+30,self.yNave+DYNAVE))
                        self.fuego1 = not self.fuego1
                        pygame.display.flip()

    def principal(self):
        """Esta es la parte principal del juego"""
        while 1:
            # pantalla inicial
            self.pantallaInicial()
            # dibujar fondo y panel
            self.pantalla.blit(self.fondo, (0, 0))
            self.pantalla.fill(COLORPANEL, (XMAPAMAX,0,DXPANEL,900))
            if self.jugar:
                self.pantalla.blit(self.bicho,(XBICHO,YBICHO))
            # mostrar pantalla
            pygame.display.flip()
            if self.jugar:
                self.jugarNivel()
            else:
                self.explorarNombres()


if __name__ == "__main__":
    juego = ConozcoUy()
    juego.principal()
#    juego.descubrirNombres()
#    juego.mostrarTodo()
