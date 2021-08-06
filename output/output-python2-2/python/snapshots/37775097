from lib.Depend.gtk2 import gobject

import lib.consts
import lib.debug
from lib.Exceptions import UserException
from lib.Gui.showExceptions import displayTraceback, displayUsrExc
def event(obj, *args):
    """
        event(obj, event)
        or
        event(type), where type is one of ('load', )
    """
    def tmp(fnc):
        if len(args) > 0:
            event = args[0]
            params = args[1:]
        else:
            event = None
            params = args
        if not hasattr(fnc, 'events'):

            def tmp2(self, *args, **kw_args):
                if lib.consts.DEBUG == True:
                    try:
                        return fnc(self, *args, **kw_args)
                    except :
                        if lib.consts.ERROR_TO_CONSOLE == True:
                            raise # reraise the exception
                        else: 
                            displayTraceback(self.application)
                        
                else:
                    try:
                        return fnc(self, *args, **kw_args)
                    except UserException:
                        displayUsrExc()
                    except:
                        displayTraceback(self.application)

            fncx = tmp2
            #else:
                #fncx = fnc
            fncx.events = []
        else:
            fncx = fnc
        fncx.events.append((obj, event, params))
        return fncx

    return tmp
