from django.conf import settings
from django.template import Template, Context
from django.http import HttpResponse


def welcome(request):
    "Create an empty URLconf 404 error response."
    if 'django.contrib.gis' in settings.INSTALLED_APPS:
        t = Template(WELCOME, name='Geodjango Welcome')
        c = Context({
            'project_name': settings.SETTINGS_MODULE.split('.')[0]
        })
        return HttpResponse(t.render(c), mimetype='text/html')


WELCOME = """
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html lang="en"><head>
  <meta http-equiv="content-type" content="text/html; charset=utf-8">
  <meta name="robots" content="NONE,NOARCHIVE"><title>Welcome to Django</title>
  <style type="text/css">
    html * { padding:0; margin:0; }
    body * { padding:10px 20px; }
    body * * { padding:0; }
    body { font:small sans-serif; }
    body>div { border-bottom:1px solid #ddd; }
    h1 { font-weight:normal; }
    h2 { margin-bottom:.8em; }
    h2 span { font-size:80%; color:#666; font-weight:normal; }
    h3 { margin:1em 0 .5em 0; }
    h4 { margin:0 0 .5em 0; font-weight: normal; }
    table { border:1px solid #ccc; border-collapse: collapse; width:100%; background:white; }
    tbody td, tbody th { vertical-align:top; padding:2px 3px; }
    thead th { padding:1px 6px 1px 3px; background:#fefefe; text-align:left; font-weight:normal; font-size:11px; border:1px solid #ddd; }
    tbody th { width:12em; text-align:right; color:#666; padding-right:.5em; }
    ul { margin-left: 2em; margin-top: 1em; }
    #summary { background: #e0ebff; }
    #summary h2 { font-weight: normal; color: #666; }
    #explanation { background:#eee; }
    #instructions { background:#f6f6f6; }
    #summary table { border:none; background:transparent; }
  </style>
</head>

<body>
<div id="summary">
  <h1>Welcome!</h1>
  <h2>Congratulations on your first GeoDjango-powered project: <u>{{project_name}}</u></h2>
</div>

<div id="instructions">
  <p>There are two basic parts of <b>{{project_name}}</b> that have been spatially enabled</p>
  <ul>
    <li>The <a href="admin/"> Administration Application</a> which allows trusted users to manage, edit, and modify all your data via a customizable OpenLayers interface.</li>
    <li>The <a href="databrowse/">Databrowse</a> which allows all your data models to be publically browsed and will display your geometry fields using an OpenLayers map with custom format serialization.</li>
  </ul>
</div>

<div id="explanation">
  <p>
    After investigating the usefulness of these two 'free' applications, the next step will be to start writing your own custom apps with new views of your data.
  </p>
</div>
</body></html>
"""
