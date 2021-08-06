#!/usr/bin/env python
# encoding: utf-8
# Licensed under BSD license
# Copyright (c) 2008, Yury Yurevich <the.pythy@gmail.com>
# All rights reserved.
# 
# Redistribution and use in source and binary forms, with or without modification, are 
# permitted provided that the following conditions are met:
# 
# * Redistributions of source code must retain the above copyright notice, 
#   this list of conditions and the following disclaimer.
# * Redistributions in binary form must reproduce the above copyright notice, 
#   this list of conditions and the following disclaimer in the documentation and/or
#   other materials provided with the distribution.
# * Neither the name of the author nor the names of its contributors may be used to
#   endorse or promote products derived from this software without specific 
#   prior written permission.
# 
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY 
# EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
# OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
# SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
# INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED 
# TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR 
# BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN 
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN 
# ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH 
# DAMAGE.
"""
Make changelog of mercurial collections, not single repo
"""
import os
import itertools
import datetime
from mercurial import ui, node, repo, localrepo


def iter_changes_from_single_repo(repository, changelog_length):
    """
    Return an iterator of changes for single repository
    
    Each item in iterator is a dict with keys 'author', 'comment',
    'rev', 'changeset', 'timestamp'
    """
    changelog = repository.changelog
    maxrev = changelog.count()
    for i in xrange(min(changelog_length, maxrev)):
        rev = maxrev - i - 1
        changeset = changelog.read(changelog.node(rev))
        manifest, author, (ts, tz), files, comment, extra = changeset
        # TODO: correctly process time zone
        dt = datetime.datetime.fromtimestamp(ts)
        yield {
            'author': author, 
            'comment': comment, 
            'rev': rev, 
            'changeset': node.hex(manifest), 
            'timestamp': dt,
        }

def get_changes(collection_path, changelog_length=10):
    """
    Return a list of changes for repositories collection
    """
    iface = ui.ui(quiet=True, interactive=False)    
    changes = []
    for name in os.listdir(collection_path):
        repo_dir = os.path.join(collection_path, name)
        if os.path.isdir(repo_dir):
            try:
                curr_repo = localrepo.localrepository(
                    iface, 
                    repo_dir)
            except repo.RepoError:
                # dir is not a hg repo
                continue
            # zip(itertools.cycle([name]), seq) -> 
            #    [(name, seq[0]), (name, seq[1]), ...]
            changes += zip(itertools.cycle([name]), 
                            iter_changes_from_single_repo(curr_repo, 
                                                         changelog_length))
    return sorted(
            changes, key=lambda x: x[1]['timestamp'], reverse=True
        )[:changelog_length]

if __name__ == '__main__':
    import sys, pprint
    if len(sys.argv) > 1:
        collection_path = sys.argv[1]
    else:
        collection_path = '/srv/mercurial/repos'
    changes = get_changes(collection_path)
    pp = pprint.PrettyPrinter()
    pp.pprint(changes)
