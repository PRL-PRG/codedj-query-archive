#!C://pytho25//python.exe
# -*- coding: gbk -*-
import re,os,httplib,time,sys,global_var
from HTMLParser import HTMLParser
from glob import glob
from getpass import getpass
#a class to setup http link to the server
class MyCon:
  def __init__(self,host='learn.tsinghua.edu.cn'):
       self.conn=httplib.HTTPConnection(host,80)
       self.precookie=''
       self.thu=' '
       self.logstat=0
      
  def open(self,uri,method="GET"):
       self.conn.close()
       if(self.logstat==1):
             headers={'Cookie':self.precookie+self.thu}
       else:
             headers={}
       self.conn.request(method,uri,None,headers)
       r=self.conn.getresponse()
       rescookie=r.getheader('set-cookie')
       if(self.logstat==0):
       	  try:
             JSESSIONID = re.findall(r'JSESSIONID=.*?;',rescookie)[0][11:-1]
             thuwebcookie=re.findall(r'thuwebcookie=.*?;',rescookie)[0][13:-1]
             self.precookie+=('JSESSIONID=' + JSESSIONID + '; thuwebcookie=' + thuwebcookie + '; ')
          except:
          	  print( '对不起，无法登陆，程序退出\n')
          	  raise 'err'
       try:
          THNSV2COOKIE=re.findall(r'THNSV2COOKIE=.*?;',rescookie)[0][13:-1]
          self.thu = ' THNSV2COOKIE=' + THNSV2COOKIE + ' '
       except:
       	   print('无法获得主机确认，退出\n')
       	   raise 'err'
       return r
       
  def login(self):
    	self.open('/use_go.jsp?userid='+global_var.userid+'&userpass='+global_var.userpass).read()
    	self.logstat=1
    
  def logout(self):
    	self.logstat=0
    	self.precookie=''
    	self.thu=''

#a class to parser courses and related URI
class parserCourse(HTMLParser):
   def __init__(self):
      HTMLParser.__init__(self)
      self.state='none'
      self.list=[]
      self.course=[]
   def handle_starttag(self,tag,attrs):
   	   if tag=='a' and attrs[0][0]=='href' and '/lesson/student/course_locate.jsp?course_id=' in attrs[0][1]:
   	       self.course.append(attrs[0][1])
   	       self.state='ok'
   def handle_data(self,data):
   	   if(self.state=='ok'):
   	   	   coursename=re.findall(r'\s\S.*$',data)[0][1:]
   	   	   #print(coursename+'\n')
   	   	   self.course.append(coursename)
   	   	   self.list.append(self.course)
   	   	   self.course=[]
   	   	   self.state='none'

#a class to parser course files' info and related URI
class parserFile(HTMLParser):
   def __init__(self):
      HTMLParser.__init__(self)
      self.state='none'
      self.files=[]
      self.file={}
      
   def handle_starttag(self,tag,attrs):
       if(self.state=='none'):
          for i in attrs:
           if i[0]=='href' and ('/uploadFile/downloadFile.jsp' in i[1]):
             url=i[1]
             self.file['file_url']=url
             self.state='name'
             return
       if(self.state=='name_c'):
          self.state='desc'
          return
       if(self.state=='desc_c'):
          self.state='size'
          return
       if(self.state=='size_c'):
          self.state='date'
          return
       if(self.state=='date_c'):
          self.state='none'
          return
          
   def handle_endtag(self,tag): 
       if(self.state=='name'):
          self.state='name_c'
          return
       if(self.state=='desc'):
          self.state='desc_c'
          return
       if(self.state=='size'):
          self.state='size_c'
          return

       if(self.state=='date'):
          self.files.append(self.file)
          self.file={}
          self.state='none'
          return
   def handle_data(self,data):
       if(self.state=='name'):
          self.file['file_name']=data
          #print(data+'\n')
       if(self.state=='desc'):
          self.file['file_desc']=data
       if(self.state=='size'):
          self.file['file_size']=data
       if(self.state=='date'):
          self.file['file_date']=data

#a function that can get a list ,which contains the full info of files on server
def getlist():
  conn=global_var.conn
  conn.login()
  ff=conn.open('/lesson/student/MyCourse.jsp')
  coursepage=ff.read()
  ff.close()
  pc=parserCourse()
  pc.feed(coursepage)
  list=pc.list
  pf=parserFile()
  for course in list:
    ff=conn.open('/lesson/student/download.jsp?course_id='+course[0][-5:])
    filepage=ff.read()
    ff.close()
    pf.__init__()
    pf.feed(filepage)
    files=pf.files
    course.append(files)
  for course in list:
  	  for file in course[2]:
  	  	  data=conn.open(file['file_url'],"HEAD")
  	  	  uu=data.getheader('content-disposition')
  	  	  data.read()
  	  	  data.close()
  	  	  file['file_realname']=re.findall(r'=".*"',uu)[0][2:-1]
  	  	  #print file['file_realname']
  return list

def DownCourse(courseindex):
	conn=global_var.conn
	list=global_var.list
	download_path=global_var.download_path
	os.chdir(download_path)
	if not glob(list[courseindex][1]):
		os.mkdir(list[courseindex][1])
	os.chdir(download_path+'//'+list[courseindex][1])
	for file in global_var.list[courseindex][2]:
		if (not glob(file['file_realname'])):
			newfile=open(file['file_realname'],'wb')
			global_var.statusBar.SetStatusText('正在下载'+file['file_realname'])
			newfile.write(conn.open(file['file_url']).read())
			newfile.close()
	os.chdir(download_path)
	global_var.statusBar.SetStatusText('下载完成')
	return 

def DownAll():
	list=global_var.list
	for courseindex in range(len(list)):
		DownCourse(courseindex)

def DownSingle(courseindex,fileindex):
	conn=global_var.conn
	list=global_var.list
	exsit=0
	download_path=global_var.download_path
	if courseindex < len(list):
		if fileindex < len(list[courseindex][2]):
			os.chdir(download_path)
			if (not glob(list[courseindex][1])):
				os.mkdir(list[courseindex][1])
			os.chdir(download_path+'//'+list[courseindex][1])
			if glob(list[courseindex][2][fileindex]['file_realname']):
				exsit=1
			if exsit:
				info="正在覆盖文件"+list[courseindex][2][fileindex]['file_realname']
			else:
				info="正在下载文件"+list[courseindex][2][fileindex]['file_realname']
			global_var.statusBar.SetStatusText(info)

			newfile=open(list[courseindex][2][fileindex]['file_realname'],'wb')
			newfile.write(conn.open(list[courseindex][2][fileindex]['file_url']).read())
			newfile.close()
			if exsit:
				info="覆盖文件"+list[courseindex][2][fileindex]['file_realname']+"成功"
			else:
				info="下载文件"+list[courseindex][2][fileindex]['file_realname']+"成功"
			global_var.statusBar.SetStatusText(info)
	os.chdir(download_path)
	return
	
def IsExist(courseindex,fileindex):
	list=global_var.list
	download_path=global_var.download_path
	if courseindex < len(list):
		if fileindex < len(list[courseindex][2]):
			os.chdir(download_path)
			if (not glob(list[courseindex][1])):
				return False
			else:
				os.chdir(download_path+'//'+list[courseindex][1])
				if glob(list[courseindex][2][fileindex]['file_realname']):
					return download_path+list[courseindex][1]
				else:
					return False
	os.chdir(download_path)
	return