import sqlite3
import pickle
import base64

from .standardsql import StandardSQLBackend, StandardSQLConnectedBackend
from .. import util
from .. import exceptions
from .. import datatypes


def _unwrap(value):
    if not isinstance(value, StandardSQLConnectedBackend.Wrapper):
        return value
    elif isinstance(value.column.datatype, datatypes.Enum):
        return value.value.index
    elif isinstance(value.column.datatype, datatypes.PythonObject):
        return base64.encodestring(pickle.dumps(value.value, 2))
    if isinstance(value.column.datatype, datatypes.ForeignKey):
        return value.connectedbackend.gethashkey(value.value)
    else:
        return value.value


def _convert(class_, connectedbackend):

    def convert(value):
        if issubclass(class_, datatypes.PythonObject):
            return pickle.loads(base64.decodestring(value))
        elif issubclass(class_, datatypes.One):
            # TODO: handle foreign keys
            return value
        elif issubclass(class_, datatypes.Many):
            # TODO: handle foreign keys
            return []
        else:
            return value

    return convert


class SQLiteBackend(StandardSQLBackend):

    def __init__(self, uri, modules=None):
        StandardSQLBackend.__init__(self, uri, modules)
        for attr in dir(datatypes):
            value = getattr(datatypes, attr)
            if isinstance(value, type):
                sqlite3.register_converter(util.fullname_underscore(value),
                                           _convert(value, self))

    def connect(self, session):
        return SQLiteConnectedBackend(self, session)


class SQLiteConnectedBackend(StandardSQLConnectedBackend):

    def __init__(self, backend, session):
        StandardSQLConnectedBackend.__init__(self, backend, session)
        self.connection = sqlite3.connect(backend.uri,
                                          detect_types=sqlite3.PARSE_COLNAMES)
        self.connection.row_factory = sqlite3.Row
        self.cursor = self.connection.cursor()

    def createtable(self, table):
        query = self._createtable(table)
        self.cursor.execute(query)
        self.insert(table)
        return table

    def gettype(self, datatype):
        if isinstance(datatype, datatypes.PythonObject):
            return 'text'
        elif isinstance(datatype, datatypes.Number):
            return 'number(%s, %s)' % (datatype.length, datatype.decimals)
        elif isinstance(datatype, datatypes.Timestamp):
            return 'timestamp'
        elif isinstance(datatype, datatypes.ForeignKey):
            return 'integer'

    def insert(self, *objs):
        for query, values in self._insert(*objs):
            self.cursor.execute(query, [_unwrap(value) for value in values])

    def delete(self, *objs):
        for query, values in self._delete(*objs):
            self.cursor.execute(query, [_unwrap(value) for value in values])

    def query(self, select):
        table = self.session.connection.tables[select.classname]

        columns = ["'%s'.'%s' AS '%s [%s]'" %
                   (table.name, column.name, column.name,
                    util.fullname_underscore(column.datatype.__class__))
                   for (name, column) in table.columns.items()]

        query, values = self._query(select, columns=columns)

        try:
            results = self.cursor.execute(query, [_unwrap(value)
                                                  for value in values])
            for result in results:
                obj = select.constructor()
                for (name, column) in table.columns.items():
                    if isinstance(column.datatype, datatypes.One):
                        # TODO: check for autofetch
                        setattr(obj, name,
                                self.session.get(column.datatype.class_,
                                                 result[column.name]))
                    elif isinstance(column.datatype, datatypes.Many):
                        # TODO: get collection or set proxy
                        pass
                    else:
                        setattr(obj, name, result[column.name])
                yield obj

        except sqlite3.OperationalError, e:
            raise exceptions.TableDoesNotExist(table.name, e)

    def commit(self):
        self.connection.commit()

    def rollback(self):
        self.connection.rollback()

    def disconnect(self):
        self.cursor = None
        self.connection = None
