"""
Plugin control
"""

import pkg_resources


def get_plugins_by_entrypoint(ep_name, plug_name=None):
    """
    Returns iterator over names, descriptions and plugin-actions 
    for current entrypoint
    """
    for entrypoint in pkg_resources.iter_entry_points(ep_name, plug_name):
        plugin_func = entrypoint.load()
        plugin_description = plugin_func.__doc__
        yield (entrypoint.name, plugin_description, plugin_func)


def get_input_plugins(plug_name=None):
    """
    Returns iterator over input plugins 
    (similar to  get_plugins_by_entrypoint)
    """
    return get_plugins_by_entrypoint("lister.input", plug_name)


def get_output_plugins(plug_name=None):
    """
    Returns iterator over output plugins 
    (similar to  get_plugins_by_entrypoint)
    """
    return get_plugins_by_entrypoint("lister.output", plug_name)
