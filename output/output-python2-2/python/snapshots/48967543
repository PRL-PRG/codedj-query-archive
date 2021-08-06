#!/usr/bin/python
# -*- coding: iso-8859-1 -*-

import pygtk 
pygtk.require ('2.0')
import gtk
import gtk.glade
import os
import commands
import random
import time
import sys



class appgui:
	def __init__(self):
		"""
		In this init the main window is displayed
		"""
		dic = {
		         "on_window1_destroy" : (gtk.mainquit), 
                         "on_bt_a_clicked" : self.bt_a_clicked, 
			 "on_bt_b_clicked" : self.bt_b_clicked, 
			 "on_bt_c_clicked" : self.bt_c_clicked, 
			 "on_bt_restart_clicked" : self.bt_restart_clicked, 
		}

		self.xml = gtk.glade.XML("guadaquiz.glade")
		self.xml.signal_autoconnect (dic)
		self.window = self.xml.get_widget('window1')
		self.window.set_size_request(800,600)
		self.question = self.xml.get_widget('question')
		self.answers = self.xml.get_widget('answers')
		self.counter = self.xml.get_widget('counter')
		self.rightanswer=""
		self.appdir=os.path.dirname(sys.argv[0])
		self.questionfiles=os.listdir(self.appdir+"/questions")

		self.ok=0
		self.wrong=0

		return

	def bt_a_clicked(self,widget):
		widget.set_sensitive(False)
		self.check_answer("A")
		self.show_another_question()
		widget.set_sensitive(True)
	def bt_b_clicked(self,widget):
		widget.set_sensitive(False)
		self.check_answer("B")
		self.show_another_question()
		widget.set_sensitive(True)
	def bt_c_clicked(self,widget):
		widget.set_sensitive(False)
		self.check_answer("C")
		self.show_another_question()
		widget.set_sensitive(True)

	def bt_restart_clicked(self,widget):
		self.ok=0
		self.wrong=0
		self.show_counter()

	def show_counter(self):
		self.counter.set_text("%d Bien\n%d Mal" % (self.ok,self.wrong))
		
	
	def check_answer(self,answer):
		if self.rightanswer != "":
			if answer == self.rightanswer:
				self.ok=self.ok+1
				status,output=commands.getstatusoutput("play "+self.appdir+"/sounds/ok.ogg")
			else:
				self.wrong=self.wrong+1
				status,output=commands.getstatusoutput("play "+self.appdir+"/sounds/wrong.ogg")
		self.show_counter()

	def show_another_question(self):
		while questionfile[0:1] != "q":
			questionfile=random.choice(self.questionfiles)
                try:
			qfile=open(self.appdir+"/questions/"+questionfile)
		except:
			sys.exit("ERROR: can't open question file "+questionfile)
		question=unicode(qfile.readline(),'Latin-1')
		answer1=qfile.readline()
		answer2=qfile.readline()
		answer3=qfile.readline()
		self.rightanswer=qfile.readline()
		qfile.close
		self.question.set_text(question)
		self.answers.set_text("A - "+answer1+"\n B - "+answer2+"\n C - "+answer3)

app=appgui()

gtk.main()
