precision mediump float;

attribute vec2 pos;
attribute vec4 color;
varying vec2 vPos;
varying vec4 vColor;

void main(void) {
    vPos = pos;
    vColor = color;
    gl_Position = vec4(pos, 0.0, 1.0);
}
