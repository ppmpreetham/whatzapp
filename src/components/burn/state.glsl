
#define RES vec2(U[0], U[1])
#define CURSOR vec2(U[2], U[3])
#define PTR vec4(U[4], U[5], U[6], U[7])
#define DT U[8]
#define CLOCK U[9]
#define ASPECT U[10]
#define IGNITE U[11]
#define CHARGE U[12]
#define SPEED U[13]
#define REV U[14]
#define FAN U[15]

vec4 effect(vec2 p) {
    vec2 vUv = p / box_size;
    vec2 texel = 1. / box_size;
    vec4 s = tap(0, vUv);
    float pr = s.r;
    float h = s.g;
    float ig = s.b;

    float n = 0.;
    float hn = 0.;
    vec4 q;
    q = tap(0, vUv + vec2(1., 0.) * texel);   n = max(n, q.r);       hn = max(hn, q.g * .94);
    q = tap(0, vUv + vec2(-1., 0.) * texel);  n = max(n, q.r);       hn = max(hn, q.g * .94);
    q = tap(0, vUv + vec2(0., 1.) * texel);   n = max(n, q.r);       hn = max(hn, q.g * .94);
    q = tap(0, vUv + vec2(0., -1.) * texel);  n = max(n, q.r);       hn = max(hn, q.g * .94);
    q = tap(0, vUv + vec2(1., 1.) * texel);   n = max(n, q.r * .72); hn = max(hn, q.g * .91);
    q = tap(0, vUv + vec2(-1., 1.) * texel);  n = max(n, q.r * .72); hn = max(hn, q.g * .91);
    q = tap(0, vUv + vec2(1., -1.) * texel);  n = max(n, q.r * .72); hn = max(hn, q.g * .91);
    q = tap(0, vUv + vec2(-1., -1.) * texel); n = max(n, q.r * .72); hn = max(hn, q.g * .91);

    float np = pr;
    vec2 A = vec2(ASPECT, 1.);
    float dp = seg(vUv * A, PTR.zw * A, PTR.xy * A);
    if (REV > .5) {
        np = ig > 0. ? clamp(PT + (CLOCK - ig) * 2.2, 0., 1.) : max(0., pr - DT * 3.);
    } else {
        float edge = tap(1, vUv).r;
        float fuel = (.5 + fbm(vUv * vec2(11., 7.) + 7.)) * (1. + 1.5 * edge);
        fuel *= .75 + .5 * vnoise(vec2(vUv.x * 34., vUv.y * 300.));
        fuel *= .8 + .4 * vnoise(vUv * vec2(90., 60.) + CLOCK * 2.6);
        vec2 dc = (vUv - CURSOR) * A;
        fuel *= 1. + FAN * exp(-dot(dc, dc) / .012);

        float lit_now = IGNITE * step(dp, .03 + .008 * vnoise(vUv * 90.));

        vec2 cp = vUv * RES / 14.;
        float landed = step(.9975, hash(floor(cp) * .013 + floor(CLOCK * 3.) * vec2(.37, .91)));
        landed *= step(length(fract(cp) - .5), .22) * step(.42, h) * step(h, .88);

        float go = max(smoothstep(.025, .08, n), max(lit_now, landed));
        if (pr > 0.) go = 1.;
        if (pr < 1. && go > 0.) np = min(1., pr + DT * SPEED * go * fuel);
        if (lit_now > 0.) np = max(np, .15);
        if (pr < PT && np >= PT) ig = CLOCK;
    }

    float charge = CHARGE * exp(-dp * dp / (.0006 + .003 * CHARGE));
    float hs = max(h * (REV > .5 ? .978 : .992), hn * (REV > .5 ? .9 : .92 + .05 * vnoise(vUv * 55.)));
    hs = max(hs, charge * .9);
    if (np > .02) hs = 1.;
    return vec4(np, hs, ig, 1.);
}
