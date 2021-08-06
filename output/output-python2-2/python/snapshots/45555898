from django.db import models
import datetime

class DownloadItem(models.Model):
    
    class Admin:
        pass

    class Meta:
        get_latest_by = 'added_at'
    
    url = models.CharField(maxlength=255, null=False)
    added_at = models.DateTimeField(auto_now_add=True)
    started_at = models.DateTimeField(null=True, blank=True)
    finished_at = models.DateTimeField(null=True, blank=True)

    def __str__(self):
        return self.url
    
    def is_started(self):
        return self.started_at is not None and datetime.datetime.now() > self.started_at
    
    def is_finished(self):
        return self.finished_at is not None and datetime.datetime.now() > self.finished_at
    
