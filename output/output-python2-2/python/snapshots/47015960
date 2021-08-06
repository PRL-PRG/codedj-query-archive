

def fullname(class_):
    return "%s.%s" % (class_.__module__, class_.__name__)


def fullname_underscore(class_):
    return fullname(class_).replace('.', '_')


def flatten(iterable, containers=(list, tuple)):
    iterable = list(iterable)
    i = 0
    while i < len(iterable):
        while isinstance(iterable[i], containers):
            if not iterable[i]:
                iterable.pop(i)
                i -= 1
                break
            else:
                iterable[i:i + 1] = (iterable[i])
        i += 1
    return iterable


def issubclass_(obj, type_):
    return isinstance(obj, type) and issubclass(obj, type_)
