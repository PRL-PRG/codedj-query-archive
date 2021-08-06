import gobject
from Widget import CWidget

class CWindow(CWidget):
    dont_delete = False
    
    def __init__(self, app, wTree):
        self.form = wTree.get_widget(self.name)
        CWidget.__init__(self, app, wTree)
        
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
