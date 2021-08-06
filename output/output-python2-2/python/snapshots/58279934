
import os
import xml.etree.ElementTree
import urllib
import urlparse

tree = xml.etree.ElementTree.parse(os.path.expanduser("~/.recently-used.xbel"))
for bookmark in tree.getiterator("bookmark"):
    path = urlparse.urlsplit(bookmark.attrib["href"])[2]
    print urllib.unquote(path)

