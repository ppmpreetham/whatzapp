float lum(vec2 uv) {
    return dot(tap(0, uv).rgb, vec3(.3, .55, .15));
}

float sobel(vec2 c) {
    vec2 st = vec2(U[0], U[1]);
    float tl = lum(c + vec2(-1., 1.) * st);
    float tt = lum(c + vec2(0., 1.) * st);
    float tr = lum(c + vec2(1., 1.) * st);
    float l = lum(c + vec2(-1., 0.) * st);
    float r = lum(c + vec2(1., 0.) * st);
    float bl = lum(c + vec2(-1., -1.) * st);
    float b = lum(c + vec2(0., -1.) * st);
    float br = lum(c + vec2(1., -1.) * st);
    float gx = tr + 2. * r + br - tl - 2. * l - bl;
    float gy = tl + 2. * tt + tr - bl - 2. * b - br;
    return length(vec2(gx, gy));
}

vec4 effect(vec2 p) {
    vec2 uv = p / box_size;
    vec2 out_step = vec2(U[2], U[3]);
    float e = 0.;
    for (int i = 0; i < 4; i++) {
        e = max(e, sobel(uv + (vec2(float(i & 1), float(i >> 1)) - .5) * out_step) * 1.5);
    }
    return vec4(clamp(e, 0., 1.), 0., 0., 1.);
}
