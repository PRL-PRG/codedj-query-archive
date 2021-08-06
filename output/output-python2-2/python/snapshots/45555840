"""
Output 'HTML' plugin for lister
"""

from lister import plug

def html_list(ilist):
    """
    Prints list as HTML unordered list
    """
    print "<ul>"
    for element in ilist:
        print "<li>%s</li>" % element
    print "</ul>"

html_list_plugin = plug.OutputPluginWrapper('html', html_list)
