import sys
from zope.interface import implements
from twisted.plugin import IPlugin
from lister.plug import IInputPlugin


class SysPathLister(object):
    implements(IInputPlugin, IPlugin)
    name = "syspath"
    desc = """
    Lists sys.path
    """
    def __call__(self):
        return sys.path

syspath_list_plugin = SysPathLister()
