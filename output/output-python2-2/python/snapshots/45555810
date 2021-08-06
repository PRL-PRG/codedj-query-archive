#!/usr/bin/env python
# Helpers for getting object by it address
# Pythy <the.pythy@gmail.com>

import sys

def get_object(name):
    target_object = name
    target_object_list = target_object.split('.')
    target_object_package = '.'.join(target_object_list[:-2])
    target_object_module = '.'.join(target_object_list[:-1])
    target_object_single = target_object_list[-1]
    if target_object_package:
        object_module = __import__(target_object_module, 
                                   globals(), locals(), 
                                   target_object_package)
    else:
        object_module = __import__(target_object_module)
    object = getattr(object_module, target_object_single)
    return object


def get_arg(argv, usage="Usage: <module.py> path.to.target.object"):
    try:
        arg = argv[1]
    except IndexError:
        print usage
        sys.exit(1)
    return get_object(arg)
