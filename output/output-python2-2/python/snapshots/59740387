import Acquisition, time
from plone.memoize import ram
from Products.Five.browser import BrowserView
from Products.Five.browser.pagetemplatefile import ViewPageTemplateFile
from Products.CMFCore.utils import getToolByName
from zope.component import getMultiAdapter

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
        self.initials = self.createInitials()

        return self.template() 


    def getLang(self):
        if not getattr(self, 'lang', None):
            portal_languages = getToolByName(self.context, 'portal_languages')
            self.lang = portal_languages.getPreferredLanguage()
        return self.lang

#    def _createInitials_cachekey(method, self):
#        preflang = getToolByName(self.context, 'portal_languages').getPreferredLanguage()
#        portal_state = getMultiAdapter((self.context, self.request), name=u'plone_portal_state')
#        navigation_root_path = portal_state.navigation_root_path()
#        return ("whoswhoalphabeticalinitials", self.getLang(), navigation_root_path, time.time()//60 * 60)
#
#    @ram.cache(_createInitials_cachekey)
    def createInitials(self):
        """ fetch the whole alphabet """
        portal_catalog = getToolByName(self.context, 'portal_catalog')
#        import pdb; pdb.set_trace() 
        # search in the navigation root of the currently selected language and in the canonical path
        # with Language = preferredLanguage or neutral
        paths = list()
        portal_state = getMultiAdapter((self.context, self.request), name=u'plone_portal_state')
        navigation_root_path = portal_state.navigation_root_path()
        paths.append(navigation_root_path)
        try:
            navigation_root = portal_state.portal().restrictedTraverse(navigation_root_path)
            canonical_path = '/'.join(navigation_root.getCanonical().getPhysicalPath())
            if canonical_path!=navigation_root_path:
                paths.append(canonical_path)
        except:
            pass
        wws = portal_catalog(portal_type="whoswho" 
                   , Language=[self.getLang(), '']
                   , path=paths
               )
        initials = {}
        for ww in wws:
            term_id = caption = ww.Title
            if len(caption)==0:
                continue
            initial = caption[0].upper()
            section = initials.get(initial, [])
            section.append((unicode(caption, 'utf-8'), 
                        unicode(term_id, 'utf-8'), 
                        unicode(ww.getURL(), 'utf-8'), 
                        unicode(ww.Description, 'utf-8')))
            initials[initial] = section
        self.initials = initials
        return initials


    def getAlphabet(self):
        """ fetch the whole alphabet """
        if not getattr(self, 'initials', None):
            self.initials = self.createInitials()
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

        if not getattr(self, 'initials', None):
            self.initials = self.createInitials()
        section = self.initials.get(letter, [])    
        section.sort()
        return section


    def getLetter(self):
        return self.letter

    def getTerm_id(self):
        return self.term_id

    def getHeading(self):
        context = Acquisition.aq_base(Acquisition.aq_inner(self.context))
        return context.Title()

    def getBodyText(self):
        """ returns body text of collection  if present """
        context = Acquisition.aq_base(Acquisition.aq_inner(self.context))
        text = getattr(context, 'getText', None) and context.getText() or ''
        return text
