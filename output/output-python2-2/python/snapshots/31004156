#!/usr/bin/env python
# -*- coding: gbk -*-

#为GUI提供必要的类和函数
import os, sys, platform,Dialogs
import global_var,download
from glob import glob
import wx

def Refresh():
	try:
		global_var.app_stat="Refresh"
		global_var.statusBar.SetStatusText(u"正在获取网络学堂文件列表")
		global_var.list=download.getlist()
		global_var.app_stat="Idle"
		ShowCourse()
		ShowFile(global_var.current_courseindex)
		print 'OK'
	except:
		global_var.statusBar.SetStatusText(u":（ 列表刷新失败")
	global_var.statusBar.SetStatusText(u"列表已经刷新")
	return

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
	if(glob('userinfo.txt')):
		f=open('userinfo.txt')
		up=f.read().split('\n')
		if len(up)>1:
			global_var.userid=up[0]
			global_var.userpass=up[1]
			print '正在登录，请稍侯...'
			Login()
	return

def Login():
	global_var.statusBar.SetStatusText(u"正在登录")
	Refresh()
	global_var.statusBar.SetStatusText(u"空闲")
	return

def logItem_cmd(event):
	ret = global_var.logDialog.ShowModal()
	if ret==wx.ID_OK:
		global_var.userid=global_var.logDialog.info[0]
		global_var.userpass=global_var.logDialog.info[1]
		Login()

def courseSelected_cmd(event):
	index=event.m_itemIndex
	global_var.current_courseindex=index
	ShowFile(index)

def fileSelected_cmd(event):
	global_var.current_fileindex=event.m_itemIndex

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
	fileindex=global_var.current_fileindex
	isexist=download.IsExist(global_var.current_courseindex,fileindex)
	if isexist:
		global_var.file_askinfo="文件"+global_var.list[global_var.current_courseindex][2][fileindex]['file_realname']+"已经存在于"+isexist+"，要覆盖吗？"
		global_var.askDialog.txtInfo.SetValue(global_var.file_askinfo)
		ret = global_var.askDialog.ShowModal()
		if ret==wx.ID_OK:
			download.DownSingle(global_var.current_courseindex,fileindex)
		else:
			global_var.statusBar.SetStatusText(u"下载已取消")
	else:
		download.DownSingle(global_var.current_courseindex,fileindex)
	
def Frame__init(frame):
	
	global_var.main_frame=frame
	global_var.askDialog=Dialogs.AskDialog(global_var.main_frame)
	global_var.logDialog=Dialogs.LoginDialog(global_var.main_frame)
	global_var.txtRemoteCourse=frame.txtRemoteCourse
	global_var.txtRemoteCourse.SetEditable(False)
	global_var.lstRemoteFile=frame.lstRemoteFile
	global_var.lstRemoteCourse=frame.lstRemoteCourse
	global_var.dirLocal=frame.dirLocal
	global_var.txtLocalPath=frame.txtLocalPath
	global_var.conn=download.MyCon()
	global_var.statusBar=frame.statusBar
	
	font1 = wx.Font(9, wx.FONTFAMILY_DEFAULT, wx.FONTSTYLE_NORMAL,wx.FONTWEIGHT_NORMAL, False, u"宋体")
	font2 = wx.Font(10, wx.FONTFAMILY_DEFAULT, wx.FONTSTYLE_NORMAL,wx.FONTWEIGHT_NORMAL, False, u"宋体")
	frame.lstRemoteFile.SetFont(font1)
	frame.lstRemoteCourse.SetFont(font2)
	frame.txtLocalPath.SetFont(font1)
	frame.lstRemoteCourse.InsertColumn(0, u"课程名",format=wx.LIST_FORMAT_LEFT, width=450)
	frame.lstRemoteFile.InsertColumn(0, u"文件名",format=wx.LIST_FORMAT_LEFT, width=250)
	frame.lstRemoteFile.InsertColumn(1, u"文件大小",format=wx.LIST_FORMAT_LEFT, width=80)
	frame.lstRemoteFile.InsertColumn(2, u"上传时间",format=wx.LIST_FORMAT_LEFT, width=100)
	os.chdir(global_var.app_path)

	EventBind(frame)
	check()


def EventBind(frame):
	frame.Bind(wx.EVT_MENU, logItem_cmd, frame.logItem)
	frame.Bind(wx.EVT_LIST_ITEM_SELECTED,courseSelected_cmd,frame.lstRemoteCourse)
	frame.Bind(wx.EVT_LIST_ITEM_SELECTED,fileSelected_cmd,frame.lstRemoteFile)
	frame.Bind(wx.EVT_TREE_SEL_CHANGED, dirLocal_changed,frame.dirLocal.GetTreeCtrl())
	frame.Bind(wx.EVT_BUTTON, downCourseSelected,frame.btnDownCourse)
	frame.Bind(wx.EVT_BUTTON, downAllSelected, frame.btnDownAll)
	frame.Bind(wx.EVT_BUTTON, RefreshSelected, frame.btnRefresh)
	frame.Bind(wx.EVT_BUTTON, downSingleSelected, frame.btnDownSingle)