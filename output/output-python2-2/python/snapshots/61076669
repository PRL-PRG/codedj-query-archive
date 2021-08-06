"""
maintenancemode application settings.

If you not configure the settings below in your own project settings.py,
they assume default values:
    
    MAINTENANCE_MODE
        Boolean. Enable/disable maintenance mode.
        Default: False
     
     Some observations:
     
     * If user is logged in and staff member, the maintenance page is
       not displayed.
     
     * If user's ip is in INTERNAL_IPS, the maintenance page is
       not displayed.
"""

from django.conf import settings

MAINTENANCE_MODE = getattr(settings, 'MAINTENANCE_MODE', False)