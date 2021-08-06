#!/usr/bin/python
"""
Simple example of egg's entrypoints usage
"""

__all__ = ['input', 'output', 'plug', 'command']

import sys

# --- imports for listit_wo_plugins
from lister.input import dir_list
from lister.output import raw_list

# --- imports for command-line runner
from lister.command import runner


def listit(input_plugin, output_plugin):
    """
    Lists data using specified input and output plugins
    """
    output_plugin(input_plugin())


def listit_wo_plugins():
    """
    Lists data w/o plugin usage (example)
    """
    raw_list(dir_list())


def main():
    """
    Runs lister from command line
    """
    iplugin, oplugin = runner(sys.argv)
    listit(iplugin, oplugin)

if __name__ == '__main__':
    main()
