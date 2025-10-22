#version 450
precision highp float;

// uniform float uCanvasWidth;
// uniform float uCanvasHeight;
uniform float uTime;
varying vec2 vUv;
uniform vec2 uResolution;


//
// Description : Array and textureless GLSL 2D simplex noise function.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : stegu
//     Lastmod : 20110822 (ijm)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
//               https://github.com/stegu/webgl-noise
// 


vec3 mod289(vec3 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec2 mod289(vec2 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec3 permute(vec3 x) {
  return mod289(((x*34.0)+10.0)*x);
}

float snoise(vec2 v)
  {
  const vec4 C = vec4(0.211324865405187,  // (3.0-sqrt(3.0))/6.0
                      0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)
                     -0.577350269189626,  // -1.0 + 2.0 * C.x
                      0.024390243902439); // 1.0 / 41.0
// First corner
  vec2 i  = floor(v + dot(v, C.yy) );
  vec2 x0 = v -   i + dot(i, C.xx);

// Other corners
  vec2 i1;
  //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0
  //i1.y = 1.0 - i1.x;
  i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
  // x0 = x0 - 0.0 + 0.0 * C.xx ;
  // x1 = x0 - i1 + 1.0 * C.xx ;
  // x2 = x0 - 1.0 + 2.0 * C.xx ;
  vec4 x12 = x0.xyxy + C.xxzz;
  x12.xy -= i1;

// Permutations
  i = mod289(i); // Avoid truncation effects in permutation
  vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
		+ i.x + vec3(0.0, i1.x, 1.0 ));

  vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);
  m = m*m ;
  m = m*m ;

// Gradients: 41 points uniformly over a line, mapped onto a diamond.           // What the magickal spell is this?
// The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)

  vec3 x = 2.0 * fract(p * C.www) - 1.0;
  vec3 h = abs(x) - 0.5;
  vec3 ox = floor(x + 0.5);
  vec3 a0 = x - ox;

// Normalise gradients implicitly by scaling m
// Approximation of: m *= inversesqrt( a0*a0 + h*h );
  m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );

// Compute final noise value at P
  vec3 g;
  g.x  = a0.x  * x0.x  + h.x  * x0.y;
  g.yz = a0.yz * x12.xz + h.yz * x12.yw;
  return 130.0 * dot(m, g);
}


float snoise_octaved(vec2 pos, int octaves, float lacunarity) {
    float value = 0.0;
    float amplitude = 1.0;
    float frequency = 1.0;
    float totalAmplitude = 0.0;
    
    for (int i = 0; i < octaves; i++) {
        float s = (snoise(pos * frequency) * 0.5) + 0.5;
        value += amplitude * s;
        totalAmplitude += amplitude;
        amplitude *= 0.5;
        frequency *= lacunarity;
        pos += vec2(321.0, -54.1);
    }
    
    return value / totalAmplitude;
}

float trunc(float x) {
    return float(int(x));
}

float fast_sin(float x){
   x = fract( 0.75 + x*0.159155 )*2.0 - 1.0; 
   return x*x * (6.0 - 4.0*x) - 1.0;
}

float fast_cos(float x) {
    return fast_sin(x + 1.5707963267948966);
}

vec3 fast_cos(vec3 x) {
    return vec3(
        fast_cos(x.x),
        fast_cos(x.y),
        fast_cos(x.z)
    );
}


// https://iquilezles.org/articles/palettes/
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d )
{
    return a + b * cos( 6.283185*(c*t+d) );

    // gives strange visual artifacts that looks cool tbh 😉😉😉
    // this is how i justify mo code quality )))
    // ok, its too broken
    // return a + b * fast_cos( 6.283185*(c*t+d) );
}


// vec2 rotate_uv_domain_slow(vec2 uv, float radians) {
//     float c = cos(radians);
//     float s = sin(radians);
//     mat2 rotationMatrix = mat2(c, -s, s, c);
//     return rotationMatrix * uv;
// }
// vec2 rotate_uv_domain_fast(vec2 uv, float radians) {
//     float c = fast_cos(radians);
//     float s = fast_sin(radians);
//     mat2 rotationMatrix = mat2(c, -s, s, c);
//     return rotationMatrix * uv;
// }
vec2 rotate_uv_domain_fast(vec2 uv, float radians) {
    float r = radians;
    float r2 = r * r;
    float approx_cos = 1.0 - 0.5 * r2;
    float approx_sin = r;
    mat2 rotationMatrix = mat2(approx_cos, -approx_sin, approx_sin, approx_cos);
    return rotationMatrix * uv;
}



void main() {

    vec2 screen_uv = gl_FragCoord.xy / uResolution.xy;

    vec2 noise_coord = gl_FragCoord.xy;
    float noise_big = snoise_octaved(
        noise_coord / 100.0 / 2.0 / 2.0 / 2.0 / 2.0, 
        2, 
        1.5
    );


    float waves_1 = rotate_uv_domain_fast(gl_FragCoord.xy, -0.9).x * (0.0025) + uTime * 0.05 + noise_big * 1.5;
    float waves_bands = trunc(waves_1);
    float waves_fracted = fract(waves_1);

    float noise_big2 = snoise_octaved(
        rotate_uv_domain_fast(gl_FragCoord.xy, -2.5) * vec2(0.2, 4.0) / 1000.0 / 5.0 + vec2(321.0, waves_bands * 50.0),
        2, 
        1.5 
    ) - 0.5;
    float waves_2 = abs((waves_fracted - 0.5 + noise_big2) * 2.0);



    // vec3 waves_gradient = palette(
    //     waves_2,
    //     vec3(0.938, 0.328, 0.718),
    //     vec3(0.659, 0.438, 0.328),
    //     vec3(0.388, 0.388, 0.296),
    //     vec3(2.538, 2.478, 0.168)
    // );
    // vec3 waves_gradient2 = palette(
    //     waves_2,
    //     vec3(0.000, 0.500, 0.500), 
    //     vec3(0.000, 0.500, 0.500), 
    //     vec3(0.000, 0.500, 0.333), 
    //     vec3(0.000, 0.500, 0.667)
    // );
    vec3 color = mix(
        // clamp(waves_gradient, 0.0, 1.0), 
        // clamp(waves_gradient2, 0.0, 1.0), 
        vec3(0.0),
        vec3(waves_2 * 0.5),
        clamp(waves_fracted, 0.0, 1.0)
    );

    // add border
    float waves_border_threshold = 0.99;
    if (waves_fracted > waves_border_threshold || waves_fracted < 1.0 - waves_border_threshold) {
        // TailwindCSS::blue-400
        // color = vec3(0.3765, 0.6471, 0.9804) * 1.5;
        // color = vec3(0.835, 0.894, 0.929); // light cloud
        color = vec3(0.502, 0.62, 0.706); // dark cloud
        // TailwindCSS::blue-800
        // color = vec3(1.0, 1.0, 1.0);
    }

    // vec3 gradient = palette(
    //     1.0 - color.r * 1.0,
    //     vec3(0.360, -0.180, 0.530), 
    //     vec3(0.310, 1.030, 0.320), 
    //     vec3(0.650, 0.390, 0.510), 
    //     vec3(1.970, 0.983, 1.967)
    // );
    vec3 gradient = mix(
        vec3(0.063, 0.094, 0.157), // dark background
        vec3(0.502, 0.62, 0.706), // dark cloud 
        // vec3(0.835, 0.894, 0.929), // light cloud
        clamp(color.r, 0.0, 1.0)
    );

    float bloom = pow(color.r, 2.0) * 0.5;

    color = gradient + bloom;
    // color = gradient;


	gl_FragColor = vec4(
        color,
        1.0
    );
}

