import sys, os
from platform import *
import gtk
import pango
import lib.consts
from common import CWindow
import traceback
import uu
from lib.config import config
from lib.Gui.dialogs import CWarningDialog

TEMP_FILE = 'tmp.frip'

class CfrmException(CWindow):
      
    widgets = ('tviewErrorLog','tviewSysInfo','btnCancel', 'btnSave',  'btnReport', 'ntbkException', 'lblMail', 'tviewUsrComment', 'chbtnIncludeProject',)
    name = 'frmException'

    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)
        # using connect, @event could not be used cause this dialog is used in lib.Gui.event
        self.btnReport.connect("clicked", self.OnBtnReportClicked, None)
        self.btnSave.connect("clicked", self.OnBtnSaveClicked, None)
        self.chbtnIncludeProject.connect("toggled", self.OnChbtnIncludeProjectToogled, None)
        self.lblMail.set_label("<span background='white'><b>"+ lib.consts.MAIL + "</b></span>")
        self.project = None
        self.append_project = False
        print 
        
    def OnChbtnIncludeProjectToogled(self, widget, event, data=None):
        if self.append_project == False:
            self.append_project = True
        else :
            self.append_project = False
        
    def OnBtnSaveClicked(self, widget, event, data=None):
        filedlg = gtk.FileChooserDialog(_('Save Error log'), self.form, gtk.FILE_CHOOSER_ACTION_SAVE, (gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))
        filter = gtk.FileFilter()
        filter.set_name(_("Text files"))
        filter.add_pattern('*.txt')
        filedlg.add_filter(filter)
        filter = gtk.FileFilter()
        filter.set_name(_("All files"))
        filter.add_pattern('*.*')
        filedlg.add_filter(filter)

        try:
            while True:
                if filedlg.run() == gtk.RESPONSE_OK: 
                    name =  filedlg.get_filter().get_name()
                    filename = filedlg.get_filename()

                    if '.' not in os.path.basename(filename):
                        filename += '.txt'

                    if not os.path.isdir(filename):
                        log_file = open(filename, 'w')
                        log_file.write(_('#UML.fri ERROR LOG:'))
                        buff = self.tviewErrorLog.get_buffer()
                        s, e = buff.get_bounds()
                        log_file.write(buff.get_text(s,e))

                        log_file.write(_('\n\n#SYSTEM INFORMATION:'))
                        buff = self.tviewSysInfo.get_buffer()
                        s, e = buff.get_bounds()
                        log_file.write(buff.get_text(s,e))

                        buff = self.tviewUsrComment.get_buffer()
                        s, e = buff.get_bounds()
                        text  = buff.get_text(s,e)
                        if len(text) > 0:
                            log_file.write(_('\n\n#USER COMMENTS:\n'))
                            log_file.write(text)

                        if self.append_project == True:
                            if self.project is not None:
                                log_file.write(_('\n\n#INCLUDED PROJECT:\n'))
                                path = config['/Paths/UserDir'] + TEMP_FILE
                                self.project.SaveProject(path)
                                in_file = open(path, 'r')
                                uu.encode(in_file, log_file)
                                in_file.close()
                                os.remove(path)


                        log_file.close()
                        filedlg.destroy()
                        CWarningDialog(None, 'File successfully saved as:\n\n' + filename).run()
                        return
                else:
                    filedlg.destroy()
                    return
        finally:
            #filedlg.destroy()
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
            buff.insert_with_tags_by_name(iter, _('\nFile '), "bold")
            buff.insert_with_tags_by_name(iter, filename,      "mono")
            buff.insert_with_tags_by_name(iter, _(' line '),   "bold")
            buff.insert_with_tags_by_name(iter, str(line_num), "mono")
            buff.insert_with_tags_by_name(iter, _(' in '),     "bold")
            buff.insert_with_tags_by_name(iter, fun_name + '\n' + text + '\n', "mono")
        #name and error
        buff.insert_with_tags_by_name(iter, '\n' + exctype.__name__  + ': ', "bold")
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
        buff.insert_with_tags_by_name(iter, machine(), "mono")
        buff.insert_with_tags_by_name(iter, _("\narchitecture:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, architecture()[0], "mono")
        ver =  str(python_version_tuple()[0]) +'.'+ str(python_version_tuple()[1])+'.'+ str(python_version_tuple()[2])
        buff.insert_with_tags_by_name(iter, _("\npython version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        ver =  str(gtk.gtk_version[0])+'.'+ str(gtk.gtk_version[1])+'.'+ str(gtk.gtk_version[2])
        buff.insert_with_tags_by_name(iter, _("\ngtk version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        ver =  str(gtk.pygtk_version[0])+'.'+ str(gtk.pygtk_version[1])+'.' + str(gtk.pygtk_version[2])
        buff.insert_with_tags_by_name(iter, _("\npygtk version:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, ver, "mono")
        buff.insert_with_tags_by_name(iter, _("\nplatform:\t\t"), "bold")
        buff.insert_with_tags_by_name(iter, platform(), "mono")



