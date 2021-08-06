import common
from lib.config import config

class CfrmAbout(common.CWindow):
    widgets = ('labelAbout',)
    name = 'frmAbout'
    
    def Show(self):
        self.labelAbout.set_use_markup(True)
        nText = self.__GetAboutText()
        self.labelAbout.set_label(nText)
        self.form.run()
        self.Hide()

    def __GetAboutText(self):
        about = ''
        lines = [line for line in file(config['/Paths/Root']+'ABOUT') if not line.strip().startswith('-')]
               
        for line in lines:
            line = line.replace('<', '&lt;').replace('>', '&gt;').rstrip('\r\n')
            if ((line[0].isspace() == False) or (line.strip().startswith('Academic'))):
                line = '<b>' + line + '</b>'
            about += line+'\n'
            
        return about