#!/usr/bin/env python
# -*- coding: UTF-8 -*-

# Copyright 2006-2007 (C) Raster Software Vigo (Sergio Costas)
# Copyright 2006-2007 (C) Peter Gill - win32 parts

# This file is part of DeVeDe
#
# DeVeDe is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# DeVeDe is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.


import signal
import os
import sys
import subprocess
import select
if sys.platform=='win32':
	import win32api
	import win32process
	import time

import threading

class executor:

	""" Base class for all launchers (Mplayer, Mencoder, SPUmux, DVDauthor, mkisofs...). """

	def __init__(self,filename=None,filefolder=None,progresbar=None):

		# FILENAME is the generic file name given by the user
		# FILEFOLDER is the path where all the temporary and finall files will be created
		# PROGRESBAR is the GtkProgressBar where the class will show the progress

		if filename!=None:
			self.bar=progresbar
			if progresbar!=None:
				progresbar.set_text(" ")
			self.filefolder=filefolder
			self.filename=filename
			self.handle=None
			self.cadena=""
			self.platform_win32=(sys.platform=='win32')
			self.printout=True
			self.print_error="Undefined error"


	def cancel(self):

		""" Called to kill this process. """

		if self.handle==None:
			return
		if sys.platform=='win32':
			try:
				win32api.TerminateProcess(int(self.handle._handle), -1)
			except Exception , err:
				print "Error: ", err
		else:
			os.kill(self.handle.pid,signal.SIGKILL)
		
	
	def wait_end(self):
		
		""" Wait until the process ends """
		
		if self.handle==None:
			return 0
		
		self.handle.wait()
		return self.handle.returncode


	def launch_shell(self,program,read_chars=80,output=True,stdinout=None):
		
		""" Launches a program from a command line shell. Usefull for programs like SPUMUX, which
		takes the input stream from STDIN and gives the output stream to STDOUT, or for programs like
		COPY, CP or LN """
		
		self.read_chars=read_chars
		self.output=output
				
		if stdinout!=None: # we want to apply a file as STDIN and another one as STDOUT
			lprogram=program+' < "'+stdinout[0]+'" > "'+stdinout[1]+'"'
			if sys.platform=='win32':
				try:
					wd=sys.path[-1:] # Current working Directory.  To work with py2exe
					b=os.path.join(wd[0], "bin", "spumux.exe")
					lprogram=lprogram.replace("spumux.exe", '"' + b + '"')
					batfile=open(os.path.join(wd[0],"menu.bat"),"w")
					batfile.write(lprogram)
					batfile.close()
				except:
					return None
				lprogram=os.path.join(wd[0],"menu.bat")
		else:
			lprogram=program

		print "Launching shell program: ",program
		print

		try:
			if output:
				if sys.platform=='win32':
					handle=MyPopen(lprogram,shell=False,bufsize=32767,stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
				else:
					handle=subprocess.Popen(lprogram,shell=True,bufsize=32767,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
			else:
				if sys.platform=='win32':
					handle=subprocess.Popen(lprogram,shell=True,stdin=subprocess.PIPE, stdout=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
				else:
					handle=subprocess.Popen(lprogram,shell=True)
		except OSError:
			print "error launching shell\n\n\n\n"
			pass
		else:
			self.handle=handle
			return handle

		print "Fallo"
		self.handle=None
		return None	


	def launch_program(self,program,read_chars=80,output=True):

		""" Launches a program that can be located in any of the directories stored in PATHLIST """

		self.read_chars=read_chars
		self.output=output

		wd=sys.path[-1:] # working directory.  This works with py2exe
		pathlist=["/usr/bin","/usr/local/bin","/usr/share/bin","/usr/share/local/bin","/bin", os.path.join(wd[0],"bin"), r'C:\WINDOWS', r'C:\WINDOWS\system32', r'C:\WINNT']

		print "Launching program:"
		print "program: ",program
		print

		for elemento in pathlist:
			print "elemento: ", elemento
			if elemento[-1]!=os.sep:
				elemento+=os.sep
			try:
				program2=program[:]
				program2[0]=elemento+program2[0]
				if output:
					if sys.platform=='win32':
						handle=MyPopen(program2,executable=program2[0],shell=False,stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
					else:
						handle=subprocess.Popen(program2,executable=program[0],shell=False,bufsize=32767,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
				else:
					if sys.platform=='win32':
						handle=MyPopen(program2,executable=program2[0],shell=False,stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=win32process.CREATE_NO_WINDOW)
					else:
						handle=subprocess.Popen(program2,executable=program[0],shell=False)
			except OSError:
				print "error in launch program\n\n\n\n"
				pass
			else:
				self.handle=handle
				return handle
		self.handle=None
		return None


	def refresh(self):
		
		""" Reads STDOUT and STDERR and refreshes the progress bar. """
		
		if self.handle==None:
			return -1 # there's no program running
		
		if self.output==False: # if we don't want to read the output...
			self.bar.pulse() # just PULSE the progress bar
			if self.handle.poll()==None:
				return 0 # if the program didn't end, return 0
			else:
				return 1 # and 1 if the program ended
		
		ret_value=1
		v1=[]
		while self.handle.poll()==None:
			if self.platform_win32:
				v1 = self.handle.recv_some()
			else:
				v1,v2,v3=select.select([self.handle.stderr,self.handle.stdout],[],[],0)

			if len(v1)==0:
				ret_value=0
				break # nothing to read, so get out of the WHILE loop
			
			for element in v1:
				if sys.platform=="win32":
					readed = element#[0,self.read_chars]
					self.cadena+=readed
					if (self.printout):# or (v1==v2[0]):
						print readed,
				else:
					readed=element.readline(self.read_chars)
					self.cadena+=readed
					if (self.printout) or (v1==self.handle.stderr):
						print readed,

		if (self.set_progress_bar()): # progress_bar is defined in each subclass to fit the format
			self.cadena=""
		
		return ret_value # 0: nothing to read; 1: program ended


	def set_progress_bar(self):
		
		# By default, just do nothing
		return True

	def create_filename(self,filename,title,file,avi):

		""" Starting from the generic filename, adds the title and chapter numbers and the extension """

		currentfile=filename+"_"
		if title<10:
			currentfile+="0"
		currentfile+=str(title)+"_"

		if file<10:
			currentfile+="0"

		if avi:
			currentfile+=str(file)+'.avi'
		else:
			currentfile+=str(file)+'.mpg'
		return currentfile
	
class MyPopen(subprocess.Popen):
	"""
	Threaded subclass of subprocess.Popen to allow for non-blocking input output of subprocesses.

	Returns a list of lists.
	eg. return [ [stdout, buffer], [stderr, buffer] ]
	Usage:
	out, err = MyPopen.recv_some()
	for x in out:
		print "This is the output: " x
	"""
	
	class PipeThread(threading.Thread):
		def __init__(self, fin):
			threading.Thread.__init__(self)
			self.fin = fin
			self.sout = []
			#self.sout = ""
			
		def run(self):
			while True:
				#temp = self.fin.readline()
				temp=self.fin.read(80)
				if not temp: break
				#print "temp from popen: ", temp
				self.sout.append(temp)
				#self.sout += temp
		def getOutput(self):
			return self.sout
				
		def reset(self):
			self.sout = []
			#self.sout = ""
		
	def __init__(self, args=None, bufsize=0, executable=None, stdin=None, stdout=None, stderr=None, preexec_fn=None, close_fds=False, shell=False, cwd=None, env=None, universal_newlines=False, startupinfo=None, creationflags=0):
		subprocess.Popen.__init__(self,args=args, bufsize=bufsize, executable=executable, stdin=stdin, stdout=stdout, stderr=stderr, preexec_fn=preexec_fn, close_fds=close_fds, shell=shell, cwd=cwd, env=env, universal_newlines=universal_newlines, startupinfo=startupinfo, creationflags=creationflags)
		self.outPipe, self.errPipe = self.PipeThread(self.stdout), self.PipeThread(self.stderr)
		self.outPipe.start(), self.errPipe.start()
	
	def recv_some(self):
		"""
		Returns a copy of the lists holding stdout and stderr
		Just before returning clears the orignal lists
		"""
		time.sleep(0.1)
		out, err = self.outPipe.getOutput(), self.errPipe.getOutput()
		self.outPipe.reset()
		self.errPipe.reset()
		out.extend(err) # this will sort of get it to work with dvdauthor and mkisofs
			# for some reason only output to read is from the stderr for those programs
		return out #[out, err]
