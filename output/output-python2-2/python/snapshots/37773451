import common

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
        lines = [line for line in file('ABOUT') if not line.strip().startswith('-')]
               
        for line in lines:
            line = line.replace('<', '&lt;').replace('>', '&gt;')
            if ((line[0].isspace() == False) or (line.strip().startswith('Academic'))):
                line = '<b>' + line[:-1] + '</b>' + line[-1]
            about += line
            
        return about