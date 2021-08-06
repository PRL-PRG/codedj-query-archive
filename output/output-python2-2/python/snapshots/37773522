import lib.consts
import lib.debug
import gobject

def event(obj, event = None):
    """
        event(obj, event)
        or
        event(type), where type is one of ('load', )
    """
    def tmp(fnc):
        if not hasattr(fnc, 'events'):
            if lib.consts.DEBUG == True:
                def tmp2(*args, **kw_args):
                    try:
                        return fnc(*args, **kw_args)
                    except:
                        lib.debug.display_exc()
                        
                        raise
                fncx = tmp2
            else:
                fncx = fnc
            fncx.events = []
        else:
            fncx = fnc
        fncx.events.append((obj, event))
        return fncx
    
    return tmp
