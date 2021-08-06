#!/usr/bin/env python

from __future__ import with_statement
from __future__ import absolute_import
from __future__ import division
from __future__ import generators
from __future__ import nested_scopes

from datetime import datetime

import psyco
psyco.full()

from simpyper.session import Session
from simpyper.metadata import Table, Column
from simpyper.datatypes import Text, One, Many
from simpyper.backends.sqlite import SQLiteBackend
from simpyper.query import Select, Field

class A:

    def __init__(self, name="", description=""):
        self.name = name
        self.description = description
        self.excluded = None


class B:

    def __init__(self, name="", a=None, status1="", status2=""):
        self.name = name
        self.a = a
        self.cs = list()
        self.status1 = status1
        self.status2 = status2


class C:

    def __init__(self, name=""):
        self.name = name
        self.excluded = True
        self.boolean = False
        self.integer = 1
        self.decimal = 1.1
        self.long = 1L
        self.datetime = datetime.now()


class D:

    def __init__(self, name=""):
        self.name = name
        self.a = None
        self.bs = []


def main():
    test(init())


def init():
    connection = SQLiteBackend("example.sqlite", modules=[__import__(__name__)]).newconnection()

    if not connection.readtables():
        connection.inittables()

        #########################
        # A
        a_table = Table(A, primarykey=('name',))

        #########################
        # B
        b_table = Table(B,
                        attributes={'name': Column('b_name', Text(2)),
                                    'a': One(A),
                                    'cs': Many(C),
                                    'status1': str,
                                    'status2': ""},
                        primarykey=('name', 'a'))

        #########################
        # C
        c_table = Table(C, name='table_c', attributes={'excluded': None},
                        primarykey=('name',))
        c_table.columns['name'].name = 'c_name'

        #########################
        # D
        d = D()
        d.a = A()
        d.bs = [B()]
        d_table = Table(D, example=d, primarykey=('name', ))

        #########################
        # CREATE TABLES
        connection.createtables(a_table, b_table, c_table, d_table)

    return connection


def test(connection):
    with Session(connection) as session:
        a1 = A("a1", "one")
        a2 = A("a2", "two")

        assert a1 not in session
        assert a2 not in session

        a3_1 = A("a3", "three A")
        a3_2 = A("a3", "three B")

        assert session.same(a3_1, a3_2)

        session.add(a1)
        session.add(a2)
        session.add(a3_1)
        session.add(a3_2)

        assert a1 in session
        assert a2 in session
        
        del session[a3_1]

        assert a1 in session
        assert a2 in session
        assert a3_1 not in session
        assert a3_2 not in session

        b1 = B("b1", a1, "ok", "good")

        session.add(b1)

    with Session(connection) as session:
        a = session.queryone(Select(A).where(Field("name"), "LIKE", "a1")
                             .orderby('name').limit(1))

        objsB = session.query(Select(B))
        for obj in objsB:
            assert session.same(a, obj.a)


    with Session(connection) as session:
        for n in range(12500):
            session.add(A("a%i" % n, "generated"))

if __name__ == '__main__':
    main()
