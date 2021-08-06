# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

revisionDates = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]

def assess(cardNode):
	# vérifier que les noeuds "review" sont bien classés par date croissante
	grade = 0
	for reviewNode in cardNode.getElementsByTagName("review"):
		if reviewNode.getAttribute('result') == 'good':
			grade += 1
		else:
			grade = 0
	return grade
