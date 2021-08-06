# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

import pygtk
pygtk.require('2.0')
import gtk

from Reviewer import CardToReview

class GUI:
	def __init__(self, reviewList):
		self.reviewList = reviewList
		self.cardToReview = self.reviewList.firstCard()

		self.window = gtk.Window(gtk.WINDOW_TOPLEVEL)
		self.window.set_title("OpenCAL")
		#self.window.set_default_size(573, 392)
		self.window.set_size_request(573, 392)
		self.window.set_resizable(False)

		self.fixed = gtk.Fixed()
		self.tw = gtk.TextView()
		self.sw = gtk.ScrolledWindow()
		self.nextButton = gtk.Button("Next", gtk.STOCK_GO_FORWARD)
		self.revButton = gtk.Button("Review")
		self.prevButton = gtk.Button("Preview", gtk.STOCK_GO_BACK)
		self.goodButton = gtk.Button("Good", gtk.STOCK_ADD)
		self.badButton = gtk.Button("Bad", gtk.STOCK_REMOVE)
		
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion()))
		self.sw.set_policy(gtk.POLICY_NEVER, gtk.POLICY_ALWAYS)
		self.tw.set_wrap_mode(gtk.WRAP_WORD)
		self.tw.set_editable(False)
		self.tw.set_cursor_visible(False)
		self.sw.add(self.tw)

		self.window.add(self.fixed)
		self.fixed.put(self.sw, 16, 16)
		self.fixed.put(self.goodButton, 488, 128)
		self.fixed.put(self.badButton, 488, 184)
		self.fixed.put(self.prevButton, 16, 352)
		self.fixed.put(self.revButton, 200, 352)
		self.fixed.put(self.nextButton, 384, 352)

		self.sw.set_size_request(456, 312)
		self.goodButton.set_size_request(64, 32)
		self.badButton.set_size_request(64, 32)
		self.prevButton.set_size_request(176, 32)
		self.revButton.set_size_request(176, 32)
		self.nextButton.set_size_request(176, 32)
		
		self.window.connect("delete_event", self.delete_event)
		self.window.connect("destroy", self.destroy)
		self.nextButton.connect("clicked", self.next)
		self.revButton.connect("clicked", self.answer)
		self.prevButton.connect("clicked", self.prev)
		self.goodButton.connect("clicked", self.good)
		self.badButton.connect("clicked", self.bad)
		
		self.prevButton.set_sensitive(False)
		if not self.reviewList.hasNextCard():
			self.nextButton.set_sensitive(False)

		self.tw.show()
		self.sw.show()
		self.nextButton.show()
		self.revButton.show()
		self.prevButton.show()
		self.goodButton.show()
		self.badButton.show()
		self.fixed.show()
		self.window.show()
		self.goodButton.set_sensitive(False)
		self.badButton.set_sensitive(False)
	
	def delete_event(self, widget, event, data=None):
		pass
	
	def destroy(self, widget, data=None):
		gtk.main_quit()
	
	def next(self, widget, data=None):
		self.cardToReview = self.reviewList.nextCard()
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion()))
		if not self.reviewList.hasNextCard():
			self.nextButton.set_sensitive(False)
		self.prevButton.set_sensitive(True)

	def prev(self, widget, data=None):
		self.cardToReview = self.reviewList.previousCard()
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion()))
		if not self.reviewList.hasPreviousCard():
			self.prevButton.set_sensitive(False)
		self.nextButton.set_sensitive(True)

	def answer(self, widget, data=None):
		self.nextButton.set_sensitive(False)
		self.revButton.set_sensitive(False)
		self.prevButton.set_sensitive(False)
		self.goodButton.set_sensitive(True)
		self.badButton.set_sensitive(True)
		#self.tw.get_buffer().set_text("\nANSWER :\n%s\n" % (self.cardToReview.getAnswer()))
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n\nANSWER :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion(), self.cardToReview.getAnswer()))

	def good(self, widget, data=None):
		self.cardToReview.putReview("good")
		self.cardToReview = self.reviewList.removeCard()
		self.goodButton.set_sensitive(False)
		self.badButton.set_sensitive(False)
		if self.cardToReview == None:
			self.prevButton.set_sensitive(False)
			self.nextButton.set_sensitive(False)
			self.revButton.set_sensitive(False)
			self.tw.get_buffer().set_text("REVIEW DONE")
		else:
			if self.reviewList.hasNextCard():
				self.nextButton.set_sensitive(True)
			self.revButton.set_sensitive(True)
			if self.reviewList.hasPreviousCard():
				self.prevButton.set_sensitive(True)
			self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion()))

	def bad(self, widget, data=None):
		self.cardToReview.putReview("bad")
		self.cardToReview = self.reviewList.removeCard()
		self.goodButton.set_sensitive(False)
		self.badButton.set_sensitive(False)
		if self.cardToReview == None:
			self.prevButton.set_sensitive(False)
			self.nextButton.set_sensitive(False)
			self.revButton.set_sensitive(False)
			self.tw.get_buffer().set_text("REVIEW DONE")
		else:
			if self.reviewList.hasNextCard():
				self.nextButton.set_sensitive(True)
			self.revButton.set_sensitive(True)
			if self.reviewList.hasPreviousCard():
				self.prevButton.set_sensitive(True)
			self.tw.get_buffer().set_text("QUESTION %d (r:%d - p:%d) :\n%s\n" % (self.reviewList.getReviewedCards() + 1, self.reviewList.getRemainingCards(), self.cardToReview.getGrade(), self.cardToReview.getQuestion()))

	def main(self):
		gtk.main()
