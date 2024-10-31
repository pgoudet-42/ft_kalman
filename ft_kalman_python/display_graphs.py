import matplotlib.pyplot as plt
from structs import Axis, Motion, EulerAngles

def displayGraph2Dxy(predict: Motion, debug: Motion):
    predict_xs = [e.X for e in predict.position]
    predict_ys = [e.Y for e in predict.position]
    debug_xs = [e.X for e in debug.position]
    debug_ys = [e.Y for e in debug.position]
    ax = plt.figure().add_subplot()
    ax.plot(predict_xs, predict_ys, color='b', label="Prediction", alpha=0.5, linewidth=2.5, zorder=2)
    ax.scatter(debug_xs, debug_ys, color='g', label="Truth", alpha=0.5, zorder=1)
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.legend()
    plt.savefig('graphs/graph2dxy.png')

def displayGraph2Dyz(predict: Motion, debug: Motion):
    predict_zs = [e.Z for e in predict.position]
    predict_ys = [e.Y for e in predict.position]
    debug_zs = [e.Z for e in debug.position]
    debug_ys = [e.Y for e in debug.position]
    ax = plt.figure().add_subplot()
    ax.plot(predict_zs, predict_ys, color='b', label="Prediction", alpha=0.5, linewidth=2.5, zorder=2)
    ax.scatter(debug_zs, debug_ys, color='g', label="Truth", alpha=0.5, zorder=1)
    ax.set_xlabel('Z')
    ax.set_ylabel('Y')
    ax.legend()
    plt.savefig('graphs/graph2dyz.png')
    
def displayGraph2Dxz(predict: Motion, debug: Motion):
    predict_zs = [e.Z for e in predict.position]
    predict_xs = [e.X for e in predict.position]
    debug_zs = [e.Z for e in debug.position]
    debug_xs = [e.X for e in debug.position]
    ax = plt.figure().add_subplot()
    ax.plot(predict_zs, predict_xs, color='b', label="Prediction", alpha=0.5, linewidth=2.5, zorder=2)
    ax.scatter(debug_zs, debug_xs, color='g', label="Truth", alpha=0.5, zorder=1)
    ax.set_xlabel('X')
    ax.set_ylabel('Z')
    ax.legend()
    plt.savefig('graphs/graph2dxz.png')

def displayGraphSpeed(predict: Motion, debug: Motion):
    predict_speed = [e for e in predict.speed]
    debug_speed = [e for e in debug.speed]
    ax = plt.figure().add_subplot()
    ax.plot(predict_speed, color='b', label="Prediction", alpha=0.5, linewidth=2.5, zorder=2)
    ax.plot(debug_speed, color='g', label="Truth", alpha=0.5, zorder=1)
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.legend()
    plt.savefig('graphs/graphspeed.png')

def displayGraph3D(predict: Motion, debug: Motion):
    predict_xs = [e.X for e in predict.position]
    predict_ys = [e.Y for e in predict.position]
    predict_zs = [e.Z for e in predict.position]
    debug_xs = [e.X for e in debug.position]
    debug_ys = [e.Y for e in debug.position]
    debug_zs = [e.Z for e in debug.position]
    print(f"debug positions number:{len(debug.position)}, predict positions number:{len(predict.position)}")
    ax = plt.figure().add_subplot(projection='3d')
    ax.plot(predict_xs, predict_ys, predict_zs, color='b', alpha=1 , label="Prediction")
    ax.scatter(debug_xs, debug_ys, debug_zs, color='g', alpha=0.5, label="Truth")
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')
    ax.legend()
    plt.savefig('graphs/graph3d.png')

def displayVariances(variances):
    ax = plt.figure().add_subplot()
    ax.plot(variances[:, 0])
    ax.plot(variances[:, 1])
    ax.plot(variances[:, 2])
    ax.plot(variances[:, 3])
    ax.plot(variances[:, 4])
    ax.set_xlabel('X')
    ax.set_ylabel('variance')
    plt.savefig('graphs/variances.png')