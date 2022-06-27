attribute vec3 position;
uniform mat4 mvpMatrix;
uniform float pointSize;

void main(void) {
    gl_PointSize = pointSize;
    gl_Position = mvpMatrix * vec4(position, 1.0);
}
