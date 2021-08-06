#!/usr/bin/env python2
# encoding=utf-8
from __future__ import division, print_function

from glob import glob
from itertools import izip
from os import system
import csv
import numpy as np
import re

from my_helper_functions_bare import *

Ns = [256000, 500000, 1000188]
files_by_N = [ glob("csv/convergence-{}-0.*.csv".format(N)) for N in Ns ]
output_files = [ "csv/convergence-{}-pretty.csv".format(N) for N in Ns ]

chosen_parameter = "msds_diffusion"
skip = 10

for input_files, output_file_name in izip(files_by_N, output_files):
    collisions = []
    parameter = dict()

    for file_number, file_name in enumerate(sorted(input_files)):
        data = np.genfromtxt(file_name, delimiter='\t', names=[
            "packings","densities","collisions","n_atoms","pressures_virial",
            "pressures_collision","msds_val","msds_diffusion","times",
            "std_pressures_virial","std_pressures_collision","std_msds_val",
            "std_msds_diffusion","std_times"])
        n_atoms = data["n_atoms"][0]
        density = data["densities"][0]
        equilibrated_collisions = data["collisions"] \
                - 2*data["collisions"][0] + data["collisions"][1]

        if len(equilibrated_collisions) > len(collisions):
            collisions = list(equilibrated_collisions)

        parameter[density] = [ uncertain_number_string(n, u)
                for n, u in zip(data[chosen_parameter],
                    data["std_"+chosen_parameter]) ]

    with open(output_file_name, "w+") as output_file:
        csv_writer = csv.writer(output_file, delimiter='\t')
        csv_writer.writerow( ["\\multicolumn{1}{c}{$C$}"]
            + [ "\\multicolumn{1}{c}{"
            + "$D$, $d^*={:.1f}$".format(k).replace(".",",") + "}"
            for k in sorted(parameter) ])
        csv_writer.writerows(zip( [int(c) for c in collisions], *[ i[1]
            for i in sorted(parameter.items(), key=lambda x: x[0])
            ])[skip-1::skip])

    system("csv2latex -ntzyx -s t -c 0.75 {} > {}".format(output_file_name,
        re.sub("\.csv", ".tex", output_file_name)))
