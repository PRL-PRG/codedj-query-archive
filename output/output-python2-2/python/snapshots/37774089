import gobject

class CWidget(gobject.GObject):
    widgets = ()
    complexWidgets = ()
    name = ''
    
    def __init__(self, app, wTree):
        gobject.GObject.__init__(self)
        self.application = app
        for widgetName in self.widgets:
            setattr(self, widgetName, wTree.get_widget(widgetName))
        for widgetClass in self.complexWidgets:
            setattr(self, widgetClass.name, widgetClass(app, wTree))
        wTree.signal_autoconnect(self)
        self.Init()
    
    def Init(self):
        pass
