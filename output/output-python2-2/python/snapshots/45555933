#!/usr/bin/env python
# references.py: store persistent references
# Pythy <the.pythy@gmail.com>

from ZODB import config
from BTrees import IOBTree, OOBTree
from persistent import Persistent

class DataStorage(object):
    """Data storage (ZODB) """
    def __init__(self):
        self.db = config.databaseFromURL('references.conf')
        self.conn = self.db.open()
        self.root = self.conn.root()
    
    def finish(self):
        self.conn.close()
        self.db.close()

class Reference(DataStorage):
    """Reference"""
    def __init__(self, referenceName, bTreeType):
        DataStorage.__init__(self)
        refname = "ref_%s" % referenceName
        if refname not in self.root.keys():
            self.root[refname] = bTreeType()
        self.refname = refname
        self.ref = self.root[refname]
    
    def __getitem__(self, key):
        return self.ref[key]
    
    def __setitem__(self, key, value):
        self.ref[key] = value

    def delete(self):
        del(self.root[self.refname])
        self.ref = None

class ReferenceById(Reference):
    """Reference where keys are integers"""
    def __init__(self, referenceName):
        Reference.__init__(self, referenceName, IOBTree.IOBTree)

class ReferenceByVal(Reference):
    """Reference where keys are some objects"""
    def __init__(self, referenceName):
        Reference.__init__(self, referenceName, OOBTree.OOBTree)

class ReferenceItem(Persistent):
    """Item of reference"""
    def __init__(self, name):
        self.name = name
    
    def __repr__(self):
        return self.name
    
    def __str__(self):
        return self.name
    
    def __getitem__(self, item):
        return getattr(self, item)
    
    def __setitem__(self, item, value):
        setattr(self, item, value)
