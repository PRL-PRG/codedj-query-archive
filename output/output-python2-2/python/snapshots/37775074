from lib.Depend.gtk2 import gtk
from lib.Depend.gtk2 import pango
import lib.consts
from common import CWindow
from lib.config import config
from lib.Gui.dialogs import CWarningDialog
import sys, os, time, tarfile, platform, traceback, cStringIO, datetime, urllib, urllib2

EXCEPTION_PROJECT_FILE = 'error.frip'


class CfrmException(CWindow):
      
    widgets = ('tviewErrorLog','tviewSysInfo','btnCancel', 'btnSend',  'btnReport', 'ntbkException', 'lblMail', 'tviewUsrComment', 'chbtnIncludeProject',)
    name = 'frmException'


    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)
        # using connect, @event could not be used cause this dialog is used in lib.Gui.event
        self.btnReport.connect("clicked", self.OnBtnReportClicked, None)
        self.btnSend.connect("clicked", self.OnBtnSendClicked, None)
        self.chbtnIncludeProject.connect("toggled", self.OnChbtnIncludeProjectToogled, None)
        self.lblMail.set_label("<span background='white'><b>"+ lib.consts.MAIL + "</b></span>")
        self.project = None
        self.append_project = True

       
    def OnChbtnIncludeProjectToogled(self, widget, event, data=None):
        if self.append_project == False:
            self.append_project = True
        else :
            self.append_project = False
 
       
    def OnBtnSendClicked(self, widget, event, data=None):
        try:
            log_tar_path = config['/Paths/UserDir'] + str(time.time()) + '.tar'  # path to tar file
            tar = tarfile.open(log_tar_path, "w")
            tarinfo = tarfile.TarInfo()
            
            # tarinfo properties
            tarinfo.name = 'error.log'
            tarinfo.type = tarfile.REGTYPE
            tarinfo.mode = 0600
            tarinfo.mtime = time.mktime(datetime.datetime.now().timetuple())
          
            # striongIO for traceback
            buff = self.tviewErrorLog.get_buffer()
            s, e = buff.get_bounds()
            io_buff =  buff.get_text(s,e)
            iof = cStringIO.StringIO(io_buff)
            iof.seek(0)
            tarinfo.size = len(io_buff)
            tar.addfile(tarinfo, iof)
            iof.close()
           
            # striongIO for sys info
            tarinfo.name = 'sys_info.log'
            buff = self.tviewSysInfo.get_buffer()
            s, e = buff.get_bounds()
            io_buff =  buff.get_text(s,e)
            iof = cStringIO.StringIO(io_buff)
            iof.seek(0)
            tarinfo.size = len(io_buff)
            tar.addfile(tarinfo, iof)
            iof.close()

            # striongIO for comment
            tarinfo.name = 'comment.log'
            buff = self.tviewUsrComment.get_buffer()
            s, e = buff.get_bounds()
            io_buff =  buff.get_text(s,e)
 
            if len(io_buff) > 0:
                iof = cStringIO.StringIO(io_buff)
                iof.seek(0)
                tarinfo.size = len(io_buff)
                tar.addfile(tarinfo, iof)
                iof.close()

            if self.append_project == True:
                if self.project is not None:
                    
                    log_project_path = config['/Paths/UserDir'] + EXCEPTION_PROJECT_FILE
                    self.project.SaveProject(log_project_path)
                    tar.add(log_project_path,EXCEPTION_PROJECT_FILE)
                    os.remove(log_project_path)
            
            tar.close() # closing the tar file, now we have all we need for sending

            ### sending....testing ###
            try:
                file_to_send = open(log_tar_path, 'r')            
                string_to_send = file_to_send.read().encode('base64_codec')
                file_to_send.close()
                
                values = {'upfile' : string_to_send}
                data = urllib.urlencode(values)
                req = urllib2.Request(lib.consts.ERROR_LOG_ADDRESS, data)
                response = urllib2.urlopen(req)
               
                # if everything goes well
                if response.code == 200:
                    t = _('File successfully send...\n\nThank you for helping improving UML. FRI')
                    self.btnSend.set_sensitive(False)
                
                # not so well, but at least we could get a response :)                
                else:
                    t = _('Uups! Sending was not successfull.\nServer response:\n ') + str(response.code) + ' ' + response.msg
               
            except urllib2.URLError, e :
                t = _('Uups! An error during sending occured:\n') + str(e).replace('<','').replace('>','')
            
            os.remove(log_tar_path)     # remove the tar-ed log file
            CWarningDialog(None, t).run()
            return

        finally:
            self.form.run()
            self.Hide()


    def OnBtnReportClicked(self, widget, event, data=None):
        from webbrowser import open_new
        open_new(lib.consts.WEB)
        self.form.run()
        self.Hide()


    def Show(self):
        self.form.run()
        buff = self.tviewErrorLog.get_buffer()
        s, e = buff.get_bounds()
        buff.delete(s,e)
        buff = self.tviewSysInfo.get_buffer()
        s, e = buff.get_bounds()
        buff.delete(s,e)
        buff = self.tviewUsrComment.get_buffer()
        s, e = buff.get_bounds()
        buff.delete(s,e)
        self.Hide()


    def SetErrorLog(self):
        exctype, value, tb = sys.exc_info()
        buff = self.tviewErrorLog.get_buffer()
        tag_tab = buff.get_tag_table()
        iter = buff.get_end_iter()
        
        if tag_tab.lookup("bold") is None:
            buff.create_tag("bold", weight=pango.WEIGHT_BOLD, family="monospace")
        if tag_tab.lookup("mono") is None:
            buff.create_tag("mono", family="monospace")

        for filename, line_num, fun_name, text in traceback.extract_tb(tb)[1:]:
            buff.insert_with_tags_by_name(iter, _('File '), "bold")
            buff.insert_with_tags_by_name(iter, filename,      "mono")
            buff.insert_with_tags_by_name(iter, _(' line '),   "bold")
            buff.insert_with_tags_by_name(iter, str(line_num), "mono")
            buff.insert_with_tags_by_name(iter, _(' in '),     "bold")
            buff.insert_with_tags_by_name(iter, (fun_name or "") + '\n  ' + (text or "") + '\n\n', "mono")
        #name and error
        buff.insert_with_tags_by_name(iter, exctype.__name__  + ': ', "bold")
        buff.insert_with_tags_by_name(iter, str(value), "mono")


    def SetSystemInfo(self):
        buff = self.tviewSysInfo.get_buffer()  
        tag_tab = buff.get_tag_table()

        if tag_tab.lookup("bold") is None:
            buff.create_tag("bold", weight=pango.WEIGHT_BOLD, family="monospace")
        if tag_tab.lookup("mono") is None:
            buff.create_tag("mono", family="monospace")
 
        iter = buff.get_iter_at_offset(0)
        buff.insert_with_tags_by_name(iter, _("\nmachine:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, platform.machine(), "mono")
        buff.insert_with_tags_by_name(iter, _("\narchitecture:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, platform.architecture()[0], "mono")
        ver =  str(platform.python_version_tuple()[0]) +'.'+ str(platform.python_version_tuple()[1])+'.'+ str(platform.python_version_tuple()[2])
        buff.insert_with_tags_by_name(iter, _("\npython version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        ver =  str(gtk.gtk_version[0])+'.'+ str(gtk.gtk_version[1])+'.'+ str(gtk.gtk_version[2])
        buff.insert_with_tags_by_name(iter, _("\ngtk version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        ver =  str(gtk.pygtk_version[0])+'.'+ str(gtk.pygtk_version[1])+'.' + str(gtk.pygtk_version[2])
        buff.insert_with_tags_by_name(iter, _("\npygtk version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        buff.insert_with_tags_by_name(iter, _("\nplatform:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, platform.platform(), "mono")

