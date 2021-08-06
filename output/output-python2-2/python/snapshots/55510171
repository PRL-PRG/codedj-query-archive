# Copyright (C) 2000-2005 by Hewlett Packard Development Company, LP
#
# Author: Yasushi Saito (yasushi.saito@hp.com)
#
# Jockey is free software; you can redistribute it and/or modify it
# under the terms of the GNU General Public License as published by the
# Free Software Foundation; either version 2, or (at your option) any
# later version.
#
# Jockey is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
# for more details.
#
from pychart import *
theme.get_options()

#data = [(10, 20, 30, 5), (20, 65, 33, 5),
#        (30, 55, 30, 5), (40, 45, 51, 7), (50, 25, 27, 3)]


###DATA###

chart_object.set_defaults(area.T, size = (640, 480), y_range = (None, None),
                          x_coord = category_coord.T(data, 0))

chart_object.set_defaults(bar_plot.T, data = data)

# Draw the 1st graph. The Y upper bound is calculated automatically.
ar = area.T(x_axis=axis.X(label="Cuenta", format="/a-30{}%d"),
            y_axis=axis.Y(label="Cantidad", tic_interval=10))

#plot1=bar_plot.T(label="foo", cluster=(0, 3))
#plot2=bar_plot.T(label="bar", hcol=2, cluster=(1, 3))
#plot3=bar_plot.T(label="baz", hcol=3, cluster=(2, 3))
#plot4=bar_plot.T(label="mecachis", hcol=1, cluster=(3,3))
#ar.add_plot(plot1, plot2, plot3, plot4)

###PLOT###

ar.draw()