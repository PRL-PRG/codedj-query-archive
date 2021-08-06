#import the necessary modules
import cv2
import numpy as np
from goToPosition import GoToPos
from transformation import getTransformationMat
import time
from moveObj import moveObj
from getGripperCenter import getGripperCenter


capture = cv2.VideoCapture()

points = [[-10,25,8.7], [0,21.22,12.86], [0,36,8.7], [10,21,15],  [-15,31,20],
          [10,31,20],   [10,29,15],      [15,29,15], [-15,20,15], [-10,20,15],
          [3,18,5],     [5,20,25],       [-5,25,25], [-5,30,6],   [15,20,6]]

DELAY = 5 

kinect_frame_pts = []
cntFrames = 0
indx = 0
TRANS_MAT = np.matrix([])

def calcTransMatrix():
    global TRANS_MAT, kinect_frame_pts
    Kinect_frame_matrix = np.matrix([[kinect_frame_pts[0][0],kinect_frame_pts[0][1],kinect_frame_pts[0][2],1],
                                    [kinect_frame_pts[1][0],kinect_frame_pts[1][1],kinect_frame_pts[1][2],1],
                                    [kinect_frame_pts[2][0],kinect_frame_pts[2][1],kinect_frame_pts[2][2],1],
                                    [kinect_frame_pts[3][0],kinect_frame_pts[3][1],kinect_frame_pts[3][2],1],
                                    [kinect_frame_pts[4][0],kinect_frame_pts[4][1],kinect_frame_pts[4][2],1],
                                    [kinect_frame_pts[5][0],kinect_frame_pts[5][1],kinect_frame_pts[5][2],1],
                                    [kinect_frame_pts[6][0],kinect_frame_pts[6][1],kinect_frame_pts[6][2],1],
                                    [kinect_frame_pts[7][0],kinect_frame_pts[7][1],kinect_frame_pts[7][2],1],
                                    [kinect_frame_pts[8][0],kinect_frame_pts[8][1],kinect_frame_pts[8][2],1],
                                    [kinect_frame_pts[9][0],kinect_frame_pts[9][1],kinect_frame_pts[9][2],1],
                                    [kinect_frame_pts[10][0],kinect_frame_pts[10][1],kinect_frame_pts[10][2],1],
                                    [kinect_frame_pts[11][0],kinect_frame_pts[11][1],kinect_frame_pts[11][2],1],
                                    [kinect_frame_pts[12][0],kinect_frame_pts[12][1],kinect_frame_pts[12][2],1],
                                    [kinect_frame_pts[13][0],kinect_frame_pts[13][1],kinect_frame_pts[13][2],1],
                                    [kinect_frame_pts[14][0],kinect_frame_pts[14][1],kinect_frame_pts[14][2],1],])

    #print Kinect_frame_matrix.transpose()
    TRANS_MAT = getTransformationMat(Kinect_frame_matrix.transpose())
    print TRANS_MAT

def mouseClick(event, y, x, flags, param):
    if event == cv2.EVENT_LBUTTONDOWN:
        capture.grab()
        ok, real = capture.retrieve(0, cv2.CAP_OPENNI_POINT_CLOUD_MAP)
        xw = 100 * real[x][y][0]
        yw = 100 * real[x][y][1]
        zw = 100 * real[x][y][2]
        point = TRANS_MAT * np.matrix([[xw],[yw],[zw],[1]])
        print point[0], point[1], point[2]
	moveObj(point[0],point[1],point[2],0,21.22,12.86)
        #GoToPos(point[0],point[1],point[2],'open')

if __name__ == "__main__":
    capture.open(cv2.CAP_OPENNI)
    x, y = 0, 0
    while 1:
        capture.grab()
        
        ok, rgb = capture.retrieve(0, cv2.CAP_OPENNI_BGR_IMAGE)
        if (indx > 1):
            cv2.circle(rgb, (y, x), 10, (0, 255, 0), 2)
        cv2.imshow('RGB image', rgb)

        #ok, depth = capture.retrieve(0, cv2.CAP_OPENNI_DISPARITY_MAP)
        #cv2.imshow('Depth map', depth)

        if (indx < 16 and cntFrames % DELAY == 0):
            ok, real = capture.retrieve(0, cv2.CAP_OPENNI_POINT_CLOUD_MAP)
            y, x = getGripperCenter(rgb)
            xw = 100 * real[x][y][0]
            yw = 100 * real[x][y][1]
            zw = 100 * real[x][y][2]
            if (indx > 0):
                print "Point ", indx, ": ", xw, yw, zw
                kinect_frame_pts.append([xw, yw, zw])
            if (indx < 15):
                GoToPos(points[indx][0], points[indx][1], points[indx][2], 'close')
            indx += 1

        if (indx == 16):
           print "Calculating transformation matrix......"
           calcTransMatrix()
           cv2.setMouseCallback('RGB image', mouseClick)
           indx += 1

        if (indx < 16):
            cntFrames += 1
        # quit program when 'esc' key is pressed
        k = cv2.waitKey(5) & 0xFF
        if k == 27:
            break;
