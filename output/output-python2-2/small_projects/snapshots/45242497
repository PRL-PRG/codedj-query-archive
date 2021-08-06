#!/usr/bin/env python
# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

# Ça semble plus difficile que prévu...
#   - http://mail.python.org/pipermail/xml-sig/2004-March/010191.html
#   - http://mail.python.org/pipermail/xml-sig/2004-March/010192.html

"""Export XML source form OpenCAL pkb file"""

import xml.dom.minidom
import sys
from Config import CARD_DATABASE

# Manage command line arguments
INDENT = False
if len(sys.argv) > 1:
	if sys.argv[1] == "--indent" or sys.argv[1] == "-i":
		INDENT = True
	else:
		print "Usage: exportxml [OPTION]"
		print "Export XML source form OpenCAL pkb file.\n"
		print "  --indent, -i   Print indented XML source\n"
		sys.exit()

# Parse the document
cardDatabaseFile = open(CARD_DATABASE, 'rU')
domDocument = xml.dom.minidom.parse(cardDatabaseFile)
cardDatabaseFile.close() 

# Delete useless pkb's TextNodes
rootElement = domDocument.documentElement
for rootChildNode in rootElement.childNodes:
	if rootChildNode.nodeType == rootChildNode.TEXT_NODE:
		rootElement.removeChild(rootChildNode)

#for cardNode in domDocument.getElementsByTagName("card"):
#	print "\n-----------------------------\n"
#	for childNode in cardNode.childNodes:
#		print childNode.nodeName
#	print cardNode.toxml()

# Delete useless card's TextNodes
for cardNode in domDocument.getElementsByTagName("card"):
	for cardChildNode in cardNode.childNodes:
		if cardChildNode.nodeType == cardChildNode.TEXT_NODE:
			cardNode.removeChild(cardChildNode)

# Print the document
if INDENT == True:
	print domDocument.toprettyxml() 
else:
	print domDocument.toxml() 
