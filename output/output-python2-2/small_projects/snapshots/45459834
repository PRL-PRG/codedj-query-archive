class Event:

    def __init__(self):
        self.callbacks = set() 

    def __iadd__(self, callback):
        self.callbacks.add(callback)
        return self

    def __isub__(self, callback):
        if callback in self.callbacks:
            self.callbacks.remove(callback)
        return self

    def __call__(self, *a, **kwds):
        for callback in self.callbacks:
            callback(*a, **kwds)

