attribute vec3 position;
attribute vec3 normal;
uniform mat4 mvpMatrix;
varying vec3 vNormal;

void main(void) {
    //gl_PointSize = 10.0;
    vNormal = normal;
    gl_Position = mvpMatrix * vec4(position, 1.0);
}
