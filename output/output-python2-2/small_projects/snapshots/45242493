# -*- coding: utf-8 -*-

# OpenCAL version 3.0
# Copyright (c) 2007,2008 Jérémie Decock (http://www.jdhp.org)

import gtk
import hildon
from Reviewer import Pile
from Reviewer import Card

class GUI:
	def __init__(self):
		self.pile = Pile(CARD_DATABASE)
		self.card = self.pile.getPointedCard()
		
		self.window = hildon.Window()
		self.window.set_title("OpenCAL")
		#self.window.set_default_size(573, 392)
		self.window.set_size_request(573, 392)
		self.window.set_resizable(False)

		self.fixed = gtk.Fixed()
		self.tw = gtk.TextView()
		self.sw = gtk.ScrolledWindow()
		self.nextButton = gtk.Button("->")
		self.revButton = gtk.Button("Rev")
		self.prevButton = gtk.Button("<-")
		self.goodButton = gtk.Button("+")
		self.badButton = gtk.Button("-")
		
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion()))
		self.sw.set_policy(gtk.POLICY_NEVER, gtk.POLICY_ALWAYS)
		self.tw.set_wrap_mode(gtk.WRAP_WORD)
		self.tw.set_editable(False)
		self.tw.set_cursor_visible(False)
		self.sw.add(self.tw)

		self.window.add(self.fixed)
		self.fixed.put(self.sw, 16, 16)
		self.fixed.put(self.goodButton, 548, 128)
		self.fixed.put(self.badButton, 548, 184)
		self.fixed.put(self.prevButton, 16, 352)
		self.fixed.put(self.revButton, 220, 352)
		self.fixed.put(self.nextButton, 424, 352)

		self.sw.set_size_request(516, 312)
		self.goodButton.set_size_request(64, 32)
		self.badButton.set_size_request(64, 32)
		self.prevButton.set_size_request(196, 32)
		self.revButton.set_size_request(196, 32)
		self.nextButton.set_size_request(196, 32)
		
		self.window.connect("delete_event", self.delete_event)
		self.window.connect("destroy", self.destroy_event)
		self.nextButton.connect("clicked", self.next_event)
		self.revButton.connect("clicked", self.answer_event)
		self.prevButton.connect("clicked", self.prev_event)
		self.goodButton.connect("clicked", self.good_event)
		self.badButton.connect("clicked", self.bad_event)
		self.window.connect("key-press-event", self.keyboard_press_event)
		
		self.prevButton.set_sensitive(False)
		if self.pile.pointerIsOnTheLastCard():
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
		print "Bye"
	
	def destroy_event(self, widget, data=None):
		gtk.main_quit()
	
	def next_event(self, widget, data=None):
		self.next()

	def prev_event(self, widget, data=None):
		self.prev()

	def answer_event(self, widget, data=None):
		self.answer()

	def good_event(self, widget, data=None):
		self.good()

	def bad_event(self, widget, data=None):
		self.bad()

	def keyboard_press_event(self, widget, event, *args):
		if event.keyval == gtk.keysyms.GDK_Up:
			self.good()
		elif event.keyval == gtk.keysyms.GDK_Down:
			self.bad()
		elif event.keyval == gtk.keysyms.GDK_Left:
			self.prev()
		elif event.keyval == gtk.keysyms.GDK_Right:
			self.next()
		elif event.keyval == gtk.keysyms.GDK_Return:
			self.answer()

	def next(self):
		self.pile.gotoNextCard()
		self.card = self.pile.getPointedCard()
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion()))
		if self.pile.pointerIsOnTheLastCard():
			self.nextButton.set_sensitive(False)
		self.prevButton.set_sensitive(True)

	def prev(self):
		self.pile.gotoPrevCard()
		self.card = self.pile.getPointedCard()
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion()))
		if self.pile.pointerIsOnTheFirstCard():
			self.prevButton.set_sensitive(False)
		self.nextButton.set_sensitive(True)

	def answer(self):
		self.nextButton.set_sensitive(False)
		self.revButton.set_sensitive(False)
		self.prevButton.set_sensitive(False)
		self.goodButton.set_sensitive(True)
		self.badButton.set_sensitive(True)
		#self.tw.get_buffer().set_text("\nANSWER :\n%s\n" % (self.card.getAnswer()))
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n\nANSWER :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion(), self.card.getAnswer()))

	def good(self):
		self.updateCard("GOOD")
		self.card = self.pile.getPointedCard()
		if not self.pile.pointerIsOnTheLastCard():
			self.nextButton.set_sensitive(True)
		self.revButton.set_sensitive(True)
		if not self.pile.pointerIsOnTheFirstCard():
			self.prevButton.set_sensitive(True)
		self.goodButton.set_sensitive(False)
		self.badButton.set_sensitive(False)
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion()))

	def bad(self):
		self.updateCard("BAD")
		self.card = self.pile.getPointedCard()
		if not self.pile.pointerIsOnTheLastCard():
			self.nextButton.set_sensitive(True)
		self.revButton.set_sensitive(True)
		if not self.pile.pointerIsOnTheFirstCard():
			self.prevButton.set_sensitive(True)
		self.goodButton.set_sensitive(False)
		self.badButton.set_sensitive(False)
		self.tw.get_buffer().set_text("QUESTION %d (r:%d - g:%d) :\n%s\n" % (self.pile.getReviewedCards() + 1, self.pile.getRemainingCards(), self.card.getGrade(), self.card.getQuestion()))

	def updateCard(self, result):
		# Write answer and date into xml file
		file = open(self.pile.getCardDb(), 'rU')
		parser = xml.sax.make_parser()
		parser.setFeature(xml.sax.handler.feature_namespaces, False)
		newFile = open(TMP_DATABASE, 'w')
		reviewHandler = ReviewHandler(newFile, self.card.getId(), result)
		parser.setContentHandler(reviewHandler)
		parser.parse(file)
		newFile.close()
		file.close()
		if os.access(self.pile.getCardDb(), os.X_OK):
			os.remove(self.pile.getCardDb())
		os.rename(TMP_DATABASE, self.pile.getCardDb())
		# ...
		self.pile.incrementReviewedCards()
		self.pile.removePointedCard()

	def main(self):
		gtk.main()
