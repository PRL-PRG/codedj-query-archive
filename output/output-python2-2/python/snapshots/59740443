import Acquisition
from Products.Five.browser import BrowserView
from Products.CMFCore.utils import getToolByName
from kss.core.ttwapi import ( startKSSCommands, getKSSCommandSet, renderKSSCommands )
import logging
logger = logging.getLogger('osha.whoswho')
from zope.component import getMultiAdapter

class kssListByLetterLoad(BrowserView):
    """called by kss, loads whoswhos by letter
    """

    def __call__(self):
        letter = self.request.get('letter', None)
        startKSSCommands(self, self.request)
#        logger.info(letter)
        whoswho_alphabetical = getMultiAdapter((self.context, self.request), name=u'whoswho_alphabetical') 
        res = whoswho_alphabetical.resultsByLetter(letter)
        restag = u"""<dl class="keylist" >"""
        for r in res:
            restag += u"""
                    <dt><a href="%(url)s" rel="nofollow">%(title)s</a>
                    </dt>
                    <dd>%(description)s</dd>
                    """ % dict(url=r[2], title=r[0], description=r[3])
        restag += u"""</dl>"""

        core = getKSSCommandSet('core')
        core.replaceInnerHTML('#slc-index-results h2', letter)
        core.replaceInnerHTML('#slc-index-results #resultcolA', restag)

        return renderKSSCommands() 


class kssListByTypeLoad(BrowserView):
    """ called by kss, load whoswho by type
    """

    def __call__(self):
        term = self.request.get('term', '')
        caption = self.request.get('caption', '')
        try:
            caption = unicode(caption, 'utf-8')
        except Exception, err:
            print "could not make unicode out of caption: %s" %err
        startKSSCommands(self, self.request)
#        logger.info(term)
        whoswho_type = getMultiAdapter((self.context, self.request), name=u'whoswho_type') 
        res = whoswho_type.resultsBySearchterm(term)

        restag = u"""<dl class="keylist" >"""
        for r in res:
            restag += u"""
                    <dt><a href="%(url)s" rel="nofollow">%(title)s</a>
                    </dt>
                    <dd>%(description)s</dd>
                    """ % dict(url=r.getURL(), title=r.Title, description=r.Description)
        restag += u"""</dl>"""

        core = getKSSCommandSet('core')
        core.replaceInnerHTML('#slc-index-results h2', caption)
        core.replaceInnerHTML('#slc-index-results #resultcolA', restag)

        return renderKSSCommands()
