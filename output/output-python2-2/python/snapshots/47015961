from . import util


class InvalidValue(Exception):

    def __init__(self, message='', exception=None):
        self.message = message
        self.exception = exception
        Exception.__init__(self)

    def __str__(self):
        return "%s: %s" % (str(self.message), repr(self.exception))


class NotMapped(InvalidValue):

    def __init__(self, class_, exception=None):
        InvalidValue.__init__(self, 'Value class %s is not mapped' %
                              util.fullname(class_), exception)


class TableDoesNotExist(Exception):

    def __init__(self, tablename = '', exception=None):
        self.tablename = tablename
        self.exception = exception
        Exception.__init__(self)

    def __str__(self):
        return "%s: %s" % (str(self.tablename), repr(self.exception))
