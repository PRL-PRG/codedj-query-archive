#!/usr/bin/env python
# -*- coding: gbk -*-
import wx,global_var,GUItools,os

class LogDialog(wx.Dialog):
    def __init__(self, *args, **kwds):
        # begin wxGlade: LogDialog.__init__
        kwds["style"] = wx.DEFAULT_DIALOG_STYLE
        wx.Dialog.__init__(self, *args, **kwds)
        self.sizer_13_staticbox = wx.StaticBox(self, -1, _(u"登录"))
        self.sizer_10_staticbox = wx.StaticBox(self, -1, _(u"路径设置"))
        self.label_1 = wx.StaticText(self, -1, _(u"设置默认下载文件夹"))
        self.txtSetDownPath = wx.TextCtrl(self, -1, "")
        self.btnSetDownPath = wx.Button(self, -1, _("......"))
        self.label_2 = wx.StaticText(self, -1, _(u"设置打印文件夹      "))
        self.txtSetPrintPath = wx.TextCtrl(self, -1, "")
        self.btnSetPrintPath = wx.Button(self, -1, _("......"))
        self.btnSaveSet = wx.Button(self, -1, _(u"保存以上设置"))
        self.btnExitSet = wx.Button(self,wx.ID_CANCEL, _(u"退出设置    "))
        self.label_3 = wx.StaticText(self, -1, _(u"用户名"))
        self.txtUserid = wx.TextCtrl(self, -1, "")
        self.label_4 = wx.StaticText(self, -1, _(u"密码   "))
        self.txtUserpass = wx.TextCtrl(self, -1, "", style=wx.TE_PASSWORD)
        self.autoSaved = wx.CheckBox(self, -1, _(u"记住我，下次自动登录"))
        self.btnLogin = wx.Button(self, -1, _(u"登录"))
        self.btnExitLogin = wx.Button(self, wx.ID_CANCEL, _(u"退出"))

        self.__set_properties()
        self.__do_layout()
        ###############################################################
        #对话框的绑定函数
        
        self.Bind(wx.EVT_BUTTON, self.btnLogin_handle, self.btnLogin)
        self.Bind(wx.EVT_BUTTON, self.btnSaveSet_handle, self.btnSaveSet)
        self.Bind(wx.EVT_BUTTON, self.btnSetDownPath_onclick, self.btnSetDownPath)
        self.Bind(wx.EVT_BUTTON, self.btnSetPrintPath_onclick, self.btnSetPrintPath)
        # end wxGlade

    def __set_properties(self):
        # begin wxGlade: LogDialog.__set_properties
        self.SetTitle(_(u"设置登录"))
        self.SetSize((380, 300))
        # end wxGlade

    def __do_layout(self):
        # begin wxGlade: LogDialog.__do_layout
        sizer_6 = wx.BoxSizer(wx.VERTICAL)
        sizer_13 = wx.StaticBoxSizer(self.sizer_13_staticbox, wx.HORIZONTAL)
        sizer_20 = wx.BoxSizer(wx.VERTICAL)
        sizer_23 = wx.BoxSizer(wx.VERTICAL)
        sizer_24 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_22 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_21 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_10 = wx.StaticBoxSizer(self.sizer_10_staticbox, wx.HORIZONTAL)
        sizer_16 = wx.BoxSizer(wx.VERTICAL)
        sizer_19 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_18 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_17 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_17.Add(self.label_1, 0, 0, 0)
        sizer_17.Add(self.txtSetDownPath, 0, 0, 0)
        sizer_17.Add(self.btnSetDownPath, 0, 0, 0)
        sizer_16.Add(sizer_17, 1, wx.EXPAND, 0)
        sizer_18.Add(self.label_2, 0, 0, 0)
        sizer_18.Add(self.txtSetPrintPath, 0, 0, 0)
        sizer_18.Add(self.btnSetPrintPath, 0, 0, 0)
        sizer_16.Add(sizer_18, 1, wx.EXPAND, 0)
        sizer_19.Add(self.btnSaveSet, 0, 0, 0)
        sizer_19.Add(self.btnExitSet, 0, 0, 0)
        sizer_16.Add(sizer_19, 1, wx.EXPAND, 0)
        sizer_10.Add(sizer_16, 1, wx.EXPAND, 0)
        sizer_6.Add(sizer_10, 1, wx.EXPAND, 0)
        sizer_21.Add(self.label_3, 0, 0, 0)
        sizer_21.Add(self.txtUserid, 0, 0, 0)
        sizer_20.Add(sizer_21, 1, wx.EXPAND, 0)
        sizer_22.Add(self.label_4, 0, 0, 0)
        sizer_22.Add(self.txtUserpass, 0, 0, 0)
        sizer_20.Add(sizer_22, 1, wx.EXPAND, 0)
        sizer_23.Add(self.autoSaved, 0, 0, 0)
        sizer_24.Add(self.btnLogin, 0, 0, 0)
        sizer_24.Add(self.btnExitLogin, 0, 0, 0)
        sizer_23.Add(sizer_24, 1, wx.EXPAND, 0)
        sizer_20.Add(sizer_23, 1, wx.EXPAND, 0)
        sizer_13.Add(sizer_20, 1, wx.EXPAND, 0)
        sizer_6.Add(sizer_13, 1, wx.EXPAND, 0)
        self.SetSizer(sizer_6)
        self.Layout()
        # end wxGlade
        
    def btnLogin_handle(self,event):
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
    
    def btnSaveSet_handle(self,event):
        if os.path.isdir(self.txtSetDownPath.GetValue()) and os.path.isdir(self.txtSetPrintPath.GetValue()):
            global_var.setting['download_path']=self.txtSetDownPath.GetValue()
            global_var.setting['print_path']=self.txtSetPrintPath.GetValue()
            GUItools.saveSetting()
            global_var.warnDialog.txtInfo.SetValue(u'默认路径设置完成')
            global_var.warnDialog.ShowModal()
        else:
            global_var.warnDialog.txtInfo.SetValue(u'你设置的路径不正确，请重新设置')
            global_var.warnDialog.ShowModal()
            self.txtSetDownPath.SetValue(global_var.setting['download_path'])
            self.txtSetPrintPath.SetValue(global_var.setting['print_path'])

    def btnSetDownPath_onclick(self,event):
        if global_var.selDirDialog.ShowModal() == wx.ID_OK:
    	    self.txtSetDownPath.SetValue(global_var.selDirDialog.GetPath())
    	    

    def btnSetPrintPath_onclick(self,event):
        if global_var.selDirDialog.ShowModal() == wx.ID_OK:
    	    self.txtSetPrintPath.SetValue(global_var.selDirDialog.GetPath())

# end of class LogDialog


#此询问框已经不用
'''
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
'''
class WarnDialog(wx.Dialog):
    def __init__(self, *args, **kwds):
        # begin wxGlade: AskDialog.__init__
        kwds["style"] = wx.DEFAULT_DIALOG_STYLE
        wx.Dialog.__init__(self, *args, **kwds)
        self.txtInfo = wx.TextCtrl(self, -1, "",size=(400, 200), style=wx.TE_MULTILINE|wx.TE_READONLY)
        self.btnYes = wx.Button(self, wx.ID_OK, u"确定")
        self.__set_properties()
        self.__do_layout()
        # end wxGlade

    def __set_properties(self):
        # begin wxGlade: AskDialog.__set_properties
        self.SetTitle("信息")
        self.SetSize((345, 180))
        
        # end wxGlade

    def __do_layout(self):
        # begin wxGlade: AskDialog.__do_layout
        sizer_2 = wx.BoxSizer(wx.VERTICAL)
        sizer_3 = wx.BoxSizer(wx.HORIZONTAL)
        sizer_2.Add(self.txtInfo, 0, 0, 0)
        sizer_3.Add(self.btnYes, 0, 0, 0)
        sizer_2.Add(sizer_3, 1, wx.EXPAND, 0)
        self.SetSizer(sizer_2)
        sizer_2.Fit(self)
        self.Layout()
        # end wxGlade
