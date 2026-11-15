#define DT U[0]
#define TIME U[1]
#define ASPECT U[2]
#define BUOY U[3]
#define CHARGE U[4]
#define CURSOR vec2(U[5], U[6])
#define WIND vec2(U[7], U[8])
#define PTR vec4(U[9], U[10], U[11], U[12])

vec4 effect(vec2 p) {
    vec2 vUv = p / box_size;
    vec2 A = vec2(ASPECT, 1.);
    vec2 q = vUv * A;
    float t = TIME;
    float n1 = vnoise(q * 8. + vec2(0., -t * 2.4));
    float n2 = vnoise(q * 8. + vec2(7., -t * 2.4));
    float n3 = vnoise(q * 24. + vec2(3., -t * 5.));
    float n4 = vnoise(q * 24. + vec2(11., -t * 5.));
    vec2 d = (vUv - CURSOR) * A;
    vec2 wind = WIND * exp(-dot(d, d) / .03);

    vec2 vf = vec2(((n1 - .5) * .3 + (n3 - .5) * .22) / ASPECT, BUOY * (.17 + .13 * n2 + .08 * n4)) + wind;
    vec2 vs = vec2((vnoise(q * 5. + vec2(0., -t * .7)) - .5) * .34 / ASPECT, BUOY * (.07 + .05 * n1)) + wind * .6;
    float f = tap(1, vUv - vf * DT).r;
    float s = tap(1, vUv - vs * DT).g;

    f *= exp(-DT * (1.3 + 3.6 * vnoise(q * 20. + vec2(0., -t * 3.))));
    s *= exp(-DT * .5);

    vec4 st = tap(0, vUv);
    float band = smoothstep(0., .02, st.r) * (1. - smoothstep(.22, .58, st.r));
    float tongue = smoothstep(.34, .72, vnoise(q * 13. + vec2(0., -t * 1.1)));
    float src = band * (.2 + .9 * tongue) * (.65 + .5 * n3);

    float dp = seg(q, PTR.zw * A, PTR.xy * A);
    float pill = exp(-dp * dp / .0007);
    src = max(src, pill * CHARGE * CHARGE * CHARGE * (.3 + .6 * n3));
    f = max(f, src);
    s = min(1., s + DT * (f * 1.1 + pill * CHARGE * .9));
    return vec4(f, s, 0., 1.);
}
