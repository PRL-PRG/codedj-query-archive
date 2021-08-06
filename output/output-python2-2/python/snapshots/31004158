#!/usr/bin/env python
# -*- coding: gbk -*-
import wx,global_var

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
        #self.Bind(wx.EVT_BUTTON, self.btnExit_onclick, self.btnExit)
        # end wxGlade
    def btnLogin_onclick(self,event):
    	userid=self.txtUserid.GetValue()
    	userpass=self.txtUserpass.GetValue()
    	self.info=[userid,userpass]
    	self.autoSaved
    	f=open(global_var.app_path+'//userinfo.txt','w')
    	f.write(userid+'\n'+userpass)
    	f.close()
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


if __name__ == "__main__":
    app = wx.PySimpleApp(0)
    wx.InitAllImageHandlers()
    frame_1 = MyFrame(None, -1, "")
    app.SetTopWindow(frame_1)
    frame_1.Show()
    app.MainLoop()
