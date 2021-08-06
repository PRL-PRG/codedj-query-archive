#!/usr/bin/env python
# -*- coding: gbk -*-
import wx,global_var,GUItools

#LoginDialog
class LoginDialog(wx.Dialog):
    def __init__(self, *args, **kwds):
        # begin wxGlade: MyDialog.__init__
        kwds["style"] = wx.DEFAULT_DIALOG_STYLE
        wx.Dialog.__init__(self, *args, **kwds)
        self.label_1 = wx.StaticText(self, -1, u"网络学堂登陆")
        self.label_3 = wx.StaticText(self, -1, u"用户名")
        self.txtUserid = wx.TextCtrl(self, -1, "")
        self.label_4 = wx.StaticText(self, -1, u"密码")
        self.txtUserpass = wx.TextCtrl(self, -1, "", style=wx.TE_PASSWORD)
        self.autoSaved = wx.CheckBox(self, -1, "")
        self.label_5 = wx.StaticText(self, -1, u"记住密码，下次自动登录")
        self.btnLogin = wx.Button(self, -1, u"登录")
        self.btnExit = wx.Button(self, wx.ID_CANCEL, u"取消")
        self.info=[]

        self.__set_properties()
        self.__do_layout()
        # end wxGlade

    def __set_properties(self):
        # begin wxGlade: MyDialog.__set_properties
        self.SetTitle(u"登录")
        self.SetSize((345, 180))
        self.SetBackgroundColour(wx.Colour(255, 228, 159))
        self.label_1.SetMinSize((392, 21))
        self.label_1.SetBackgroundColour(wx.Colour(220, 255, 185))
        self.label_1.SetFont(wx.Font(13, wx.DEFAULT, wx.NORMAL, wx.BOLD, 0, ""))
        self.Bind(wx.EVT_BUTTON, self.btnLogin_onclick, self.btnLogin)
        # end wxGlade
    def btnLogin_onclick(self,event):
    	userid=self.txtUserid.GetValue()
    	userpass=self.txtUserpass.GetValue()
    	self.info=[userid,userpass]
    	if self.autoSaved.GetValue():
    		global_var.setting['userinfo'][0]=userid
    		global_var.setting['userinfo'][1]=userpass
    		global_var.setting['autologin']=True
    	else:
    		global_var.setting['userinfo'][0]=''
    		global_var.setting['userinfo'][1]=''
    		global_var.setting['autologin']=False
    	GUItools.saveSetting()
    	self.EndModal(wx.ID_OK)
        return

    def __do_layout(self):
        # begin wxGlade: MyDialog.__do_layout
        sizer_2 = wx.BoxSizer(wx.VERTICAL)
        sizer_4 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_5 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_3 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_2.Add(self.label_1, 0, wx.EXPAND, 0)
        sizer_3.Add(self.label_3, 0, 0, 0)
        sizer_3.Add(self.txtUserid, 0, 0, 0)
        sizer_3.Add(self.label_4, 0, 0, 0)
        sizer_3.Add(self.txtUserpass, 0, 0, 0)
        sizer_2.Add(sizer_3, 1, wx.EXPAND, 0)
        sizer_5.Add(self.autoSaved, 0, 0, 0)
        sizer_5.Add(self.label_5, 0, 0, 0)
        sizer_2.Add(sizer_5, 1, wx.EXPAND, 0)
        sizer_4.Add(self.btnLogin, 0, wx.ALIGN_CENTER_HORIZONTAL, 0)
        sizer_4.Add(self.btnExit, 0, wx.ALIGN_CENTER_HORIZONTAL, 0)
        sizer_2.Add(sizer_4, 1, wx.EXPAND, 0)
        self.SetSizer(sizer_2)
        self.Layout()
        # end wxGlade

# end of class LoginDialog


class AskDialog(wx.Dialog):
    def __init__(self, *args, **kwds):
        # begin wxGlade: AskDialog.__init__
        kwds["style"] = wx.DEFAULT_DIALOG_STYLE
        wx.Dialog.__init__(self, *args, **kwds)
        self.txtInfo = wx.TextCtrl(self, -1, "",size=(400, 200), style=wx.TE_MULTILINE|wx.TE_READONLY)
        self.btnYes = wx.Button(self, wx.ID_OK, u"是，我要覆盖掉")
        self.btnNo = wx.Button(self, wx.ID_CANCEL, u"哦，那就不下了")

        self.__set_properties()
        self.__do_layout()
        # end wxGlade

    def __set_properties(self):
        # begin wxGlade: AskDialog.__set_properties
        self.SetTitle("要覆盖吗?")
        self.SetSize((345, 180))
        
        # end wxGlade

    def __do_layout(self):
        # begin wxGlade: AskDialog.__do_layout
        sizer_2 = wx.BoxSizer(wx.VERTICAL)
        sizer_3 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_2.Add(self.txtInfo, 0, 0, 0)
        sizer_3.Add(self.btnYes, 0, 0, 0)
        sizer_3.Add(self.btnNo, 0, 0, 0)
        sizer_2.Add(sizer_3, 1, wx.EXPAND, 0)
        self.SetSizer(sizer_2)
        sizer_2.Fit(self)
        self.Layout()
        # end wxGlade

# end of class AskDialog

class SetDialog(wx.Dialog):
    def __init__(self, *args, **kwds):
        # begin wxGlade: SetDialog.__init__
        kwds["style"] = wx.DEFAULT_DIALOG_STYLE
        wx.Dialog.__init__(self, *args, **kwds)
        self.label_1 = wx.StaticText(self, -1, u"设置")
        self.static_line_1 = wx.StaticLine(self, -1)
        self.label_2 = wx.StaticText(self, -1, u"选择下载的默认路径")
        self.txtSetPath = wx.TextCtrl(self, -1, "")
        self.btnSetPath = wx.Button(self, -1, "......")
        #self.chkBox = wx.CheckBox(self, -1, u"在下载时自动去除文件名后的随机数")
        self.btnYes = wx.Button(self, -1, u"确定")
        self.btnNo = wx.Button(self, wx.ID_CANCEL, u"取消")

        self.__set_properties()
        self.__do_layout()
        # end wxGlade

    def __set_properties(self):
        # begin wxGlade: SetDialog.__set_properties
        self.SetTitle("设置")
        self.SetSize((300, 200))
        self.label_1.SetForegroundColour(wx.Colour(0, 25, 255))
        self.label_1.SetFont(wx.Font(14, wx.DEFAULT, wx.NORMAL, wx.BOLD, 0, ""))
        self.Bind(wx.EVT_BUTTON, self.btnYes_onclick, self.btnYes)
        self.Bind(wx.EVT_BUTTON, self.btnSetPath_onclick, self.btnSetPath)
        #self.txtSetPath.SetEditable(False)
        # end wxGlade

    def __do_layout(self):
        # begin wxGlade: SetDialog.__do_layout
        sizer_1 = wx.BoxSizer(wx.VERTICAL)
        sizer_4 = wx.BoxSizer(wx.VERTICAL)
        sizer_5 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_2 = wx.BoxSizer(wx.VERTICAL)
        sizer_3 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_1.Add(self.label_1, 0, 0, 0)
        sizer_1.Add(self.static_line_1, 0, wx.EXPAND, 0)
        sizer_2.Add(self.label_2, 0, 0, 0)
        sizer_3.Add(self.txtSetPath, 0, 0, 0)
        sizer_3.Add(self.btnSetPath, 0, 0, 0)
        sizer_2.Add(sizer_3, 1, wx.EXPAND, 0)
        sizer_1.Add(sizer_2, 1, wx.EXPAND, 0)
        sizer_5.Add(self.btnYes, 0, wx.ALIGN_CENTER_VERTICAL, 0)
        sizer_5.Add(self.btnNo, 0, wx.ALIGN_CENTER_VERTICAL, 0)
        sizer_4.Add(sizer_5, 1, wx.EXPAND, 0)
        sizer_1.Add(sizer_4, 1, wx.EXPAND, 0)
        self.SetSizer(sizer_1)
        self.Layout()
        # end wxGlade

    def btnYes_onclick(self,event):
    	path=self.txtSetPath.GetValue()
    	if(len(path)>3):
    		path += u'\\'
    	global_var.setting['download_path']=path
    	print global_var.setting['download_path']
    	global_var.txtLocalPath.SetValue(path)
    	global_var.dirLocal.SetPath(path)
    	GUItools.saveSetting()
    	self.EndModal(wx.ID_OK)
        return
    def btnSetPath_onclick(self,event):
    	if global_var.selDirDialog.ShowModal() == wx.ID_OK:
    		self.txtSetPath.SetValue(global_var.selDirDialog.GetPath())

if __name__ == "__main__":
    app = wx.PySimpleApp(0)
    wx.InitAllImageHandlers()
    frame_1 = MyFrame(None, -1, "")
    app.SetTopWindow(frame_1)
    frame_1.Show()
    app.MainLoop()
