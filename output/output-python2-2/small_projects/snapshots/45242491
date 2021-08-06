# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

"""Alan is a lazy guy. He doesn't care about too late or too early reviews."""

#class Inspector:
def assess(cardNode):
	# vérifier que les noeuds "review" sont bien classés par date croissante
	grade = 0
	for reviewNode in cardNode.getElementsByTagName("review"):
		if reviewNode.getAttribute('result') == 'good':
			grade += 1
		else:
			grade = 0
	return grade
