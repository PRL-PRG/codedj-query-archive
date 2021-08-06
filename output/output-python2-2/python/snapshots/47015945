import inspect


def fullname(class_):
    """
    Returns the full name of the class.

    For example:

    >>> from datetime import datetime
    >>> fullname(datetime)
    'datetime.datetime'
    """

    return "%s.%s" % (class_.__module__, class_.__name__)


def fullname_underscore(class_):
    """
    Returns the full name of the class with underscores instead of dots.

    For example:

    >>> from datetime import datetime
    >>> fullname_underscore(datetime)
    'datetime_datetime'
    """

    return fullname(class_).replace('.', '_')


def flatten(iterable, containers=(list, tuple)):
    """
    Flattens an iterable with internal iterables:

    For example:

    >>> flatten([[1, 2]])
    [1, 2]
    >>> flatten([(1, 2), 3, [], 4])
    [1, 2, 3, 4]
    >>> flatten([1, [2, [3, [4]]]])
    [1, 2, 3, 4]
    >>> flatten([(1, 2)], (list,))
    [(1, 2)]
    >>> flatten([(1, 2)], (tuple,))
    [1, 2]
    """

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
    """
    Checks is obj is a class and is a subclass of type_.

    For example:

    >>> issubclass_("", str)
    False
    >>> class A: pass
    ...
    >>> class B(A): pass
    ...
    >>> issubclass_(B, A)
    True
    >>> issubclass_(A, B)
    False
    >>> issubclass_(A, A)
    True
    """
    return inspect.isclass(obj) and issubclass(obj, type_)


def getclass(obj):
    """
    Gets the class of obj or return obj if it is a class.

    For example:

    >>> getclass("")
    <type 'str'>
    >>> getclass(str)
    <type 'str'>
    """

    return inspect.isclass(obj) and obj or obj.__class__


def _test():
    import doctest
    doctest.testmod()


if __name__ == "__main__":
    _test()
