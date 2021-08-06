#!/usr/bin/python

# queditor 1.0.0
# author: Alfonso E.M. alfonso@el-magnifico.org
# date: 25/Feb/2008

import pygtk
pygtk.require ('2.0')
import gtk
import gtk.glade
import os,sys
import getopt
import time

class Questions:
	dirname=""
	def __init__(self,dirname):
		"""
		Questions directory with question files

		"""

		self.dirname=dirname
        	self.liststore = gtk.ListStore(int,str,str,str,str,str,str)
                self.item=[]
		self.error=''
		
		self.load()


	def load(self):

        	self.liststore.clear()

		try:
			filelist=os.listdir(self.dirname)
		except:
			self.error="Error. El directorio "+self.dirname+" no existe o no se puede leer."
			print self.error
		else:
                  self.id=0
 		  for filename in filelist:
		    if filename[:1] == "q":
                      self.id=self.id+1
		      file=open(self.dirname+"/"+filename)
		      question=file.readline().replace("\n","")
		      answer1=file.readline().replace("\n","")
		      answer2=file.readline().replace("\n","")
		      answer3=file.readline().replace("\n","")
		      ok=file.readline().replace("\n","")
		      i=self.liststore.append([self.id,filename,question,answer1,answer2,answer3,ok])
                      self.item.append({})
		      file.close


class Appgui:
	def __init__(self, questions):
		"""
		In this init the  window is displayed
		"""
		dic = {
			 "on_menu_new_activate" : (self.menu_new_activate),
			 "on_menu_remove_activate" : (self.menu_remove_activate),
			 "on_menu_quit_activate" : (self.quit),
			 "on_menu_about_activate" : (self.about),
		         "on_win_main_destroy" : (self.quit), 
		         "on_tree_questions_row_activated" : self.edit_item, 
		         "on_bt_edit_ok_clicked" : self.bt_edit_ok_clicked, 
		         "on_bt_edit_cancel_clicked" : self.bt_edit_cancel_clicked, 
		         "on_win_edit_destroy_event" : self.bt_edit_cancel_clicked, 
		}

		self.questions = questions
		self.xml = gtk.glade.XML("queditor.glade")
		self.xml.signal_autoconnect (dic)
		self.treeview = self.xml.get_widget('tree_questions')
		self.win_main = self.xml.get_widget('win_main')
		self.win_edit=self.xml.get_widget('win_edit')
		self.win_about=self.xml.get_widget('win_about')

		self.ent_question = self.xml.get_widget('ent_question')

		self.ent_answer1 = self.xml.get_widget('ent_answer1')
		self.ent_answer2 = self.xml.get_widget('ent_answer2')
		self.ent_answer3 = self.xml.get_widget('ent_answer3')
		self.radio_ok1 = self.xml.get_widget('radio_ok1')
		self.radio_ok2 = self.xml.get_widget('radio_ok2')
		self.radio_ok3 = self.xml.get_widget('radio_ok3')

		self.treeview.set_rules_hint(True)

		self.treeview.set_model(model=self.questions.liststore)

	        # Add columns to treeview
	        column = gtk.TreeViewColumn('Nº')
       		self.treeview.append_column(column)
	        cell = gtk.CellRendererText()
                column.pack_start(cell, True)
                column.set_attributes(cell, text=0)

	        column = gtk.TreeViewColumn('Pregunta')
       		self.treeview.append_column(column)
	        cell = gtk.CellRendererText()
                column.pack_start(cell, True)
                column.set_attributes(cell, text=2)

		self.filename=''


	def run(self):
		gtk.main()

	def quit(*args):
		if hasattr(gtk, 'main_quit'):
	            gtk.main_quit()
	        else:
	            gtk.mainquit()
	
	def about(self,widget):
    		self.win_about.show()
		return

	                            

	def edit_item(self,treeview,TreePath,TreeViewColumn):
                selection=treeview.get_selection()
		(model,iter)=selection.get_selected()

		self.filename=self.questions.liststore.get_value(iter,1)
		self.ent_question.set_text(self.questions.liststore.get_value(iter,2))
		self.ent_answer1.set_text(self.questions.liststore.get_value(iter,3))
		self.ent_answer2.set_text(self.questions.liststore.get_value(iter,4))
		self.ent_answer3.set_text(self.questions.liststore.get_value(iter,5))
		ok=self.questions.liststore.get_value(iter,6)

		if ok == "A":
		   self.radio_ok1.set_active(1)
		elif ok == "B":
		   self.radio_ok2.set_active(1)
   		else:
		   self.radio_ok3.set_active(1)
             
                self.win_edit.show()

		return

	def menu_remove_activate(self,widget):
                selection=self.treeview.get_selection()
 		(model,iter)=selection.get_selected()
		if iter:
   		  self.filename=self.questions.liststore.get_value(iter,1)
		  os.remove(self.questions.dirname+"/"+self.filename)
		  self.questions.liststore.remove(iter)
		return

	def menu_new_activate(self,widget):
		self.filename=''

		self.ent_question.set_text('¿?')
		self.ent_answer1.set_text('')
		self.ent_answer2.set_text('')
		self.ent_answer3.set_text('')
                self.win_edit.show()
                return


	def bt_edit_ok_clicked(self,widget):

		question=self.get_value('ent_question')
		answer1=self.get_value('ent_answer1')
		answer2=self.get_value('ent_answer2')
		answer3=self.get_value('ent_answer3')
		ok=""
		if self.radio_ok1.get_active():
		   ok = "A"
		if self.radio_ok2.get_active():
		   ok = "B"
		if self.radio_ok3.get_active():
		   ok = "C"

	        if self.filename == '':
		   self.filename="q"+str(int(time.time()))
		   self.questions.id=self.questions.id+1
                   i=self.questions.liststore.append([self.questions.id,self.filename,question,answer1,answer2,answer3,ok])

		else:
		   selection=self.treeview.get_selection()
   		   (model,iter)=selection.get_selected()
        	   self.questions.liststore.set_value(iter,2,question)
            	   self.questions.liststore.set_value(iter,3,answer1)
        	   self.questions.liststore.set_value(iter,4,answer2)
        	   self.questions.liststore.set_value(iter,5,answer3)
        	   self.questions.liststore.set_value(iter,6,ok)


		file=open(self.questions.dirname+"/"+self.filename,"w")
		file.write(question+"\n")
		file.write(answer1+"\n"+answer2+"\n"+answer3+"\n")
		file.write(ok)
		file.close

                 
   	        self.win_edit.hide()

                return


        def get_value(self,widgetname):
                widget=self.xml.get_widget(widgetname)
	        return widget.get_text()

        def bt_edit_cancel_clicked(self,widget,*args):
	        self.win_edit.hide()  
		return True


		
def usage():
	print """
Usage:
   -h	--help		This simple help
   -d	--dir=xxx	Questions dir to edit (default is ./questions/)
	"""


def main():
	try:
		opts, args = getopt.getopt(sys.argv[1:],"hd:",["help","dir="])
	except getopt.GetoptError:
		usage()
		sys.exit(2)

	dir="./questions/"
	for opt, arg in opts:     
	    if opt in ("-h", "--help"):
		usage()
		sys.exit()                  
	    elif opt in ("-d", "--dir"):
		dir = arg
	
	questions=Questions(dir)
	app=Appgui(questions)
	app.run()
	  

if __name__ == '__main__':
	main()

