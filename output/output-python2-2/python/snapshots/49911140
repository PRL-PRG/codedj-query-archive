#!/usr/bin/env python

# To run this as a script you must set the Django settings file
# as an environment setting before importing LayerMapping
# Alternatively you can place 
# export DJANGO_SETTINGS_MODULE=settings
# in your .bash_profile
#
# or paste this code into a $ manage.py shell

import os
os.environ['DJANGO_SETTINGS_MODULE'] = 'settings'

from django.contrib.gis.utils import mapping, LayerMapping, add_postgis_srs
from world.models import WorldBorders

add_postgis_srs(900913)
world_borders = 'data/TM_WORLD_BORDERS_SIMPL-0.2/TM_WORLD_BORDERS_SIMPL-0.2.shp'
layer = LayerMapping(WorldBorders,world_borders,mapping(world_borders, geom_name='geometry',multi_geom=True), encoding='Latin1')
layer.save(verbose=True, progress=True)
