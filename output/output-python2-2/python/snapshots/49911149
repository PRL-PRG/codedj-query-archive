from django.contrib import admin
from django.contrib.gis.admin import OSMGeoAdmin
from django.contrib import databrowse
from world.models import WorldBorders

class WorldBordersAdmin(OSMGeoAdmin):
    """
    Note: another common practice is to put the AdminOptions in a file called admin.py
    """
    list_display = ('name','pop2005','region','subregion',)
    search_fields = ('name',)
    ordering = ('name',)
    list_filter = ('region','subregion',)
    save_as = True
    search_fields = ['name','iso2','iso3','subregion','region']
    list_select_related = True
    fieldsets = (
      ('Country Attributes', {'fields': (('name','pop2005')), 'classes': ('show','extrapretty')}),
      ('Country Codes', {'fields': ('region','subregion','iso2','iso3','un',), 'classes': ('collapse',)}),
      ('Area and Coordinates', {'fields': ('area','lat','lon',), 'classes': ('collapse', 'wide')}),
      ('Editable Map View', {'fields': ('geometry',), 'classes': ('show', 'wide')}),
    )

    # Default OL map options: uncomment and modify as desired
    #default_lon = 0
    #default_lat = 0
    #default_zoom = 4
    #display_wkt = False
    #display_srid = 4326
    #display_projection = True
    #max_zoom = False
    #min_zoom = False
    #units = 'm'
    #max_resolution = False
    #max_extent = False
    #modifiable = True
    #hoverable = False
    #mouse_position = True
    #scale_text = True
    #layerswitcher = True
    scrollable = False
    map_width = 700
    map_height = 325
    #debug = False
    #wms_layer = 'basic'
    #openlayers_url = 'http://openlayers.org/api/2.6/OpenLayers.js'
    #wms_url = 'http://labs.metacarta.com/wms/vmap0'
    #wms_name = 'OpenLayers WMS'
    #map_template = 'gis/admin/osm.html'
    #extra_js = ['http://openstreetmap.org/openlayers/OpenStreetMap.js']
    #num_zoom = 20
    #map_srid = 900913

databrowse.site.register(WorldBorders)
admin.site.register(WorldBorders,WorldBordersAdmin)