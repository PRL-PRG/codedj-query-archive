#!/usr/bin/python

import xml.etree.ElementTree
import os
import sys

try:
    register_namespace = xml.etree.ElementTree.register_namespace
except AttributeError:
    def register_namespace(prefix, uri):
        xml.etree.ElementTree._namespace_map[uri] = prefix


namespaces = [
    ("bookmark", "http://www.freedesktop.org/standards/desktop-bookmarks"),
    ("mime", "http://www.freedesktop.org/standards/shared-mime-info")
]

for namespace in namespaces:
    register_namespace(*namespace)

recently_used_filepath = os.path.expanduser("~/.recently-used.xbel")

tree = xml.etree.ElementTree.parse(open(recently_used_filepath))

dirpath = sys.argv[1]

for bookmark in tree.getroot().findall("bookmark"):
    if bookmark.get("href").startswith("file://%s" % dirpath):
        tree.getroot().remove(bookmark)

for prefix, uri in namespaces:
    tree.getroot().set("xmlns:%s" % prefix, uri)

tree.write(open(recently_used_filepath, "w"))
#tree.write(sys.stdout)

