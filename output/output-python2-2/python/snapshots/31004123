#!/usr/bin/env python
# -*- coding: gbk -*-
import os,aeslib

#定义程序运行期间的全局变量
conn=None
app_stat="Idle"
log_stat='no'
log_num=0
list=[]
app_path=os.path.abspath('')
if len(app_path)>=5:
	app_path+='/'
userid=""
userpass=""
local_dir=""
current_courseindex=0
current_fileindex=[]
current_markfile=[]
localsel=[]
local_files=[]
print_files=[]
file_askinfo=""
setting={'userinfo':[aeslib.encode(u''),aeslib.encode(u'')],'autologin':False,'download_path':u'D:\\','print_path':u'C:\\','filter':[]}
#注意新公告中直接显示了公告内容,格式：{(courseindex,noteindex):notecontent,...}
newnote={}
newfile=[]

main_frame=None
logDialog=None
txtLocalPath=None
txtRemoteCourse=None
lstRemoteFile=None
lstRemoteCourse=None
dirLocal=None
statusBar=None
askDialog=None
setDialog=None
selDirDialog=None
aboutDialog=None
warnDialog=None
html=None
theThread=None
t2=None