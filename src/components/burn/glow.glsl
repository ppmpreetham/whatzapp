#define RES vec2(U[0], U[1])
#define PILL_C vec2(U[2], U[3])
#define PILL_H vec2(U[4], U[5])
#define HOLD U[6]
#define TIME U[7]
#define FLASH U[8]

vec4 effect(vec2 p) {
    vec2 vUv = p / box_size;
    vec2 px = vUv * RES;
    vec4 s = tap(0, vUv);
    float f = tap(1, vUv).r;
    float e = emberMask(s.r) * (.7 + .6 * vnoise(px * .06 + vec2(0., TIME * 5.)));
    vec3 c = fire(clamp(e, 0., 1.)) * e * 2.4;
    c += fire(clamp(f * 1.15, 0., 1.)) * smoothstep(.08, .6, f) * 1.5;
    c += fire(.55) * tap(2, vUv).r * smoothstep(.62, .98, s.g) * step(s.r, .001) * 1.25;
    float inP = 1. - smoothstep(-1., 1., sdPill(px, PILL_C, PILL_H));
    c += fire(.5 + .3 * HOLD) * inP * HOLD * HOLD * 1.4 * (1. - smoothstep(0., .3, s.r));
    c += vec3(1., .62, .22) * FLASH * 1.5 * exp(-pow(sdPill(px, PILL_C, PILL_H) / 60., 2.));
    return vec4(c, 1.);
}
