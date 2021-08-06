from types import TupleType

from . import util


class Query(object):

    def __init__(self):
        pass


class Select(Query):

    def __init__(self, class_, constructor=None):
        self.constructor = constructor or class_
        self.classname = util.fullname(class_)
        self.whereclause = None
        self.orderbyclauses = []
        self.limitclause = None
        Query.__init__(self)

    def where(self, value=None, *args):
        self.whereclause = args and CompareWhereClause(value, *args) or value
        return self

    def and_(self, value=None, *args):
        whereclause = args and CompareWhereClause(value, *args) or value
        self.whereclause = AndWhereClause(self.whereclause, whereclause)
        return self

    def or_(self, value=None, *args):
        whereclause = args and CompareWhereClause(value, *args) or value
        self.whereclause = OrWhereClause(self.whereclause, whereclause)
        return self

    def orderby(self, *orderbyclauses):
        for orderbyclause in orderbyclauses:
            if type(orderbyclause) is OrderByClause:
                self.orderbyclauses.append(orderbyclause)
            elif type(orderbyclause) is TupleType:
                self.orderbyclauses.append(OrderByClause(orderbyclause[0],
                                                         orderbyclause[1]))
            else:
                self.orderbyclauses.append(OrderByClause(orderbyclause))
        return self

    def limit(self, limit=None):
        self.limitclause = LimitClause(limit)
        return self


class Field:

    def __init__(self, name):
        self.name = name


class WhereClause(object):

    def __init__(self):
        pass


class CompareWhereClause(WhereClause):

    def __init__(self, value1, comparator, value2):
        self.value1 = value1
        self.value2 = value2
        self.comparator = comparator
        WhereClause.__init__(self)


class AndWhereClause(WhereClause):

    def __init__(self, whereclause1, whereclause2):
        self.whereclause1 = whereclause1
        self.whereclause2 = whereclause2
        WhereClause.__init__(self)


class OrWhereClause(WhereClause):

    def __init__(self, whereclause1, whereclause2):
        self.whereclause1 = whereclause1
        self.whereclause2 = whereclause2
        WhereClause.__init__(self)


class OrderByClause(object):

    def __init__(self, field, ascending=True):
        self.field = field
        self.ascending = ascending


class LimitClause(object):

    def __init__(self, start=None, end=None):
        self.start = start
        self.end = end
