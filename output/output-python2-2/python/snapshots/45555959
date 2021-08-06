"""
Plugin control
"""
import lister.plugins  # needed for getPlugins 
from zope import interface
from twisted import plugin

class IInputPlugin(interface.Interface):
    """
    An input plugin interface
    """
    name = interface.Attribute("Name of plugin")
    desc = interface.Attribute("Plugin's description")
    def __call__():
        """
        Returns the iterator
        """
        pass

class IOutputPlugin(interface.Interface):
    """
    An output plugin interface
    """
    name = interface.Attribute("Name of plugin")
    desc = interface.Attribute("Plugin's description")
    def __call__(xiter):
        """
        Iterates over xiter and prints it in some format
        """
        pass

class PluginWrapper(object):
    """
    Wrapper for making Twisted plugins for lister be easier
    """
    
    def __init__(self, name, action):
        """
        Making plugins to be twisted

        name - name of plugin
        action - actioner
        """
        self.name = name 
        self.action = action
        self.desc = action.__doc__
    
    def __call__(self):
        """
        Run the action
        """
        return self.action()


class InputPluginWrapper(PluginWrapper):
    """
    Wrapper for input plugins
    """
    interface.implements(plugin.IPlugin, IInputPlugin)

class OutputPluginWrapper(PluginWrapper):
    """
    Wrapper for output plugins
    """
    interface.implements(plugin.IPlugin, IOutputPlugin)

    def __call__(self, xiter):
        """
        Run the action for output plugin
        """
        return self.action(xiter)

def get_input_plugins(name=None):
    """
    Returns iterator over available input plugins

    name - show only plugins with such name, all if None
    """
    if name is None:
        res = plugin.getPlugins(IInputPlugin, lister.plugins)
    else:
        res = (p for p in get_input_plugins() if p.name == name)
    return res

def get_output_plugins(name=None):
    """
    Returns iterator over available output plugins

    name - show only plugins with such name, all if None
    """
    if name is None:
        res = plugin.getPlugins(IOutputPlugin, lister.plugins)
    else:
        res = (p for p in get_output_plugins() if p.name == name)
    return res
