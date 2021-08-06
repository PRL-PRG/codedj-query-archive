from django.contrib.gis.db import models
class InterestingLocation(models.Model):
    """A spatial model for interesting locations """
    name = models.CharField(max_length=50, )
    description = models.TextField()
    interestingness = models.IntegerField()
    geometry = models.PointField(srid=4326) 
    objects = models.GeoManager() # so we can use spatial queryset methods

    def __unicode__(self): return self.name
