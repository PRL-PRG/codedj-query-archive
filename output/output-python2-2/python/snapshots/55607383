# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8
# =============================================================================
# $Id: config.py 3 2008-01-13 15:16:11Z s0undt3ch $
# =============================================================================
#             $URL: http://devnull.ufsoft.org/svn/TracAdsPanel/trunk/adspanel/config.py $
# $LastChangedDate: 2008-01-13 15:16:11 +0000 (Sun, 13 Jan 2008) $
#             $Rev: 3 $
#   $LastChangedBy: s0undt3ch $
# =============================================================================
# Copyright (C) 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# =============================================================================

from trac.core import *
from trac.config import Option, BoolOption

class AdsPanelOptions(Component):
    hide_for_authenticated = BoolOption(
        'adspanel', 'hide_for_authenticated', True,
        """Should the ads be hidden for authenticated users."""
    )
    ads_code = Option(
        'adspanel', 'ads_code', None,
        """The HTML code which displays the ads.

        NOTE: You are responsible for the HTML code you add."""
    )
    store_in_session = BoolOption(
        'adspanel', 'store_in_session', True,
        """Should the hidden/shown status be stored in session. If True,
        a user returning won't ever see the ads again until session is
        invalidated or user visits
        `http://domain.tld/<script_path>/adspanel/show`"""
    )
