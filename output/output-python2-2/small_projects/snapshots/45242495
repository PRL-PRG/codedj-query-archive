#!/usr/bin/env python
# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

"""OpenCAL's main file."""

import xml.dom.minidom
from Config import CARD_DATABASE
from Config import GUI

if GUI == 'GTK':
	from GtkGUI import GUI
elif GUI == 'HILDON':
	from HildonGUI import GUI

cardDatabaseFile = open(CARD_DATABASE, 'rU')
domDocument = xml.dom.minidom.parse(cardDatabaseFile)
cardDatabaseFile.close() 

from Reviewer import ReviewList
reviewList = ReviewList(domDocument)

gui = GUI(reviewList)
gui.main()
