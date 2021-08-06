#!C://pytho25//python.exe
# -*- coding: gbk -*-
import re,os,httplib,threading,urllib
import global_var,GUItools
from HTMLParser import HTMLParser
from copy import deepcopy

#a class to setup http link to the server
class MyCon:  
    def __init__(self,host='learn.tsinghua.edu.cn'):
        self.conn=httplib.HTTPConnection(host,80)
        self.precookie=''
        self.thu=' '
        self.logstat=0
        
    def open(self,uri,body=None,method="GET"):
        self.conn.close()
        if(self.logstat==1):
            headers={'Cookie':self.precookie+self.thu}
        else:
            headers={}
        if method=="POST":
            headers['Content-Type']="application/x-www-form-urlencoded"
        else:
            pass
        self.conn.request(method,uri,body,headers)
        r=self.conn.getresponse()
        rescookie=r.getheader('set-cookie')
        if(self.logstat==0):
            try:
                JSESSIONID = re.findall(r'JSESSIONID=.*?;',rescookie)[0][11:-1]
                thuwebcookie=re.findall(r'thuwebcookie=.*?;',rescookie)[0][13:-1]
                self.precookie+=('JSESSIONID=' + JSESSIONID + '; thuwebcookie=' + thuwebcookie + '; ')
            except:
                print('对不起，无法登陆，程序退出\n')
                raise 'err'
        try:
            THNSV2COOKIE=re.findall(r'THNSV2COOKIE=.*?;',rescookie)[0][13:-1]
            self.thu = ' THNSV2COOKIE=' + THNSV2COOKIE + ' '
        except:
            print('无法获得主机确认，退出\n')
            raise 'err'
        return r

    def login(self):
        params = urllib.urlencode({'userid': global_var.userid, 'userpass': global_var.userpass, 'submit1': '登录'})
        self.open('/use_go.jsp',body=params,method="POST")
        self.logstat=1

    def logout(self):
        self.logstat=0
        self.precookie=''
        self.thu=''

#解析出课程url和课程名称注意：list格式:[['courseURL','coursename',[][]],...etc]
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
            #去除括号内的信息
            coursename=coursename.split('(')[0]
            #print(coursename+'\n')
            self.course.append(coursename)
            self.course.append([])
            self.course.append([])
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
            


#a class to parser notelist
class parserNote(HTMLParser):
    def __init__(self):
        HTMLParser.__init__(self)
        self.state='none'
        self.notes=[]
        self.note={}

    def handle_starttag(self,tag,attrs):
        if(self.state=='none'):
            for i in attrs:
                if i[0]=='href' and ('note_reply.jsp?bbs_type=' in i[1]):
                    url=i[1]
                    self.note['note_url']='/public/bbs/'+url
                    self.state='title'
                    return
        if(self.state=='title_c'):
            self.state='author'
            return
        if(self.state=='author_c'):
            self.state='date'
            return

    def handle_endtag(self,tag): 
        if(self.state=='title'):
            self.state='title_c'
            return
        if(self.state=='author'):
            self.state='author_c'
            return
        if(self.state=='date'):
            self.notes.append(self.note)
            self.note={}
            self.state='none'
            return

    def handle_data(self,data):
        if(self.state=='title'):
            self.note['note_title']=data
            #print(data+'\n')
        if(self.state=='author'):
            self.note['note_author']=data
        if(self.state=='date'):
            self.note['note_date']=data
            

#getCourse函数对global_var.list初始化，加入基本的课程信息
#警告：此函数应该仅在用户登录或应用程序初始化时调用
#一经调用，global_var.list中将失去所有文件，公告的信息!
def getCourse():
    global_var.app_stat='getcourse'
    #这里先读取旧列表中的信息
    conn=global_var.conn
    conn.login()
    ff=conn.open('/lesson/student/MyCourse.jsp')
    coursepage=ff.read()
    ff.close()
    pc=parserCourse()
    pc.feed(coursepage)
    list=pc.list
    global_var.list=list
    global_var.app_stat='refresh'
    

#此函数在已有课程信息基础上刷新global_var.list中的课件信息
def refreshFiles():
    global_var.app_stat='refreshfile'
    global_var.newfile=[]
    oldfile=[]
    global_var.prelist=deepcopy(global_var.list)
    list=global_var.list
    prelist=global_var.prelist
    conn=global_var.conn
    conn.login()
    pf=parserFile()
    #存储旧文件
    for course in prelist:
            for file in course[2]:
                oldfile.append(file['file_url'])
    #打开课程下载界面，解析文件地址
    for course in list:
        ff=conn.open('/lesson/student/download.jsp?course_id='+course[0][-5:])
        filepage=ff.read()
        ff.close()
        pf.__init__()
        pf.feed(filepage)
        course[2]=pf.files
    
    #把新的文件加入新文件列表中
    for courseindex in range(len(list)):
        course=list[courseindex]
        for fileindex in range(len(course[2])):
            file=course[2][fileindex]
            if not (file['file_url'] in oldfile):
                global_var.newfile.append((courseindex,fileindex))
    
    #确定每个文件的具体信息(文件名，实际大小)
    for course in list:
        for file in course[2]:
            data=conn.open(file['file_url'],method="HEAD")
            uu=data.getheader('content-disposition')
            file['file_realsize']=int(data.getheader('content-length'))
            data.read()
            data.close()
            raw_name=re.findall(r'=".*"',uu)[0][2:-1]
            #找寻随机数
            file_random=re.findall(r'\S+_(\d{7,9}).\w+$',raw_name)[0]
            if file_random:
                file['file_realname']=raw_name.replace('_'+file_random,'')
            else:
                print '无法解析除随机号，使用原文件名，请报告这个错误'
                file['file_realname']=raw_name
    ShowNew()

#此函数在已有课程信息的基础上刷新公告信息
def refreshNotes():
    global_var.app_stat="refreshnote"
    global_var.prelist=deepcopy(global_var.list)
    prelist=global_var.prelist
    list=global_var.list
    conn=global_var.conn
    conn.login()
    oldnote=[]
    global_var.newnote={}
    pn=parserNote()
    
    for course in prelist:
        for note in course[3]:
            oldnote.append(note['note_url'])


    for courseindex in range(len(list)):
        #以下为公告信息
        course=list[courseindex]
        data=conn.open('/public/bbs/getnoteid_student.jsp?course_id='+course[0][-5:],method="HEAD")
        uu=data.getheader('Location').replace('http://learn.tsinghua.edu.cn','')
        data.read()
        data.close()
        data=conn.open(uu)
        pn.__init__()
        pn.feed(data.read())
        course[3]=pn.notes
        data.close()
        #公告地址抓取结束
        CreateHtml(courseindex,oldnote)
    ShowNew()

#此函数刷新指定课程的文件列表和公告
def RefreshCourse(courseindex):
    global_var.app_stat="refreshcourse"
    course=global_var.list[courseindex][:2]
    conn=global_var.conn
    ff=conn.open('/lesson/student/download.jsp?course_id='+course[0][-5:])
    filepage=ff.read()
    ff.close()
    pf=parserFile()
    pn=parserNote()
    pf.__init__()
    pf.feed(filepage)
    files=pf.files
    course.append(files)
    data=conn.open('/public/bbs/getnoteid_student.jsp?course_id='+course[0][-5:],method="HEAD")
    uu=data.getheader('Location').replace('http://learn.tsinghua.edu.cn','')
    data.read()
    data.close()
    data=conn.open(uu)
    pn.__init__()
    pn.feed(data.read())
    course.append(pn.notes)
    data.close()
    for file in course[2]:
        data=conn.open(file['file_url'],method="HEAD")
        uu=data.getheader('content-disposition')
        file['file_realsize']=int(data.getheader('content-length'))
        data.read()
        data.close()
        raw_name=re.findall(r'=".*"',uu)[0][2:-1]
        #找寻随机数
        file_random=re.findall(r'\S+_(\d{7,9}).\w+$',raw_name)[0]
        if file_random:
            file['file_realname']=raw_name.replace('_'+file_random,'')
        else:
            print '无法解析除随机号，使用原文件名，请报告这个错误'
            file['file_realname']=raw_name   
    global_var.list[courseindex]=course
    CreateHtml(courseindex)
    

#此函数生成指定课程的公告网页，被调用
def CreateHtml(courseindex,oldnote=[]):
    list=global_var.list
    conn=global_var.conn
    f=open(os.path.join(global_var.app_path,'notes',list[courseindex][1]+'.htm'),'w')
    pre='''
    <html>
    <head>
    <META http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>
    '''
    pre+=list[courseindex][1]
    pre+='''
    </title>
    <link rel="stylesheet" href="style.css" type="text/css" media="screen">
    </head>
    <body>
    <div id="header"><a name="-1"><h1>
    '''
    pre+=list[courseindex][1]+'的课程公告'
    pre+='''
    </h1></a></div>
    <div id="list">
    <ul>
    '''
    for noteindex in range(len(list[courseindex][3])):
        note=list[courseindex][3][noteindex]
        pre+='''<li><a href="#'''+str(noteindex)+'''">'''+note['note_title']+'  ('+note['note_date']+')</a></li>\n'
    
    pre+='''</ul></div>\n<div id="content">\n'''
    for noteindex in range(len(list[courseindex][3])):
        note=list[courseindex][3][noteindex]
        data=conn.open(note['note_url'])
        uu=data.read()
        data.close()
        uu=uu.split('''colspan="3" >''')[1]
        uu=uu.split('</td>')[0]
        if not(note['note_url'] in oldnote):
            global_var.newnote[(courseindex,noteindex)]=uu
        pre+='''<div class="textbox"><a name="'''+str(noteindex)+'''"></a><div class="title"><h3>'''
        pre+=note['note_title']
        pre+='''</h3><div class="label">'''+note['note_date']+'   '+note['note_author']+'''</div></div><div class="content">'''+uu
        pre+='''
        </div>
        <div class="go-top"><a href="#-1">Top</a>
        </div>
        </div><br>'''
    pre+="</div></body></html>"
    pre=pre.decode('gbk').encode('utf')
    f.write(pre)
    f.close()
    

#此函数生成显示更新信息的页面

def ShowNew():
    list=global_var.list
    pre='''
    <html>
    <head>
    <META http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>更新</title>
    <link rel="stylesheet" href="style.css" type="text/css" media="screen">
    </head>
    <body>
    <div id="header"><h1>更新的文件</h1></div>
    <div id="content">
    <ul>
    '''
    for ft in global_var.newfile:
        coursename=list[ft[0]][1]
        filename=list[ft[0]][2][ft[1]]['file_realname']
        pre+='<li>'+filename+'......'+coursename+'</li>\n'
    pre+='''
    </ul>
    </div><div id="header"><h1>更新的公告</h1></div>
    <div id="content">
    '''
    newnote=global_var.newnote
    for nt in newnote.keys():
        note=list[nt[0]][3][nt[1]]
        coursename=list[nt[0]][1]
        notecontent=newnote[nt]
        pre+='''
        <div class="textbox"><a name="1"></a>
        <div class="title"><h3>
        '''
        pre+=note['note_title']
        pre+='''
        </h3><div class="label">'''
        pre+=note['note_date']+'--'+coursename
        pre+='''
        </div>
        </div>
        <div class="content">'''
        pre+=notecontent
        pre+='''
        </div>
        </div>
        </div>'''
    pre+='''
    </div>
    </body>
    </html>
    '''
    f=open(os.path.join(global_var.app_path,'notes','newinfo.htm'),'w')
    f.write(pre.decode('gbk').encode('utf'))
    f.close()
        
        

#此函数下载指定课程的文件
def DownCourse(courseindex):
    global_var.app_stat='downcourse'
    conn=global_var.conn
    list=global_var.list
    download_path=global_var.setting['download_path']
    os.chdir(download_path)
    coursedir=os.path.join(download_path,list[courseindex][1].decode('gbk'))
    if not os.path.exists(coursedir):
        os.mkdir(coursedir)
    for fileindex in range(len(global_var.list[courseindex][2])):
        file=global_var.list[courseindex][2][fileindex]
        if FileType(courseindex,fileindex)!=0:
            continue
        else:
            if global_var.app_stat=='breakdown':
                return
            filepath=os.path.join(download_path,list[courseindex][1].decode('gbk'),file['file_realname'].decode('gbk'))
            newfile=open(filepath,'wb')
            global_var.statusBar.SetStatusText('正在下载'+file['file_realname'])
            newfile.write(conn.open(file['file_url']).read())
            newfile.close()
            #此句刷新文件列表显示
            GUItools.ShowFile(courseindex)
            global_var.lstLocalFile.InsertStringItem(len(global_var.local_files),global_var.list[courseindex][2][fileindex]['file_realname'])
            global_var.lstLocalFile.SetItemImage(len(global_var.local_files),0)
            #注意print_file列表中只有索引没有其他信息
            #local_files中是课程索引和文件索引的元组
            global_var.print_files.append(len(global_var.local_files))
            global_var.local_files.append((courseindex,fileindex))
    global_var.statusBar.SetStatusText('下载完成')
    

#此函数下载所有课程的文件
def DownAll():
    list=global_var.list
    for courseindex in range(len(list)):
        if global_var.app_stat=='breakdown':
            global_var.statusBar.SetStatusText('下载已经被中断')
            return
        global_var.current_courseindex=courseindex
        global_var.current_fileindex=[]
        DownCourse(courseindex)
        

#此函数下载指定的文件
def DownSingle(courseindex,fileindex):
    conn=global_var.conn
    list=global_var.list
    exsit=0
    download_path=global_var.setting['download_path']
    if courseindex < len(list):
        if fileindex < len(list[courseindex][2]):
            #os.chdir(download_path)
            coursedir=os.path.join(download_path,list[courseindex][1].decode('gbk'))
            if (not os.path.exists(coursedir)):
                os.mkdir(coursedir)
            #此处的字符编码统一成unicode，防止出错
            os.chdir(download_path+u'\\'+list[courseindex][1].decode('gbk'))
            filepath=os.path.join(coursedir,list[courseindex][2][fileindex]['file_realname'].decode('gbk'))
            if os.path.exists(filepath):
                exsit=1
                info="正在覆盖文件"+list[courseindex][2][fileindex]['file_realname']
            else:
                info="正在下载文件"+list[courseindex][2][fileindex]['file_realname']
            global_var.statusBar.SetStatusText(info)
            newfile=open(filepath,'wb')
            newfile.write(conn.open(list[courseindex][2][fileindex]['file_url']).read())
            newfile.close()
            if exsit:
                info="覆盖文件"+list[courseindex][2][fileindex]['file_realname']+"成功"
            else:
                info="下载文件"+list[courseindex][2][fileindex]['file_realname']+"成功"
            global_var.statusBar.SetStatusText(info)
    #os.chdir(download_path)
    return
    

#此函数判断列表中的文件是否存在于默认文件夹中
def IsExist(courseindex,fileindex):
    list=global_var.list
    download_path=global_var.setting['download_path']
    path=download_path+u'\\'+list[courseindex][1].decode('gbk')+u'\\'+list[courseindex][2][fileindex]['file_realname'].decode('gbk')
    if os.path.exists(path) and os.path.isfile(path):
        return path
    else:
        return False
    

#此函数判断文件大小是否匹配
def IsNew(courseindex,fileindex):
    list=global_var.list
    download_path=global_var.setting['download_path']
    path=download_path+u'\\'+list[courseindex][1].decode('gbk')+u'\\'+list[courseindex][2][fileindex]['file_realname'].decode('gbk')
    if os.path.exists(path) and os.path.isfile(path) and abs(os.path.getsize(path)-list[courseindex][2][fileindex]['file_realsize'])>2 :
        return path
    else:
        return False
    

#确定文件的显示类别：待下载，不下载...etc
def FileType(courseindex,fileindex):
    if(IsExist(courseindex,fileindex) and IsNew(courseindex,fileindex)):
        return 2
    if ((courseindex,fileindex) in global_var.setting['filter']):
        return 3
    else:
        if IsExist(courseindex,fileindex):
            return 1
        else:
            return 0
        