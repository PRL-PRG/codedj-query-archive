# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

"""
Brian is a little more professional than Alan.
He don't validate reviews when it's too early...
but he doesn't care about lates.
"""

from time import strptime
from datetime import date
from datetime import timedelta

REVISION_DATES = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]

def assess(cardNode):
	# vérifier que les noeuds "review" sont bien classés par date croissante
	grade = 0
	lastRevisionDate = date(*strptime(cardNode.getAttribute('cdate') ,"%Y-%m-%d")[0:3])
	expectedRevisionDate = lastRevisionDate + timedelta(days=REVISION_DATES[grade]) 

	for reviewNode in cardNode.getElementsByTagName("review"):
		rdate = date(*strptime(reviewNode.getAttribute('rdate') ,"%Y-%m-%d")[0:3])
		if reviewNode.getAttribute('result') == 'good':
			if rdate >= expectedRevisionDate:
				# That's ok, we're not in advance.
				grade += 1
				lastRevisionDate = rdate
				expectedRevisionDate = lastRevisionDate + timedelta(days=REVISION_DATES[grade]) 
		else:
			grade = 0
			lastRevisionDate = rdate
			expectedRevisionDate = lastRevisionDate + timedelta(days=REVISION_DATES[grade]) 

	if date.today() >= expectedRevisionDate:
		# That's ok, we're not in advance.
		return grade
	else:
		# It's too early to review this card. The card will be hide.
		return -1
