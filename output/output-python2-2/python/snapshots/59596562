#!/usr/bin/env python2
# encoding=utf-8
from __future__ import division, print_function

from glob import glob
from itertools import izip
from matplotlib import pyplot as plt
import numpy as np

#input_files = glob("csv/convergence-256000-0.*.csv")
#input_files = glob("csv/convergence-500000-0.*.csv")
input_files = glob("csv/convergence-1000188-0.*.csv")

plotted_parameter = "msds_diffusion"
#plotted_parameter = "pressures_collision"
#plotted_parameter = "pressures_virial"

#plotted_parameter = "msds_val"
#plotted_parameter = "times"


legend_names = []
tight_layout = False
show_legend = False

for file_number, file_name in enumerate(sorted(input_files)):
    data = np.genfromtxt(file_name, delimiter='\t', names=[
        "packings","densities","collisions","n_atoms","pressures_virial",
        "pressures_collision","msds_val","msds_diffusion","times",
        "std_pressures_virial","std_pressures_collision","std_msds_val",
        "std_msds_diffusion","std_times"])
    n_atoms = data["n_atoms"][0]
    density = data["densities"][0]
    equilibrated_collisions = data["collisions"] - 2*data["collisions"][0] \
            + data["collisions"][1]

    ###   5 graphs: D(CPS)   ###
    tight_layout = True
    skip_points = 0
    ax = plt.subplot(3, 2, file_number+1)
    plt.fill_between((equilibrated_collisions / n_atoms)[skip_points:],
            data[plotted_parameter][skip_points:]
            - data["std_" + plotted_parameter][skip_points:],
            data[plotted_parameter][skip_points:]
            + data["std_" + plotted_parameter][skip_points:], alpha=0.3)
    plt.plot((equilibrated_collisions / n_atoms)[skip_points:],
            data[plotted_parameter][skip_points:], lw=2)
    if plotted_parameter == "msds_diffusion":
        plt.ylim(0.990*data[plotted_parameter][-1],
                1.005*data[plotted_parameter][-1])
    plt.legend(["Density {}".format(data["densities"][0])], loc="lower right")
    ax.yaxis.set_major_formatter(plt.FormatStrFormatter('%.4f'))
    plt.xlabel("Collisions per sphere")
    plt.ylabel("D")
    """
    ###   5 graphs: D(1/CPS)   ###
    tight_layout = True
    skip_points = 40
    ax = plt.subplot(3, 2, file_number+1)
    plt.fill_between((n_atoms / equilibrated_collisions)[skip_points:],
            data[plotted_parameter][skip_points:]
            - data["std_" + plotted_parameter][skip_points:],
            data[plotted_parameter][skip_points:]
            + data["std_" + plotted_parameter][skip_points:], alpha=0.3)
    plt.plot((n_atoms / equilibrated_collisions)[skip_points:],
            data[plotted_parameter][skip_points:], lw=2)
    plt.title("Density {}:".format(data["densities"][0]))
    ax.yaxis.set_major_formatter(plt.FormatStrFormatter('%.7f'))
    plt.xlim(xmin=0)
    plt.xlabel("1 / Collisions per sphere")
    plt.ylabel("D")
    """
    """
    ###   1 graph: D(CPS) / Dinf   ###
    show_legend = True
    plt.fill_between(equilibrated_collisions / n_atoms,
            (data[plotted_parameter] - data["std_" + plotted_parameter])
            / data[plotted_parameter][-1] - 1,
            (data[plotted_parameter] + data["std_" + plotted_parameter])
            / data[plotted_parameter][-1] - 1, color="grey", alpha=0.4)
    plt.plot(equilibrated_collisions / n_atoms,
            data[plotted_parameter] / data[plotted_parameter][-1] - 1, lw=2)
    legend_names.append(data["densities"][0])
    plt.xlabel("Collisions per sphere")
    plt.ylabel("D / D(t --> inf)")
    """
    """
    ###   1 graph: D(1/CPS) / Dinf   ###
    show_legend = True
    plt.fill_between(n_atoms / equilibrated_collisions,
            (data[plotted_parameter] - data["std_" + plotted_parameter])
            / data[plotted_parameter][-1] - 1,
            (data[plotted_parameter] + data["std_" + plotted_parameter])
            / data[plotted_parameter][-1] - 1, color="grey", alpha=0.4)
    plt.plot( n_atoms / equilibrated_collisions,
            data[plotted_parameter] / data[plotted_parameter][-1] - 1)
    legend_names.append(data["densities"][0])
    plt.xlabel(" 1 / Collisions per sphere")
    plt.ylabel(plotted_parameter)
    """

#if tight_layout:
#    plt.tight_layout(pad=0.0, w_pad=0.0, h_pad=0.0)
if show_legend:
    plt.legend(legend_names, title="Density:", loc="lower right")

plt.show()
