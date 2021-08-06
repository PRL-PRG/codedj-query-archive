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
                if hasattr(fnc, 'events'):
                    for event in fnc.events:
                        obj, event = event
                        if event is None:
                            if obj == 'load':
                                gobject.idle_add(fnc)
                        else:
                            events.setdefault(obj, []).append((event, fnc))
        self.application = app
        for widgetName in self.widgets:
            setattr(self, widgetName, wTree.get_widget(widgetName))
        for widgetClass in self.complexWidgets:
            setattr(self, widgetClass.name, widgetClass(app, wTree))
            
        for obj, oevents in events.iteritems():
            objtxt = obj.split(".")
            obj = getattr(self, objtxt[0])
            for attr in objtxt[1:]:
                try:
                    obj = getattr(obj, attr)
                except AttributeError:
                    obj = obj.get_property(attr)
            for event, fnc in oevents:
                obj.connect(event, fnc)
