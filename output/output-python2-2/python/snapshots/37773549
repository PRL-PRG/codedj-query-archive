import lib.consts
import lib.debug

def event(obj, event):
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
