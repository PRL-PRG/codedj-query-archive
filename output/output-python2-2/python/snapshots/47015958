from .datatypes import guessdbdatatype, ForeignKey, Many, Unknown
from . import util


class Table(object):

    def __init__(self, class_=None, name=None, attributes=None, example=None,
                 primarykey=None):
        if class_ is not None:
            attributes = attributes or {}
            obj = example or class_()
            for attr in obj.__dict__:
                if attr[0] != '_' and not attr in attributes:
                    value = getattr(obj, attr)
                    if value == None:
                        attributes[attr] = None
                    else:
                        attributes[attr] = value.__class__

            self.name = name or class_.__name__
            self.class_ = class_
            self.classname = util.fullname(class_)
            self.columns, self.foreignkeys = self._generatecolumns(attributes)
            self.primarykey = primarykey or tuple(sorted(self.columns.keys()))

    def _generatecolumns(self, attributes):
        columns = {}
        foreignkeys = {}

        for (attr, value) in attributes.items():
            if isinstance(value, Column):
                self._addcolumn(columns, foreignkeys, attr, value)
            else:
                dbdatatype = guessdbdatatype(value)
                if dbdatatype:
                    self._addcolumn(columns, foreignkeys, attr,
                                    Column(attr, dbdatatype))

        return columns, foreignkeys

    def _addcolumn(self, columns, foreignkeys, attr, column):
        if isinstance(column.datatype, ForeignKey):
            foreignkeys[attr] = column

        if (not isinstance(column.datatype, Many) and
            not isinstance(column.datatype, Unknown)):
            columns[attr] = column


class Column(object):

    def __init__(self, name, datatype):
        self.name = name
        self.datatype = datatype

    def __repr__(self):
        return "%s('%s', %s)" % (util.fullname(self.__class__), self.name,
                                 self.datatype)
