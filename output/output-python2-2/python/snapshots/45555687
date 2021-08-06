#!/usr/bin/env python
"""
Monkey patch for hgext.keyword
for showing tiprev keyword.

This extensions MUST be BEFORE hgext.keyword in hgrc
"""

import hgext.keyword
import mercurial.templatefilters

def get_tip_rev(rep):
    chlog = rep.changelog
    return chlog.count() - 1

def patched_substitute(self, data, path, node, subfunc):
    def kwsub(match):
        kw = match.group(1)
        if kw == 'tiprev':
            tiprev = get_tip_rev(self.repo)
            ekw = str(tiprev)
        else:
            # original kwsub
            self.ct.use_template(self.templates[kw])
            self.ui.pushbuffer()
            self.ct.show(changenode=node, 
                         root=self.repo.root,
                         file=path)
            ekw = mercurial.templatefilters.firstline(self.ui.popbuffer())
        return '$%s: %s $' % (kw, ekw)
    return subfunc(kwsub, data)

if not getattr(hgext.keyword.kwtemplater, '_orig_substitute', False):
    # here we go, gorilllaz, yeeeeeep
    hgext.keyword.kwtemplater._orig_substitute = hgext.keyword.kwtemplater.substitute
    hgext.keyword.kwtemplater.substitute = patched_substitute
