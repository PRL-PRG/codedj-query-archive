import gobject

class CWidget(gobject.GObject):
    widgets = ()
    complexWidgets = ()
    name = ''
    
    def __init__(self, app, wTree):
        gobject.GObject.__init__(self)
        events = {}
        for fnc in dir(self):
            fnc = getattr(self, fnc)
            if callable(fnc):
                if hasattr(fnc, 'event'):
                    obj, event = fnc.event
                    events.setdefault(obj, []).append((event, fnc))
        self.application = app
        for widgetName in self.widgets:
            setattr(self, widgetName, wTree.get_widget(widgetName))
        for widgetClass in self.complexWidgets:
            setattr(self, widgetClass.name, widgetClass(app, wTree))
        #wTree.signal_autoconnect(self)
        for obj, oevents in events.iteritems():
            obj = getattr(self, obj)
            for event, fnc in oevents:
                obj.connect(event, fnc)
        self.Init()
    
    def Init(self):
        pass
