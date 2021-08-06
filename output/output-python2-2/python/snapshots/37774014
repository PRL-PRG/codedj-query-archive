def event(obj, event):
    def tmp(fnc):
        if not hasattr(fnc, 'events'):
            fnc.events = []
        fnc.events.append((obj, event))
        return fnc
    return tmp
