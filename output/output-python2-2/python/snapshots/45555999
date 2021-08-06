"""
Command line interface for Lister
"""

import getopt
import sys

from lister import plug


def usage(msg=""):
    """
    Shows usage information
    """
    print msg
    print """
lister [options]

Options:
    -h, --help      Help
    -i, --input     Input plugin name
    -o, --output    Output plugin name
    -l, --list      List available plugins
    """
    sys.exit(1)


def get_plugins_info(plugins_type):
    """
    Returns info (name, description) 
    for current plugin type (input or output)
    """
    assert plugins_type in ('input', 'output')
    entrypoint = "lister.%s" % plugins_type
    for p_name, p_desc, p_action in plug.get_plugins_by_entrypoint(entrypoint):
        yield "%s\t\t%s" % (p_name, p_desc)


def list_plugins():
    """
    Prints input/output plugins
    """
    print "Input plugins:"
    print "\n".join(get_plugins_info('input'))
    print "Output plugins:"
    print "\n".join(get_plugins_info('output'))
    sys.exit(1)


def runner(argv):
    """
    Parses command-line options
    """
    # defaults    
    iplugin = 'dir'
    oplugin = 'raw'
    try:
        opts, args = getopt.getopt(argv[1:], 
                     "hi:o:l", 
                     ["help", "input=", "output=", "list"])
    except getopt.error, msg:
        usage()
    for o, a in opts:
        if o in ("-h", "--help"):
            usage()
        if o in ("-l", "--list"):
            list_plugins()
        if o in ("-i", "--input"):
            iplugin = a
        if o in ("-o", "--output"):
            oplugin = a
    
    iplugins_data = list(plug.get_input_plugins(iplugin))
    if not iplugins_data:
        print "No such input plugin: %s" % iplugin
        sys.exit(1)
    else:
        input_plugin = iplugins_data[0][-1]

    oplugins_data = list(plug.get_output_plugins(oplugin))
    if not oplugins_data:
        print "No such output plugin: %s" % oplugin
        sys.exit(1)
    else:
        output_plugin = oplugins_data[0][-1]
    
    return input_plugin, output_plugin
