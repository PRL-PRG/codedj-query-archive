import Acquisition, time
from plone.memoize import ram
from Products.Five.browser import BrowserView
from Products.Five.browser.pagetemplatefile import ViewPageTemplateFile
from Products.CMFCore.utils import getToolByName

class IndexAlphabetical(BrowserView):
    """View for displaying WhosWho Items by alphabet
    """
    template = ViewPageTemplateFile('templates/index_alphabetical.pt')

    def __call__(self):
        self.request.set('disable_border', True)
        context = Acquisition.aq_inner(self.context)

        portal_languages = getToolByName(context, 'portal_languages')

        self.lang = portal_languages.getPreferredLanguage()
        self.letter = str(self.request.get('letter', '')).upper()
        if len(self.letter)==2:
            try:
                self.letter = unicode(self.letter, 'utf-8')
            except:
                print "index_alphabetical:: could not convert to unicode"
        self.term_id = self.request.get('term_id', '')

        return self.template() 


#    def _getInitials_cachekey(method, self):
#        return ("whoswhoalphabeticalinitials", self.lang)
#
#    @ram.cache(_getInitials_cachekey)
    def getInitials(self):
        """ fetch the whole alphabet """
        portal_catalog = getToolByName(self.context, 'portal_catalog')
        wws = portal_catalog(portal_type="whoswho", Language=[self.lang, ''])
        initials = {}
        for ww in wws:
            term_id = caption = ww.Title
            if len(caption)==0:
                continue
            initial = caption[0].upper()
            section = initials.get(initial, [])
            section.append((caption, term_id, ww.getURL(), ww.Description))
            initials[initial] = section
        return initials


    def getAlphabet(self):
        """ fetch the whole alphabet """
        self.initials = self.getInitials()
        alphabet = self.initials.keys()
        alphabet.sort()
        self.alphabet = alphabet
        return alphabet


    def resultsByLetter(self, letter=None):
        """ returns the sorted resultmap by letter based on the search above """
        if letter is None:
            letter = self.getLetter()
        if letter == '':
            return [[], {}]

        section = self.initials.get(letter, [])    
        section.sort()
        return section


    def getLetter(self):
        return self.letter

    def getTerm_id(self):
        return self.term_id


