#!/usr/bin/env python
#-*- coding: utf-8 -*-
#

import os
import sys
import os.path

class DevClientException(Exception):
    pass

class ClassNotFoundException(DevClientException):
    pass

class Loader(object):
    def __init__(self, config):
        self.__config = config

    def _loadModules(self):
        module_path = self.__config['module_path']
        sys.path.append(os.path.join(os.getcwd(), module_path))

        modules = os.listdir(module_path)
        modules = [os.path.splitext(x)[0] for x in modules \
                   if os.path.isfile(os.path.join(module_path, x))]

        self.modules = set(modules)

    def _loadClasses(self, classes):

        def findParent(element, level):
            for parent in element.__bases__:
                if parent.__name__ in classes:
                    return (parent.__name__, level)
                elif len(parent.__bases__) > 0:
                    base, level = findParent(parent, level + 1)
                    if level != -1:
                        return (base,level)

            return ('', -1)

        cdict = dict(zip(classes, [[None, 0] for x in xrange(len(classes))]))
        for src in self.modules:
            __import__(src)
            for el, val in sys.modules[src].__dict__.iteritems():
                if type(val).__name__ in ['type','classobj']:
                    if el in classes and cdict[el][1] == 0:
                        cdict[el] = [val, 0]
                    else:
                        base, level = findParent(val, 1)
                        if level != -1 and cdict[base][1] < level:
                            cdict[base] = [val, level]

        classes_found = {}
        for el in classes:
            if not cdict[el][0]:
                raise ClassNotFoundException()
            classes_found[el] = cdict[el][0]

        return classes_found

    def load(self, classes):
        self._loadModules()
        return self._loadClasses(classes)

if __name__ == '__main__':
    config = {'module_path': 'modules'}
    classes = ['AbstractFactory']

    loader = Loader(config)
    classes =  loader.load(classes)

    print classes




