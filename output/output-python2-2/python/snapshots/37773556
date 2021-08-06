import lib.consts
import lib.debug

def event(obj, event):
    def tmp(fnc):
        if lib.consts.DEBUG == True:
            def tmp2(*args, **kw_args):
                try:
                    fnc(*args, **kw_args)
                except:
                    lib.debug.display_exc()
            fncx = tmp2
        else:
            fncx = fnc
        
        if not hasattr(fncx, 'events'):
            fncx.events = []
        fncx.events.append((obj, event))
        return fncx
    
    return tmp
