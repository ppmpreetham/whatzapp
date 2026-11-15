vec4 effect(vec2 p) {
    vec2 vUv = p / box_size;
    vec2 px = vec2(U[0], U[1]);
    vec3 c = vec3(0.);
    c += tap(0, vUv + vec2(-2., 0.) * px).rgb + tap(0, vUv + vec2(2., 0.) * px).rgb;
    c += tap(0, vUv + vec2(0., -2.) * px).rgb + tap(0, vUv + vec2(0., 2.) * px).rgb;
    c += (tap(0, vUv + vec2(-1., 1.) * px).rgb + tap(0, vUv + vec2(1., 1.) * px).rgb) * 2.;
    c += (tap(0, vUv + vec2(-1., -1.) * px).rgb + tap(0, vUv + vec2(1., -1.) * px).rgb) * 2.;
    c /= 12.;
    float l = dot(c, vec3(.3, .55, .15));
    c = mix(c, vec3(.1, .26, 1.) * l * 1.7, U[2]);
    return vec4(c * .92 + tap(1, vUv).rgb * .8, 1.);
}
