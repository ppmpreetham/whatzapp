#define PT 0.6

float hash(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}

float vnoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3. - 2. * f);
    return mix(mix(hash(i), hash(i + vec2(1., 0.)), f.x), mix(hash(i + vec2(0., 1.)), hash(i + vec2(1., 1.)), f.x), f.y);
}

float fbm(vec2 p) {
    float a = .5;
    float s = 0.;
    for (int i = 0; i < 4; i++) {
        s += a * vnoise(p);
        p = p * 2.07 + 13.1;
        a *= .5;
    }
    return s;
}

vec3 fire(float t) {
    vec3 c = mix(vec3(.42, .04, .01), vec3(1., .3, .03), smoothstep(0., .35, t));
    c = mix(c, vec3(1., .68, .14), smoothstep(.35, .68, t));
    return mix(c, vec3(1., .96, .78), smoothstep(.68, 1., t));
}

float emberMask(float p) {
    return smoothstep(0., .012, p) * (1. - smoothstep(.02, .1, p));
}

float sdPill(vec2 q, vec2 c, vec2 h) {
    vec2 d = abs(q - c) - h + h.y;
    return length(max(d, 0.)) + min(max(d.x, d.y), 0.) - h.y;
}

float seg(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    return length(pa - ba * clamp(dot(pa, ba) / max(dot(ba, ba), 1e-8), 0., 1.));
}

vec4 tap(int i, vec2 uv) {
    return input_raw(i, uv * input_size(i));
}
