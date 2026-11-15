#define RES vec2(U[0], U[1])
#define PILL_C vec2(U[2], U[3])
#define PILL_H vec2(U[4], U[5])
#define TIME U[6]
#define HOLD U[7]
#define CUT U[8]

vec2 hash2(vec2 p) { return vec2(hash(p), hash(p + 19.19)); }

vec2 plates(vec2 x) {
    vec2 n = floor(x);
    vec2 f = fract(x);
    float f1 = 8.;
    float f2 = 8.;
    float id = 0.;
    for (int j = -1; j <= 1; j++) {
        for (int i = -1; i <= 1; i++) {
            vec2 g = vec2(float(i), float(j));
            vec2 r = g + hash2(n + g) - f;
            float d = dot(r, r);
            if (d < f1) { f2 = f1; f1 = d; id = hash(n + g); }
            else if (d < f2) { f2 = d; }
        }
    }
    return vec2(sqrt(f2) - sqrt(f1), id);
}

vec4 effect(vec2 p) {
    vec2 uv = p / box_size;
    vec2 px = uv * RES;
    vec2 px1 = 1. / RES;
    float t = TIME;
    float rot = hash(px) * 6.2831;

    vec2 jit = (vec2(vnoise(px * .11), vnoise(px * .11 + 17.)) - .5) * 3.6 * px1;
    vec4 s = tap(0, uv + jit);
    float h = s.g;
    float pr = s.r > .001 ? max(s.r + (fbm(px * .05) - .5) * .14 + (hash(floor(px * .5)) - .5) * .05, .001) : s.r;

    vec2 fluid = tap(1, uv).rg;
    vec3 bloom = tap(2, uv).rgb;
    float glow = dot(bloom, vec3(.3, .55, .15));
    float fl = vnoise(px * .06 + vec2(0., t * 5.));

    float hz = smoothstep(.2, .6, h) * (1. - smoothstep(.85, .99, h)) + clamp(glow * .5, 0., 1.);
    vec2 wob = (vec2(vnoise(px * .02 + vec2(0., t * 1.8)), vnoise(px * .02 + 9. + vec2(t * 1.3, 0.))) - .5) * hz * 8. * px1;
    vec2 e = vec2(6.) * px1;
    vec2 gh = vec2(tap(0, uv + vec2(e.x, 0.)).g - tap(0, uv - vec2(e.x, 0.)).g, tap(0, uv + vec2(0., e.y)).g - tap(0, uv - vec2(0., e.y)).g);
    float give = smoothstep(.15, .9, h) * (1. - smoothstep(.92, 1., h));
    vec2 tuv = uv + wob + gh * 16. * give * px1;
    vec2 q = tuv * RES;

    vec3 sheet = tap(4, tuv).rgb;

    float melt = smoothstep(.3, .92, h);
    if (melt > .01 && s.r <= .001) {
        vec3 soft = vec3(0.);
        for (int i = 0; i < 7; i++) {
            float fi = float(i);
            float a = fi * 2.39996 + rot;
            soft += tap(4, tuv + vec2(cos(a), sin(a)) * sqrt((fi + .5) / 7.) * melt * 4.2 * px1).rgb;
        }
        soft /= 7.;
        sheet = mix(sheet, min(sheet, soft * soft * 1.04), melt);
        float dry = smoothstep(.45, .8, vnoise(q * .6)) * step(.5, hash(floor(q * 1.3)));
        sheet = mix(sheet, max(sheet, vec3(.78, .76, .72)), dry * melt * .55);
    }

    float mat = smoothstep(.04, .55, h);
    if (mat > .001) {
        float cockle = fbm(q * .012 + 1.7);
        float thread = .5 * vnoise(vec2(q.x * .85, q.y * .07)) + .5 * vnoise(vec2(q.x * .07, q.y * .85));
        float fib = fbm(vec2(q.x * .02, q.y * .6)) * .5 + fbm(vec2(q.x * .6, q.y * .02)) * .5;
        float v = (cockle - .5) * .16 + (thread - .5) * .09 + (fib - .42) * .08 + (hash(floor(q)) - .5) * .05;
        v -= step(.9985, hash(floor(q * .8) + 3.1)) * (.1 + .3 * hash(floor(q * .8)));
        sheet *= 1. + v * mat;
        sheet *= 1. + dot(gh, vec2(-.7, .7)) * 2.4 * give + (cockle - .5) * .22 * give;
    }

    float mott = fbm(px * .03 + 5.);
    float sc = pow(clamp(smoothstep(.06, .95, h) * (.8 + .5 * mott) * 1.25, 0., 1.), 1.25);
    vec3 toned = mix(sheet, sheet * vec3(.97, .8, .54), smoothstep(0., .42, sc));
    toned = mix(toned, sheet * vec3(.62, .36, .16), smoothstep(.34, .72, sc));
    toned = mix(toned, vec3(.13, .065, .03), smoothstep(.68, 1., sc));

    float ed = tap(3, uv).r;
    toned = mix(toned, fire(.42 + .4 * fl), ed * smoothstep(.62, .97, h) * (1. - smoothstep(0., .01, pr)) * .85);

    float inP = (1. - smoothstep(-.6, .6, sdPill(q, PILL_C, PILL_H))) * HOLD;
    if (inP > 0.) {
        float tm = smoothstep(.5, .85, dot(tap(4, tuv).rgb, vec3(.33)));
        vec3 hot = fire(mix(.8, .18, clamp((PILL_C.x + PILL_H.x - q.x) / 110., 0., 1.)) + .08 * fl);
        toned = mix(toned, mix(hot, vec3(.04), tm), inP);
    }

    float cp = clamp(pr / PT, 0., 1.);
    vec3 top = toned;
    if (pr > 0.) {
        vec2 big = plates(px * .036);
        vec2 fine = plates(px * .115 + 4.);
        float crackA = 1. - smoothstep(0., .08, big.x);
        float crackB = 1. - smoothstep(0., .1, fine.x);
        vec3 charc = vec3(.02 + .05 * big.y) * vec3(1., .96, .93) * (.65 + .7 * vnoise(px * .35));
        charc *= 1. - .6 * crackA;
        float alive = 1. - smoothstep(.04, .8, cp + (big.y - .5) * .2);
        float pulse = .6 + .5 * vnoise(vec2(big.y * 40., t * 3.));
        charc += fire(.3 + .35 * fl + .25 * big.y) * (crackA + crackB * .4) * alive * pulse * 1.4;
        charc = mix(vec3(.16, .07, .025), charc, smoothstep(0., .12, cp));
        top = mix(toned, charc, smoothstep(0., .03, pr));
    }

    float root = smoothstep(0., .005, pr) * (1. - smoothstep(.005, .018, pr));
    top = mix(top, vec3(.2, .42, 1.), root * .75);
    float ember = emberMask(pr);
    top = mix(top, fire(clamp(ember * (.78 + .5 * fl), 0., 1.)), ember * .96);

    float rim = smoothstep(PT - .16, PT - .06, pr) * (1. - smoothstep(PT - .06, PT, pr));
    top += (vec3(.2, .2, .22) + fire(.45) * .5 * clamp(glow, 0., 1.)) * rim;
    float lace = step(.7, vnoise(px * .5)) * smoothstep(PT - .26, PT - .1, s.r);
    float a = (1. - smoothstep(PT - .05, PT, pr)) * (1. - lace);

    float cover = 0.;
    for (int i = 0; i < 6; i++) {
        float fi = float(i);
        float an = fi * 2.39996 + rot;
        vec2 o = vec2(cos(an), sin(an)) * sqrt((fi + .5) / 6.) * 12. + vec2(4., -7.);
        cover += 1. - smoothstep(PT - .1, PT, tap(0, uv + o * px1).r);
    }
    cover /= 6.;

    vec3 rgb = top * a;
    float alpha = a + (1. - a) * cover * .6;

    float sa = smoothstep(.02, .6, fluid.g) * .34;
    vec3 sc1 = vec3(.09, .085, .09) + bloom * .35;
    rgb = rgb * (1. - sa) + sc1 * sa;
    alpha = alpha + (1. - alpha) * sa;

    float fa = smoothstep(.1, .5, fluid.r);
    vec3 fc = fire(clamp(fluid.r * 1.15, 0., 1.));
    rgb = rgb * (1. - fa) + fc * fa;
    alpha = alpha + (1. - alpha) * fa * .6;

    rgb *= 1. - clamp(glow * .55, 0., .75) * vec3(.55, .75, .85);
    rgb += bloom * mix(1.15, .9, a);

    if (CUT > .5) {
        float k = smoothstep(.003, .03, h + fluid.r + fluid.g + glow);
        rgb *= k;
        alpha *= k;
    }

    return vec4(rgb, alpha);
}
