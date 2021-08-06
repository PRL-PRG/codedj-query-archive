#import the necessary modules
import cv2
import numpy as np
from goToPosition import GoToPos
from transformation import getTransformationMat
import time
from moveObj import moveObj

capture = cv2.VideoCapture()
real = None

points = [[-10,25,8.7], [0,21.22,12.86], [0,36,8.7], [10,21,15],  [-15,31,20],
          [10,31,20],   [10,29,15],      [15,29,15], [-15,20,15], [-10,20,15],
          [3,18,5],     [5,20,25],       [-5,25,25], [-5,30,6],   [15,20,6]]

kinect_frame_pts = []
cnt = 1
done = False
TRANS_MAT = np.matrix([])

def go_to_position_mouse(event, y, x, flags, param):
    global TRANS_MAT, cnt, done
    if event == cv2.EVENT_LBUTTONDOWN:
        print y, x
        capture.grab()
        ok, real = capture.retrieve(0, cv2.CAP_OPENNI_POINT_CLOUD_MAP)
        xw = 100 * real[x][y][0]
        yw = 100 * real[x][y][1]
        zw = 100 * real[x][y][2]
        print xw, yw, zw
        if cnt < 16:
            if cnt < 15:
                GoToPos(points[cnt][0], points[cnt][1], points[cnt][2], 'close')
                #time.sleep(5)
            kinect_frame_pts.append([xw, yw, zw + 1.5])
            cnt += 1
        else:
            if not done: 
                done = True
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

                #print kinect_frame_pts
                #print Kinect_frame_matrix.transpose()
                TRANS_MAT = getTransformationMat(Kinect_frame_matrix.transpose())
                print TRANS_MAT
            else:
                point = TRANS_MAT * np.matrix([[xw],[yw],[zw],[1]])
                print point[0],point[1],point[2]
		moveObj(point[0],point[1],point[2],0,21.22,12.86)
                #GoToPos(point[0],point[1],point[2],'open')



cv2.namedWindow('RGB image')
cv2.setMouseCallback('RGB image', go_to_position_mouse)

if __name__ == "__main__":
    GoToPos(points[0][0], points[0][1], points[0][2], 'close')
    capture.open(cv2.CAP_OPENNI2)
    if (capture.isOpened() == False):
        capture.open(cv2.CAP_OPENNI)
    while 1:
        if not capture.grab():
            print "Unable to grab frame from kinect"
            break

        ok, rgb = capture.retrieve(0, cv2.CAP_OPENNI_BGR_IMAGE)
        if not ok:
            print "Unable to retrieve RGB image"
            break
        cv2.imshow('RGB image', rgb)

        ok, depth = capture.retrieve(0, cv2.CAP_OPENNI_DISPARITY_MAP)
        if not ok:
            print "Unable to retrieve depth map"
            break
        cv2.imshow('Depth map', depth)

        # quit program when 'esc' key is pressed
        k = cv2.waitKey(5) & 0xFF
        if k == 27:
            break;
    cv2.destroyAllWindows()

