# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

from time import strftime, localtime
from Config import CARD_DATABASE
from Config import INSPECTOR

if INSPECTOR == 'Alan':
	from InspectorAlan import assess
else:
	from InspectorBrian import assess

################
# CardToReview #
################

class CardToReview:
	def __init__(self, domDocument, cardNode):
		self.domDocument = domDocument
		self.cardNode = cardNode
		self.grade = assess(cardNode)
		
	def getQuestion(self):
		questionList = self.cardNode.getElementsByTagName("question")
		question =  questionList.item(0).firstChild.data
		return question
		
	def getAnswer(self):
		answerList = self.cardNode.getElementsByTagName("answer")
		answer =  answerList.item(0).firstChild.data
		return answer
		
	def getGrade(self):
		return self.grade
		
	def putReview(self, result):
		reviewElement = self.domDocument.createElement('review')
		reviewElement.setAttribute('rdate', strftime('%Y-%m-%d', localtime()))
		reviewElement.setAttribute('result', result)
		self.cardNode.appendChild(reviewElement)
		# Chercher de la doc sur writexml() (cf. http://docs.python.org/lib/dom-objects.html)
		cardDatabaseFile = open(CARD_DATABASE, 'wU')
		cardDatabaseFile.write(self.domDocument.toxml("utf-8"))
		cardDatabaseFile.close() 

##############
# ReviewList #
##############

class ReviewList:
	def __init__(self, domDocument):
		self.domDocument = domDocument
		self.reviewList = []
		self.pointer = 0
		for cardNode in self.domDocument.getElementsByTagName("card"):
			cardToReview = CardToReview(self.domDocument, cardNode)
			if cardToReview.getGrade() >= 0:
				self.reviewList.append(cardToReview )
		self.reviewList.sort(self.sortCards)

	def sortCards(self, e1, e2):
		# return 1 means e1 > e2
		if e1.getGrade() > e2.getGrade():
			return 1
		# return -1 means e1 < e2
		elif e1.getGrade() < e2.getGrade():
			return -1
		# return 0 means e1 == e2
		else:
			return 0

	def firstCard(self):
		self.pointer = 0
		return self.reviewList[self.pointer]

	def lastCard(self):
		self.pointer = len(self.reviewList) - 1
		return self.reviewList[self.pointer]

	def previousCard(self):
		if self.hasPreviousCard():
			self.pointer -= 1
		return self.reviewList[self.pointer]

	def nextCard(self):
		if self.hasNextCard():
			self.pointer += 1
		return self.reviewList[self.pointer]

	def hasPreviousCard(self):
		if self.pointer > 0:
			return True
		else:
			return False

	def hasNextCard(self):
		if self.pointer < len(self.reviewList) - 1:
			return True
		else:
			return False

	def getReviewedCards(self):
		# TODO : Attention, ça n'a rien à faire là, ça c une autre liste !
		return 0
		
	def getRemainingCards(self):
		return len(self.reviewList)

	def removeCard(self):
		if self.hasNextCard() and self.hasPreviousCard():
			del self.reviewList[self.pointer]
			return self.reviewList[self.pointer]
		elif not self.hasNextCard() and self.hasPreviousCard():
			del self.reviewList[self.pointer]
			self.pointer -= 1
			return self.reviewList[self.pointer]
		elif self.hasNextCard() and not self.hasPreviousCard():
			del self.reviewList[self.pointer]
			return self.reviewList[self.pointer]
		elif not self.hasNextCard() and not self.hasPreviousCard():
			del self.reviewList[self.pointer]
			return None
