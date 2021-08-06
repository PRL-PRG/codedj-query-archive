import scipy.io as io
import pickle
import numpy as np
import timeit
from operator import eq
#from sklearn import linear_model

class EpochClassifier(object):
	SamplesData = 0
	Flashing = 0
	StimulusCode = 0
	model = 0
	window = 70
	nChannels = 14
	nflash = 13
	data = 0
	nRC = 6
	nIntensifications = 15
	screen=[	['A','B','C'],
			['D','E','F'],
			['G','H','I']	]

	def setData(self, SamplesData, Flashing, StimulusCode, model):
		self.model = model
		self.SamplesData = SamplesData
		self.Flashing = Flashing
		self.StimulusCode = StimulusCode
	
	def processData(self):
		rowColCnt = [0] * self.nRC
		self.data = np.zeros((self.nRC, self.nIntensifications, self.window, self.nChannels))
		ind = 1
		while (min(rowColCnt) < 15):
			lst = self.Flashing[ind - 1]
			cur = self.Flashing[ind]
			if (lst == 1 and cur == 0):
				rowcol = self.StimulusCode[ind - 1] - 1
				L = self.SamplesData[ind - self.nflash : ind + self.window - self.nflash]
				assert(len(L) == self.window)
				self.data[rowcol, rowColCnt[rowcol], :, :] = np.array(list(L))
				rowColCnt[rowcol] += 1
			ind += 1
		self.data = np.reshape(self.data, (self.nRC, self.nIntensifications, self.window * self.nChannels), order = "F")
		self.data = np.mean(self.data, axis=1)

	def classify(self):
		self.processData()
		score = self.model.decision_function(self.data)
		bestcol = np.argmax(score[0 : self.nRC / 2])
		bestrow = np.argmax(score[self.nRC / 2 : self.nRC])
		target = self.screen[bestcol][bestrow]
		return target
