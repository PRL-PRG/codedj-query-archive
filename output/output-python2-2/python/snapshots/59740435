import Acquisition, time
from plone.memoize import ram
from Products.Five.browser import BrowserView
from Products.Five.browser.pagetemplatefile import ViewPageTemplateFile
from Products.CMFCore.utils import getToolByName
from zope.component import getMultiAdapter

class IndexKeyword(BrowserView):
    """View for displaying WhosWho Items by alphabet
    """
    template = ViewPageTemplateFile('templates/index_keyword.pt')

    def __call__(self):
        self.request.set('disable_border', True)
        context = Acquisition.aq_inner(self.context)

        portal_languages = getToolByName(context, 'portal_languages')

        self.lang = portal_languages.getPreferredLanguage()
        self.searchterm = str(self.request.get('searchterm', ''))

        return self.template()


    def getLang(self):
        if not getattr(self, 'lang', None):
            portal_languages = getToolByName(self.context, 'portal_languages')
            self.lang = portal_languages.getPreferredLanguage()
        return self.lang


    def getInitials(self):
        """ fetch all keywords that have content associated """
        paths = list()
        portal_state = getMultiAdapter((self.context, self.request), name=u'plone_portal_state')
        portal_catalog = getToolByName(self.context, 'portal_catalog')
        navigation_root_path = portal_state.navigation_root_path()
        paths.append(navigation_root_path)
        try:
            navigation_root = portal_state.portal().restrictedTraverse(navigation_root_path)
            canonical_path = '/'.join(navigation_root.getCanonical().getPhysicalPath())
            paths.append(canonical_path)
        except:
            pass

        initials = {}
        pv = getToolByName(self, 'portal_vocabularies')
        VOCAB = getattr(pv, 'OSHAMetadata', None) 
        vocabDict = VOCAB.getVocabularyDict(VOCAB)
        whoswhotype = vocabDict.get('WhosWhoType', {})
        for term_id in whoswhotype[1].keys():
            caption = whoswhotype[1][term_id][0]
            if len(caption)==0:
                continue
            res = portal_catalog(portal_type="whoswho"
                    , Language=[self.getLang(),'']
                    , osha_metadata=term_id
                    , path=paths
                    )
            if len(res):
                initials[term_id] = dict(caption=caption, res=res)
        return initials


    def getKeywords(self):
        """ fetch all keywords"""
        self.initials = self.getInitials()
        keywords = [dict(id=k, title=self.initials[k]['caption']) for k in self.initials.keys()]
        keywords.sort(lambda a,b: cmp(a['title'], b['title']))
        return keywords


    def resultsBySearchterm(self, searchterm=None):
        """ returns the sorted resultmap by letter based on the search above """
        if searchterm is None:
            searchterm = self.getSearchterm()
        if searchterm == '':
            return [[], {}]

        portal_catalog = getToolByName(self.context, 'portal_catalog')
        # search in the navigation root of the currently selected language and in the canonical path
        # with Language = preferredLanguage or neutral
        paths = list()
        portal_state = getMultiAdapter((self.context, self.request), name=u'plone_portal_state')
        navigation_root_path = portal_state.navigation_root_path()
        paths.append(navigation_root_path)
        try:
            navigation_root = portal_state.portal().restrictedTraverse(navigation_root_path)
            canonical_path = '/'.join(navigation_root.getCanonical().getPhysicalPath())
            paths.append(canonical_path)
        except:
            pass

        res = portal_catalog(portal_type="whoswho"
                , Language=[self.getLang(),'']
                , osha_metadata=searchterm
                , path=paths
                )
        return res


    def getSearchterm(self):
        """ """
        return self.searchterm

    def getTermById(self, term_id=''):
        """ """
        if not term_id:
            term_id = self.getSearchterm()
        return self.initials.has_key(term_id) and self.initials[term_id]['caption'] or ''

    def getHeading(self):
        context = Acquisition.aq_base(Acquisition.aq_inner(self.context))
        return context.Title()

    def getBodyText(self):
        """ returns body text of collection  if present """
        context = Acquisition.aq_base(Acquisition.aq_inner(self.context))
        text = getattr(context, 'getText', None) and context.getText() or ''
        return text