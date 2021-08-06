import gobject
from Widget import CWidget

class CWindow(CWidget):
    dont_delete = False
    glade = None
    
    def __init__(self, app, wTree):
        self.form = wTree.get_widget(self.name)
        CWidget.__init__(self, app, wTree)
        """
        events = {}
        for fnc in dir(self):
            fnc = getattr(self, fnc)
            if callable(fnc):
                if hasattr(fnc, 'events'):
                    for event in fnc.events:
                        obj, event = event
                        events.setdefault(obj, []).append((event, fnc))
        self.application = app
        for widgetName in self.widgets:
            setattr(self, widgetName, wTree.get_widget(widgetName))
        for widgetClass in self.complexWidgets:
            setattr(self, widgetClass.name, widgetClass(app, wTree))
        # wTree.signal_autoconnect(self)
        for obj, oevents in events.iteritems():
            obj = getattr(self, obj)
            for event, fnc in oevents:
                obj.connect(event, fnc)
        """
        
        if self.dont_delete:
            self.form.connect('delete-event', self.__on_delete_event)
        
    
    def Show(self):
        self.form.show()
    
    def Hide(self):
        self.form.hide()
    
    def SetParent(self, win):
        self.form.set_transient_for(win.form)
    
    def __on_delete_event(self, win, event):
        win.hide()
        return True
