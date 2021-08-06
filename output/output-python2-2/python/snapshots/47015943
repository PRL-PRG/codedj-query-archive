from datetime import datetime

import enum

from . import util


class Datatype(object):

    def __init__(self):
        pass

    def __repr__(self):
        return "%s()" % (util.fullname(self.__class__))


class Unknown(Datatype):

    def __init__(self):
        Datatype.__init__(self)


class LongText(Datatype):

    def __init__(self):
        Datatype.__init__(self)


class Text(Datatype):

    def __init__(self, length):
        self.length = length
        Datatype.__init__(self)

    def __repr__(self):
        return "%s(%s)" % (util.fullname(self.__class__), self.length)


class Number(Datatype):

    def __init__(self, length, decimals=0):
        self.length = length
        self.decimals = decimals
        Datatype.__init__(self)

    def __repr__(self):
        if self.decimals:
            return "%s(%s, %s)" % (util.fullname(self.__class__), self.length,
                                   self.decimals)
        elif self.length > 1:
            return "%s(%s)" % (util.fullname(self.__class__), self.length)
        else:
            return Datatype.__repr__(self)


class Boolean(Number):

    def __init__(self):
        Number.__init__(self, 1)


class Timestamp(Datatype):

    def __init__(self):
        Datatype.__init__(self)


class ForeignKey(Datatype):

    def __init__(self, class_, reverse, autofetch):
        self.class_ = class_
        self.classname = util.fullname(class_)
        self.reverse = reverse
        self.autofetch = autofetch
        Datatype.__init__(self)

    def __repr__(self):
        if self.reverse is None:
            return "%s(%s)" % (util.fullname(self.__class__), self.classname)
        else:
            return "%s(%s, '%s')" % (util.fullname(self.__class__),
                                     self.classname, self.reverse)


class One(ForeignKey):

    def __init__(self, class_, reverse=None, autofetch=True):
        ForeignKey.__init__(self, class_, reverse, autofetch)


class Many(ForeignKey):

    def __init__(self, class_, reverse=None, autofetch=False):
        ForeignKey.__init__(self, class_, reverse, autofetch)


class PythonObject(Datatype):

    def __init__(self):
        Datatype.__init__(self)


class Enum(Datatype):

    def __init__(self, class_):
        Datatype.__init__(self)
        self.class_ = class_
        self.classname = util.fullname(class_)


def guessdbdatatype(value):
    if value == None:
        return None
    elif isinstance(value, Datatype):
        return value
    elif util.issubclass_(value, Datatype):
        return value()
    elif _issubclassorinstance(value, str):
        return LongText()
    elif _issubclassorinstance(value, bool):
        return Boolean()
    elif _issubclassorinstance(value, float):
        return Number(10, 5)
    elif _issubclassorinstance(value, long):
        return Number(20)
    elif _issubclassorinstance(value, int):
        return Number(10)
    elif _issubclassorinstance(value, datetime):
        return Timestamp()
    elif _issubclassorinstance(value, enum.Enum):
        return Enum()
    else:
        return Unknown()


def _issubclassorinstance(value, class_):
    return util.issubclass_(value, class_) or isinstance(value, class_)
