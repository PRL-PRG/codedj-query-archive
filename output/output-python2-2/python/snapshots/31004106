import MainFrame,wx

class DownApp(wx.App):
    def OnInit(self):
        wx.InitAllImageHandlers()
        mainFrame = MainFrame.MainFrame(None, -1, "")
        self.SetTopWindow(mainFrame)
        mainFrame.Show()
        return 1

# end of class DownApp

if __name__ == "__main__":
    import gettext
    gettext.install("appDown") # replace with the appropriate catalog name

    appDown = DownApp(0)
    appDown.MainLoop()