uniform sampler2D texture;

void main() {
    vec4 pixel = texture2D(texture, gl_TexCoord[0].st);

    if (pixel.a == 0.0) {
       discard;
    }

    // gl_FragColor = gl_Color;
    gl_FragColor = pixel;
}