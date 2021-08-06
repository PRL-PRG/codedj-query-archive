from Tkinter import *
from menuAction import MenuAction


class AppMenu:
    def __init__(self,parent):
	self.parent = parent
	mBar = Frame(parent, relief=GROOVE, borderwidth=2)
	self.__action = MenuAction(mBar)
	mBar.tk_menuBar(self.mnuEmpresa(mBar), self.mnuFicheros(mBar), self.mnuContabilidad(mBar),
                        self.mnuListados(mBar), self.mnuUtilidades(mBar))
	mBar.pack(fill=X)
        
	
    def salir(self):
	self.parent.quit()
	
    def mnuEmpresa(self,mBar):
	CmdBtn = Menubutton(mBar, text='Empresa',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu=Menu(CmdBtn)
 	CmdBtn.menu.add_command(label='Seleccionar empresa',command=self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Mantenimiento de empresas', underline=0,command=self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Usuarios',underline=0, command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Cierre',underline=0, command= self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Salir',underline=0, command = self.salir)
	CmdBtn['menu']=CmdBtn.menu
	return CmdBtn

    def mnuFicheros(self,mBar):
	CmdBtn = Menubutton(mBar, text='Ficheros',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu = Menu(CmdBtn)
	CmdBtn.menu.add_command(label='Plan de cuentas', command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Saldos iniciales', command= self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Conceptos contables', command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Conceptos IVA', command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Diarios', command=self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Grupo de empresas', command=self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Medios de pago', command=self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Centros de coste', command= self.__action.noImplementado)
	CmdBtn['menu']=CmdBtn.menu
	return CmdBtn

    def mnuContabilidad(self, mBar):
	CmdBtn = Menubutton(mBar, text='Contabilidad',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu = Menu(CmdBtn)
	CmdBtn.menu.add_command(label='Borrador...', command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Pagos...', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Cobros...', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Cajas', command= self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Modificar asiento', command= self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Conciliación', command=self.__action.noImplementado)
	CmdBtn['menu']=CmdBtn.menu
	return CmdBtn

    def mnuListados(self, mBar):
        CmdBtn = Menubutton(mBar, text='Listados',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu = Menu(CmdBtn)
	CmdBtn.menu.add_command(label='Plan de cuentas', command= self.__action.noImplementado)
	CmdBtn.menu.add_command(label='Extracto de cuentas', command= self.__action.noImplementado)
        CmdBtn.menu.add('separator')
        CmdBtn.menu.add_command(label='Diario', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Mayor', command= self.__action.noImplementado)
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Resumen anual', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Saldos mensuales', command= self.__action.noImplementado) 
	CmdBtn.menu.add('separator')
	CmdBtn.menu.add_command(label='Balances de comprobación', command=self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Balances de definidos', command=self.__action.noImplementado)
        CmdBtn.menu.add('separator')
        CmdBtn.menu.add_command(label='Resumen 347', command=self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Declaración IVA/IRPF', command=self.__action.noImplementado)
	CmdBtn['menu']=CmdBtn.menu
	return CmdBtn 

    def mnuUtilidades(self, mBar):
        CmdBtn = Menubutton(mBar, text='Utilidades',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu = Menu(CmdBtn)
	CmdBtn.menu.add_command(label='Liberar borrador', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Eliminar borrador', command= self.__action.noImplementado)
        CmdBtn.menu.add('separator')
        CmdBtn.menu.add_command(label='Traspasar apuntes...', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Cambion de moneda', command= self.__action.noImplementado)
        CmdBtn.menu.add('separator')
        CmdBtn.menu.add_command(label='Exportar tablas ...', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Copia de seguridad ...', command = self.__action.noImplementado)
        CmdBtn['menu']=CmdBtn.menu                               
	return CmdBtn
    def mnuAyuda(self, mBar):
        CmdBtn = Menubutton(mBar, text='Ayuda',underline=0)
	CmdBtn.pack(side=LEFT, padx="2m")
	CmdBtn.menu = Menu(CmdBtn)
	CmdBtn.menu.add_command(label='Manual', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Sobre BulConta ...', command= self.__action.noImplementado)
        CmdBtn.menu.add('separator')
        CmdBtn.menu.add_command(label='Informar de un error', command= self.__action.noImplementado)
        CmdBtn.menu.add_command(label='Actualizaciones', command= self.__action.noImplementado)
        CmdBtn['menu']=CmdBtn.menu                               
	return CmdBtn
    
if __name__=='__main__':
    root = Tk()
    root.title('BulConta')
    # Aquí cream el menú de l'apicació
    Menu = AppMenu(root)
    root.mainloop()
