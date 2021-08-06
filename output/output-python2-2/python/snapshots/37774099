import gobject

class CWindow(gobject.GObject):
    widgets = ()
    complexWidgets = ()
    name = ''
    dont_delete = False
    glade = None
    
    def __init__(self, app, wTree):
        self.application = app
        for widgetName in self.widgets:
            setattr(self, widgetName, wTree.get_widget(widgetName))
        for widgetClass in self.complexWidgets:
            setattr(self, widgetClass.name, widgetClass(app, wTree))
        self.form = wTree.get_widget(self.name)
        wTree.signal_autoconnect(self)
        
        if self.dont_delete:
            self.form.connect('delete-event', self.__on_delete_event)
        
        self.Init()
    
    def Init(self):
        pass
    
    def Show(self):
        self.form.show()
    
    def Hide(self):
        self.form.hide()
    
    def SetParent(self, win):
        self.form.set_transient_for(win.form)
    
    def __on_delete_event(self, win, event):
        win.hide()
        return True
