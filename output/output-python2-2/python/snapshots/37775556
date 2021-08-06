import lib.consts
import lib.debug
import gobject
from lib.Exceptions import *
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
                        Traceback.display_traceback(self.application)
                else:
                    try:
                        return fnc(self, *args, **kw_args)
                    except UserException:
                        Traceback.display_usr_exc()
                    except:
                        Traceback.display_traceback(self.application)

            fncx = tmp2
            #else:
                #fncx = fnc
            fncx.events = []
        else:
            fncx = fnc
        fncx.events.append((obj, event, params))
        return fncx

    return tmp
