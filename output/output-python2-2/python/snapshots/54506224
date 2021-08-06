from distutils.core import setup
import py2exe

setup(
  name = 'casnet-gui',
  description = 'CASNET',
  version = '1.2',

  windows = [
              {
                'script': 'casnet-gui.py',
                'icon_resources': [(1, "pics/casnet.ico")],
              }
            ],

  options = {
              'py2exe': {
                'packages':'encodings',
                'includes': 'cairo, pango, pangocairo, atk, gobject',
              }
            },

  data_files=[]
)