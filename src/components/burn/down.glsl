vec4 effect(vec2 p) {
    vec2 vUv = p / box_size;
    vec2 px = vec2(U[0], U[1]);
    vec3 c = tap(0, vUv).rgb * 4.;
    c += tap(0, vUv + vec2(-1., -1.) * px).rgb + tap(0, vUv + vec2(1., -1.) * px).rgb;
    c += tap(0, vUv + vec2(-1., 1.) * px).rgb + tap(0, vUv + vec2(1., 1.) * px).rgb;
    return vec4(c / 8., 1.);
}
