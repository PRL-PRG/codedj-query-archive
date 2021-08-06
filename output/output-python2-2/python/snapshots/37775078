from lib.Depend.gtk2 import pango

from common import CWindow, event
from lib.config import config
import lib.consts

class CfrmAbout(CWindow):
    widgets = ('tviewCredits', 'lblAboutUmlfri', 'lbtnProjectWeb', )
    name = 'frmAbout'
    
    def Show(self):
        # clear the buffer
        buff = self.tviewCredits.get_buffer()
        s, e = buff.get_bounds()
        buff.delete(s,e)
        # set the about info
        text = _('<span size="xx-large">UML. FRI</span>\n<b>Version ') + self.application.GetVersion() + _('</b>\n\nFree python-based CASE tool.')
        self.lblAboutUmlfri.set_use_markup(True)
        self.lblAboutUmlfri.set_label(text)
        # set credits
        self.__SetCredits()
        # set web address 
        self.lbtnProjectWeb.set_uri(lib.consts.WEB)
        self.lbtnProjectWeb.set_label(lib.consts.WEB)
        self.form.run()
        self.Hide()
        
    @event("lbtnProjectWeb", "clicked")
    def OnLbtnProjectWebClicked(self, widget):
        from webbrowser import open_new
        open_new(lib.consts.WEB)
        self.form.run()
        self.Hide()

    def __SetCredits(self):
        buff = self.tviewCredits.get_buffer()
        tag_tab = buff.get_tag_table()
        iter = buff.get_end_iter()

        if tag_tab.lookup("bold") is None:
            buff.create_tag("bold", weight=pango.WEIGHT_BOLD)

        lines = [line for line in file(config['/Paths/Root']+'ABOUT') if not line.strip().startswith('-')]
 
        for line in lines:
            if ((line[0].isspace() == False) or (line.strip().startswith('Academic'))):
                buff.insert_with_tags_by_name(iter, line, "bold")
            else: buff.insert(iter, line)










