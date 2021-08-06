import pickle

class Config:
    """
    	Clase genérica para el manejo de objetos persistentes
	tipo archivos de configuración.
	Autor: Antoni Aloy López
	Fecha: 4/8/2001
	Licencia : GNU

    """
    def __init__(self,arxiu):
        self.arxiu = arxiu
    def save(self):
        try:
            arx = open(self.arxiu,'w')
            pickle.dump(self,arx)
        except:
            print "Error grabando la configuración"

    def load(self):
        try:
            arx = open(self.arxiu,'r')
            self = pickle.load(arx)
        except:
            print "Arxiu de configuració no trobat"
            print "Passant a aparèmetres per defecte."

class ParametresEmpresa(Config):
	#Ejemplo 1 - Parámetros básicos de configuraicón de la empresa
    def __init__(self,arxiu):
        Config.__init__(self,arxiu)
        self.longitud = 8
        self.bloquejada ='NO'
        self.inici='01/01/2001'
        self.final='31/12/2001'
        
    def __repr__(self):
        return "Longitud   :"+ `self.longitud`+'\n'+(
               "Bloquejada : "+self.bloquejada +'\n')+(
               "Data inici : "+self.inici+'\n')+(
               "Data final :" +self.final)
    
class CofiguracioUsuari(Config):
	#Ejemplo 2
    def __init__(self,arxiu):
        Config.__init__(self,arxiu)
        self.nom ='noname'
        self.entrada='01:01:01'
        self.sortida='00:00:00'

	Config.__init__(self,arxiu)
	
	
	
if __name__=='__main__':
    config = Prova('/home/conta/config.txt')
    config.nom = 'Antoni Aloy'
    config.save()
    
    #print config
    #config.bloquejada ='NO'
    #config.load()
    #print config
