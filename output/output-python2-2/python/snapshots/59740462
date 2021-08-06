import Acquisition, time
from plone.memoize import ram
from Products.Five.browser import BrowserView
from Products.Five.browser.pagetemplatefile import ViewPageTemplateFile
from Products.CMFCore.utils import getToolByName

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


#    def _getInitials_cachekey(method, self):
#        return ("whoaswhotypeinitials", self.lang)
#
#    @ram.cache(_getInitials_cachekey)
    def getKeywords(self):
        """ fetch all keywords"""
        pv = getToolByName(self, 'portal_vocabularies')
        VOCAB = getattr(pv, 'WhosWhoType', None)
        keywords = list()
        keywordsById = dict()
        for term_id, caption in VOCAB.getVocabularyDict().items():
            if len(caption)==0:
                continue
            keywords.append(dict(id=term_id, title=caption))
            keywordsById[term_id] = caption

        self.keywordsById = keywordsById
        keywords.sort(lambda a,b: cmp(a['title'], b['title']))
        return keywords


    def resultsBySearchterm(self, searchterm=None):
        """ returns the sorted resultmap by letter based on the search above """
        if searchterm is None:
            searchterm = self.getSearchterm()
        if searchterm == '':
            return [[], {}]

#        import pdb; pdb.set_trace()
        portal_catalog = getToolByName(self, 'portal_catalog')
        res = portal_catalog(portal_type="whoswho", Language=[self.lang,''], getWhoswho_type=searchterm)
        return res


    def getSearchterm(self):
        """ """
        return self.searchterm

    def getTermById(self, term_id=''):
        """ """
        if not term_id:
            term_id = self.getSearchterm()
        return self.keywordsById.get(term_id, '')