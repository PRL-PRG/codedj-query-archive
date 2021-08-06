#!/usr/bin/env python
# -*- coding: gbk -*-

#为GUI提供必要的类和函数
import os, sys, platform,Dialogs
import global_var,download
from glob import glob
import wx
import cPickle as pickle

def Refresh():

	try:
		global_var.app_stat="Refresh"
		global_var.statusBar.SetStatusText(u"正在获取网络学堂文件列表")
		print u"正在获取网络学堂文件列表"
		global_var.list=download.getlist()
		print "获得列表"
		global_var.app_stat="Idle"
		ShowCourse()
		ShowFile(global_var.current_courseindex)
		print 'OK!'
	except:
		global_var.statusBar.SetStatusText(u":（ 列表刷新失败，登录认证失败")
		return False
	global_var.statusBar.SetStatusText(u"列表已经刷新")
	return True

def ShowFile(courseindex=0):
	lstControl = global_var.lstRemoteFile
	lstControl.DeleteAllItems()
	global_var.txtRemoteCourse.SetValue(global_var.list[courseindex][1])
	for itemindex in range(len(global_var.list[courseindex][2])):  
		item=global_var.list[courseindex][2][itemindex]
		index = lstControl.InsertStringItem(itemindex,item['file_realname'] )
		lstControl.SetStringItem(index, 1, item['file_size'])
		lstControl.SetStringItem(index, 2, item['file_date'])

def ShowCourse():
	lstControl = global_var.lstRemoteCourse
	lstControl.DeleteAllItems()
	for itemindex in range(len(global_var.list)):  
		item=global_var.list[itemindex]
		lstControl.InsertStringItem(itemindex,item[1] )

def check():
	if global_var.setting['autologin']:
		global_var.userid=global_var.setting['userinfo'][0]
		global_var.userpass=global_var.setting['userinfo'][1]
		print u'正在为您自动登录，请稍侯...'

		try:
			Login()
		except:
			print u'登陆失败'
			return
		global_var.log_stat='yes'
	return

def Login():
	global_var.statusBar.SetStatusText(u"正在登录")
	if Refresh():
		global_var.statusBar.SetStatusText(u"空闲")
	else:
		global_var.statusBar.SetStatusText(u"登录失败")
	return

def logItem_cmd(event):
	ret = global_var.logDialog.ShowModal()
	if ret==wx.ID_OK:
		global_var.userid=global_var.logDialog.info[0]
		global_var.userpass=global_var.logDialog.info[1]
		Login()

def setItem_cmd(event):
	ret = global_var.setDialog.ShowModal()
	if ret==wx.ID_OK:
		global_var.statusBar.SetStatusText(u"更新了设置")

def exitItem_cmd(event):
	global_var.main_frame.Close()
	
def courseSelected_cmd(event):
	index=event.m_itemIndex
	global_var.current_courseindex=index
	global_var.current_fileindex=[]
	ShowFile(index)

def fileSelected_cmd(event):
	global_var.current_fileindex.append(event.GetIndex())
	#print '加入'+str(global_var.current_courseindex)+','+str(event.GetIndex())
	#print '目前课程'+str(global_var.current_courseindex)+'文件栈中的文件列表：'+str(global_var.current_fileindex)

def fileDeSelected_cmd(event):
	global_var.current_fileindex.remove(event.GetIndex())
	#print '去除'+str(global_var.current_courseindex)+','+str(event.GetIndex())
	#print '目前课程'+str(global_var.current_courseindex)+'文件栈中的文件列表：'+str(global_var.current_fileindex)


def dirLocal_changed(event):
	localdir=global_var.dirLocal.GetPath()
	global_var.txtLocalPath.SetValue(localdir)
	global_var.local_dir=localdir

def downCourseSelected(event):
	download.DownCourse(global_var.current_courseindex)

def downAllSelected(event):
	download.DownAll()

def RefreshSelected(event):
	Refresh()

def downSingleSelected(event):
	filelist=global_var.current_fileindex
	filelist.sort()
	for fileindex in filelist:
		isexist=download.IsExist(global_var.current_courseindex,fileindex)
		if isexist:
			global_var.file_askinfo=u"文件"+global_var.list[global_var.current_courseindex][2][fileindex]['file_realname'].decode('gbk')+u"已经存在于"+isexist+u"，要覆盖吗？"
			global_var.askDialog.txtInfo.SetValue(global_var.file_askinfo)
			ret = global_var.askDialog.ShowModal()
			if ret==wx.ID_OK:
				download.DownSingle(global_var.current_courseindex,fileindex)
			else:
				global_var.statusBar.SetStatusText(u"下载已取消")
		else:
			download.DownSingle(global_var.current_courseindex,fileindex)

def lstRemoteFile_RightClick(event):
    """lstRemoteFile的右击事件"""
    lstControl = global_var.lstRemoteFile
    
    #生成弹出菜单
    if global_var.current_fileindex:
    	popmenu = wx.Menu()
    	menu_id_down = wx.NewId()
    	popmenu.Append(menu_id_down, u"下载文件到默认目录")
    	
    	#是否需要加选项"下载到左侧选中的文件夹中"?
    	#menu_id_down_toleft=wx.NewId()
    	#popmenu.Append(menu_id_down_toleft, u"下载文件到左侧选中目录")
    	
    	global_var.main_frame.Bind(wx.EVT_MENU,downSingleSelected, id=menu_id_down)
    	lstControl.PopupMenu(popmenu)
    	popmenu.Destroy()
    return

def saveSetting():
	f=open(global_var.app_path+'//setting','wb')
	pickle.dump(global_var.setting,f,True)
	f.close()

def loadSetting():
	f=open(global_var.app_path+'//setting','rb')
	global_var.setting=pickle.load(f)
	f.close()

#important:整个框架的初始化
def Frame__init(frame):
	
	#初始化全局变量,便于模块间互相使用
	global_var.main_frame=frame
	global_var.txtRemoteCourse=frame.txtRemoteCourse
	global_var.txtRemoteCourse.SetEditable(False)
	global_var.lstRemoteFile=frame.lstRemoteFile
	global_var.lstRemoteCourse=frame.lstRemoteCourse
	global_var.dirLocal=frame.dirLocal
	global_var.txtLocalPath=frame.txtLocalPath
	
	#建立对话框对象
	global_var.selDirDialog=wx.DirDialog(None, u"选择默认目录",style=wx.DD_DEFAULT_STYLE | wx.DD_NEW_DIR_BUTTON)
	global_var.askDialog=Dialogs.AskDialog(frame)
	global_var.setDialog=Dialogs.SetDialog(frame)
	global_var.logDialog=Dialogs.LoginDialog(frame)
	
	#建立连接对象
	global_var.conn=download.MyCon()
	global_var.statusBar=frame.statusBar
	
	#保证本地的配置文件存在
	os.chdir(global_var.app_path)
	if not glob('setting'):
		f=open('setting','wb')
		setting={'userinfo':['',''],'autologin':False,'modifyname':False,'download_path':'D://'}
		pickle.dump(setting,f,True)
		print setting
		f.close()
	
	#把配置文件读入全局变量
	loadSetting()
	
	#各控件的初始化
	global_var.setDialog.txtSetPath.SetValue(global_var.setting['download_path'])
	global_var.logDialog.txtUserid.SetValue(global_var.setting['userinfo'][0])
	global_var.logDialog.txtUserpass.SetValue(global_var.setting['userinfo'][1])
	global_var.logDialog.autoSaved.SetValue(global_var.setting['autologin'])
	font1 = wx.Font(9, wx.FONTFAMILY_DEFAULT, wx.FONTSTYLE_NORMAL,wx.FONTWEIGHT_NORMAL, False, u"宋体")
	font2 = wx.Font(10, wx.FONTFAMILY_DEFAULT, wx.FONTSTYLE_NORMAL,wx.FONTWEIGHT_NORMAL, False, u"宋体")
	frame.lstRemoteFile.SetFont(font1)
	frame.lstRemoteCourse.SetFont(font2)
	frame.txtLocalPath.SetFont(font1)
	frame.txtLocalPath.SetValue(global_var.setting['download_path'])
	frame.dirLocal.SetPath(global_var.setting['download_path'])
	frame.lstRemoteCourse.InsertColumn(0, u"课程名",format=wx.LIST_FORMAT_LEFT, width=450)
	frame.lstRemoteFile.InsertColumn(0, u"文件名",format=wx.LIST_FORMAT_LEFT, width=250)
	frame.lstRemoteFile.InsertColumn(1, u"文件大小",format=wx.LIST_FORMAT_LEFT, width=80)
	frame.lstRemoteFile.InsertColumn(2, u"上传时间",format=wx.LIST_FORMAT_LEFT, width=100)
	os.chdir(global_var.app_path)
	#控件初始化完毕
	
	#绑定事件
	EventBind(frame)
	
	#检测是否需要自动登录
	check()


def EventBind(frame):
	frame.Bind(wx.EVT_MENU, logItem_cmd, frame.logItem)
	frame.Bind(wx.EVT_MENU, setItem_cmd, frame.setItem)
	frame.Bind(wx.EVT_MENU, exitItem_cmd, frame.exitItem)
	frame.Bind(wx.EVT_LIST_ITEM_SELECTED,courseSelected_cmd,frame.lstRemoteCourse)
	frame.Bind(wx.EVT_LIST_ITEM_SELECTED,fileSelected_cmd,frame.lstRemoteFile)
	frame.Bind(wx.EVT_LIST_ITEM_DESELECTED,fileDeSelected_cmd,frame.lstRemoteFile)
	frame.Bind(wx.EVT_LIST_ITEM_RIGHT_CLICK,lstRemoteFile_RightClick,frame.lstRemoteFile)
	frame.Bind(wx.EVT_TREE_SEL_CHANGED, dirLocal_changed,frame.dirLocal.GetTreeCtrl())
	frame.Bind(wx.EVT_BUTTON, downCourseSelected,frame.btnDownCourse)
	frame.Bind(wx.EVT_BUTTON, downAllSelected, frame.btnDownAll)
	frame.Bind(wx.EVT_BUTTON, RefreshSelected, frame.btnRefresh)
	frame.Bind(wx.EVT_BUTTON, downSingleSelected, frame.btnDownSingle)
	