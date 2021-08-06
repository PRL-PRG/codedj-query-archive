import sys
from lib.Gui.dialogs import CExceptionDialog
from lib.Gui.frmException import CfrmException


def displayTraceback(app):
    win = app.GetWindow('frmException')
    win.SetParent(app.GetWindow('frmMain'))
    win.SetErrorLog()
    win.SetSystemInfo()
    win.project = app.GetProject()
    win.Show()

def displayUsrExc():
    exctype, value = sys.exc_info()[:2] 
    text = _('An exception has occured:')+ '\n\n<b>'+exctype.__name__ +':</b> '+ str(value)
    CExceptionDialog(None, text).run()