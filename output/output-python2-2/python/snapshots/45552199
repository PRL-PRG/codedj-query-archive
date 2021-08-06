import sys

sys.path.append('UI/MainUI')
sys.path.append('UI/GridUI')
sys.path.append('Classes')
sys.path.append('arm_classes')

from PyQt4 import QtGui
from PyQt4.QtGui import QPixmap
from PyQt4 import QtCore
import MainUI
from Player import Player
from GridWindow import GridWindow
from PyQt4.QtCore import pyqtSignal
from PyQt4.QtCore import QThread
from PyQt4.QtCore import QTimer
import numpy as np
from moveObj import moveObj

class App(QtGui.QMainWindow, MainUI.Ui_BCARM):
    player = Player()
    grid = None

    def __init__(self):
        super(self.__class__, self).__init__()
        self.setupUi(self)
        self.grid = GridWindow()
	self.grid.labelSignaltoGUI.connect(self.receiveLabel)
	self.grid.patterDetectedSingaltoGUI.connect(self.startClassification)
        self.player.frameReady.connect(self.display)
        self.player.objectsReady.connect(self.sendObjects)
        self.player.start()
	self.grid.startDetection()

    def receiveLabel(self, label):
        self.player.setFreeze(False)
        self.player.start()
	self.grid.startDetection()
	(x, y, z) = self.player.getPosition(label)
	print(x, y, z)
	if (not(x == 0 and y == 0 and z == 0)):
		TRANS_MAT = np.matrix([[3.82940410e-02,   9.88007145e-02,  -9.86460643e-01,   1.11954824e+02],
 				       [1.19521619e+00,   5.20338376e-02,  -5.10819249e-02,   4.26214500e+01],
				       [8.46543450e-02,   1.25626058e+00,   6.72425551e-02,   6.49483004e+00],
				       [-1.11022302e-16,   0.00000000e+00,   0.00000000e+00,   1.00000000e+00]])

		pos = np.matrix([[x],[y],[z],[1]])
		point = TRANS_MAT * pos
		moveObj(point[0],point[1],point[2],0,21.22,12.86)

    def display(self, rgb):
        self.lbl_rgb.setPixmap(QPixmap.fromImage(rgb).scaled(self.lbl_rgb.size()))

    def startClassification(self):
        self.player.setFreeze(True)
        self.grid.startClassification()

    def sendObjects(self, objs):
        self.grid.setObjects(objs)

def main():
	app = QtGui.QApplication(sys.argv)
	form = App();
	form.showFullScreen();
	app.exec_();

if __name__ == '__main__':
    main()
