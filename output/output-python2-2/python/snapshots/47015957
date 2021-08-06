from __future__ import with_statement

from . import exceptions
from .metadata import Table
from .datatypes import LongText, PythonObject
from .query import Select
from .session import Session
import cryo

_TABLES_TABLE = Table(Table, name = '_cryo_tables',
                      attributes = {'name': LongText(),
                                    'classname': LongText(),
                                    'columns': PythonObject(),
                                    'foreignkeys': PythonObject(),
                                    'primarykey': PythonObject()})
_TABLES_TABLE.primarykey = ('name', )


class Connection(object):

    def __init__(self, backend):
        self.backend = backend
        self.tables = {}

    def readtables(self):
        try:
            self.tables[_TABLES_TABLE.classname] = _TABLES_TABLE
            with Session(self) as session:
                tables = dict([(table.classname, table)
                               for table in session.query(Select(Table))])
                for table in tables.values():
                    table.class_ = eval(table.classname, self.backend.modules)
                self.tables.update(tables)
        except exceptions.TableDoesNotExist:
            return False

        return True

    def inittables(self):
        self.createtables(_TABLES_TABLE)

    def createtables(self, *tables):
        with Session(self) as session:
            for table in tables:
                table = session.connectedbackend.createtable(table)
                self.tables[table.classname] = table


class Backend(object):

    def __init__(self, uri, modules = None):
        self.uri = uri
        self.modules = dict([(module.__name__, module)
                             for module in (modules or [])])
        self.modules[cryo.__name__] = cryo

    def newconnection(self):
        return Connection(self)

    def connect(self):
        pass


class ConnectedBackend(object):

    def __init__(self, backend, session):
        self.backend = backend
        self.session = session

    def gethashkey(self, obj):
        pass

    def getfullhashkey(self, obj):
        pass

    def createtable(self, table):
        pass

    def insert(self, *objs):
        pass

    def delete(self, *objs):
        pass

    def get(self, table, hashkey):
        pass

    def query(self, query):
        pass

    def commit(self):
        pass

    def rollback(self):
        pass

    def disconnect(self):
        pass
