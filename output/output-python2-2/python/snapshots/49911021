#!/usr/bin/env python

# For this script to work we must set the Django settings file
# as an environment setting before importing LayerMapping

# Alternatively you can place 
# export DJANGO_SETTINGS_MODULE=settings
# in your .bash_profile
#
# or paste this code into a $ manage.py shell

import os
os.environ['DJANGO_SETTINGS_MODULE'] = 'settings'
from psycopg2 import IntegrityError

from django.contrib.gis.utils import mapping, LayerMapping, add_postgis_srs
from world.models import WorldBorders

try:
    add_postgis_srs(900913)
except IntegrityError:
    print "The Google Spherical Mercator projection, or a projection with srid 900913, already exists, skipping insert"

WORLD_SHP = 'world/data/TM_WORLD_BORDERS-0.3.shp'

layer = LayerMapping(WorldBorders,
                      WORLD_SHP,
                      mapping(WORLD_SHP,geom_name='geometry',multi_geom=True),
                      transform=False,
                      encoding='iso-8859-1')
                    
layer.save(verbose=True,strict=True,progress=True)
