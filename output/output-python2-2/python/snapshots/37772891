import lib.consts
import lib.debug
import gobject

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
        fncx.events.append((obj, event, params))
        return fncx
    
    return tmp
