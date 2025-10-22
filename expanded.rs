#![feature(prelude_import)]
//! # A website that just exists for now, no flexy / fancy things etc.
//!
//! - [x] bootstrap
//! - [ ] [`thaw`] ui
//! - [x] vibecode old site to new one with some AI
//! - [ ] simple css
//! - [ ] glsl backgroung
//! - [ ] link tree replacement with all links that are out there
//! - [ ] push to github pages
#![allow(non_snake_case)]
#![feature(cold_path)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use leptos::prelude::*;
mod circles {
    use leptos::prelude::*;
    /// Props for the [`Patterns`] component.
    ///
    ///
    /// # Required Props
    /// - **amount**: [`usize`]
    /// - **radius_base**: [`usize`]
    /// - **radius_max**: [`usize`]
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct PatternsProps {
        #[builder(setter(doc = "**amount**: [`usize`]"))]
        pub amount: usize,
        #[builder(setter(doc = "**radius_base**: [`usize`]"))]
        pub radius_base: usize,
        #[builder(setter(doc = "**radius_max**: [`usize`]"))]
        pub radius_max: usize,
    }
    #[automatically_derived]
    impl PatternsProps {
        /**
                Create a builder for building `PatternsProps`.
                On the builder, call `.amount(...)`, `.radius_base(...)`, `.radius_max(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `PatternsProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> PatternsPropsBuilder<((), (), ())> {
            PatternsPropsBuilder {
                fields: ((), (), ()),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct PatternsPropsBuilder<TypedBuilderFields = ((), (), ())> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for PatternsPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<
        __radius_base,
        __radius_max,
    > PatternsPropsBuilder<((), __radius_base, __radius_max)> {
        ///**amount**: [`usize`]
        #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
        pub fn amount(
            self,
            amount: usize,
        ) -> PatternsPropsBuilder<((usize,), __radius_base, __radius_max)> {
            let amount = (amount,);
            let ((), radius_base, radius_max) = self.fields;
            PatternsPropsBuilder {
                fields: (amount, radius_base, radius_max),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Repeated_field_amount {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<
        __radius_base,
        __radius_max,
    > PatternsPropsBuilder<((usize,), __radius_base, __radius_max)> {
        #[deprecated(note = "Repeated field amount")]
        ///**amount**: [`usize`]
        pub fn amount(
            self,
            _: PatternsPropsBuilder_Error_Repeated_field_amount,
        ) -> PatternsPropsBuilder<((usize,), __radius_base, __radius_max)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<__amount, __radius_max> PatternsPropsBuilder<(__amount, (), __radius_max)> {
        ///**radius_base**: [`usize`]
        #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
        pub fn radius_base(
            self,
            radius_base: usize,
        ) -> PatternsPropsBuilder<(__amount, (usize,), __radius_max)> {
            let radius_base = (radius_base,);
            let (amount, (), radius_max) = self.fields;
            PatternsPropsBuilder {
                fields: (amount, radius_base, radius_max),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Repeated_field_radius_base {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<
        __amount,
        __radius_max,
    > PatternsPropsBuilder<(__amount, (usize,), __radius_max)> {
        #[deprecated(note = "Repeated field radius_base")]
        ///**radius_base**: [`usize`]
        pub fn radius_base(
            self,
            _: PatternsPropsBuilder_Error_Repeated_field_radius_base,
        ) -> PatternsPropsBuilder<(__amount, (usize,), __radius_max)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<__amount, __radius_base> PatternsPropsBuilder<(__amount, __radius_base, ())> {
        ///**radius_max**: [`usize`]
        #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
        pub fn radius_max(
            self,
            radius_max: usize,
        ) -> PatternsPropsBuilder<(__amount, __radius_base, (usize,))> {
            let radius_max = (radius_max,);
            let (amount, radius_base, ()) = self.fields;
            PatternsPropsBuilder {
                fields: (amount, radius_base, radius_max),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Repeated_field_radius_max {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<
        __amount,
        __radius_base,
    > PatternsPropsBuilder<(__amount, __radius_base, (usize,))> {
        #[deprecated(note = "Repeated field radius_max")]
        ///**radius_max**: [`usize`]
        pub fn radius_max(
            self,
            _: PatternsPropsBuilder_Error_Repeated_field_radius_max,
        ) -> PatternsPropsBuilder<(__amount, __radius_base, (usize,))> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Missing_required_field_amount {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    #[automatically_derived]
    impl<
        __radius_base,
        __radius_max,
    > PatternsPropsBuilder<((), __radius_base, __radius_max)> {
        #[deprecated(note = "Missing required field amount")]
        pub fn build(
            self,
            _: PatternsPropsBuilder_Error_Missing_required_field_amount,
        ) -> ! {
            ::core::panicking::panic("explicit panic")
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Missing_required_field_radius_base {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    #[automatically_derived]
    impl<__radius_max> PatternsPropsBuilder<((usize,), (), __radius_max)> {
        #[deprecated(note = "Missing required field radius_base")]
        pub fn build(
            self,
            _: PatternsPropsBuilder_Error_Missing_required_field_radius_base,
        ) -> ! {
            ::core::panicking::panic("explicit panic")
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum PatternsPropsBuilder_Error_Missing_required_field_radius_max {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    #[automatically_derived]
    impl PatternsPropsBuilder<((usize,), (usize,), ())> {
        #[deprecated(note = "Missing required field radius_max")]
        pub fn build(
            self,
            _: PatternsPropsBuilder_Error_Missing_required_field_radius_max,
        ) -> ! {
            ::core::panicking::panic("explicit panic")
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl PatternsPropsBuilder<((usize,), (usize,), (usize,))> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> PatternsProps {
            let (amount, radius_base, radius_max) = self.fields;
            let amount = amount.0;
            let radius_base = radius_base.0;
            let radius_max = radius_max.0;
            #[allow(deprecated)]
            PatternsProps {
                amount,
                radius_base,
                radius_max,
            }
                .into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for PatternsProps {
        type Builder = PatternsPropsBuilder;
        fn builder() -> Self::Builder {
            PatternsProps::builder()
        }
    }
    ///
    /// # Required Props
    /// - **amount**: [`usize`]
    /// - **radius_base**: [`usize`]
    /// - **radius_max**: [`usize`]
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    pub fn Patterns(props: PatternsProps) -> impl IntoView {
        let PatternsProps { amount, radius_base, radius_max } = props;
        ::leptos::reactive::graph::untrack_with_diagnostics(move || {
            __component_patterns(amount, radius_base, radius_max)
        })
    }
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments, clippy::needless_lifetimes)]
    pub fn __component_patterns(
        amount: usize,
        radius_base: usize,
        radius_max: usize,
    ) -> impl IntoView {
        let _ = (amount, radius_base, radius_max);
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new(
                        ::leptos::tachys::html::element::div()
                            .child((
                                "There are ",
                                ::leptos::prelude::IntoRender::into_render({ amount }),
                                " things, just use your ImAgInAtIoN",
                            )),
                    )
                    .with_view_marker("src-circles.rs-7")
            }
        }
    }
}
mod consts {
    pub mod socials {
        pub const BLUE_SKY: &'static str = "https://bsky.app/profile/yangvar6.bsky.social";
        pub const GITHUB: &'static str = "https://github.com/yangvar6";
        pub const INSTAGRAM: &'static str = "https://www.instagram.com/yangvar6";
        pub const MAILTO: &'static str = "mailto:yangvar6.media@gmail.com";
        pub const TELEGRAM: &'static str = "https://t.me/yangvar6";
        pub const TIKTOK: &'static str = "https://www.tiktok.com/@yangvar6";
        pub const WEBSITE: &'static str = "https://yangvar6.com/";
        pub const XDOTCOM: &'static str = "https://x.com/yangvar6";
    }
}
mod glsl {
    use js_sys::Date;
    use leptos::html::Canvas;
    use leptos::prelude::*;
    use leptos::*;
    use std::cell::RefCell;
    use std::mem::MaybeUninit;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use web_sys::{WebGl2RenderingContext as WebGl, WebGlProgram, WebGlShader};
    const MAZE_SHADER_VERT: &'static str = "#version 450\r\nprecision highp float;\r\n\r\nvarying vec2 vUv;\r\n\r\nvoid main() {\r\n    // vUv = uv;\r\n    // gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);\r\n}\r\n";
    const MAZE_SHADER_FRAG: &'static str = "#version 450\r\nprecision highp float;\r\n\r\n// uniform float uCanvasWidth;\r\n// uniform float uCanvasHeight;\r\nuniform float uTime;\r\nvarying vec2 vUv;\r\nuniform vec2 uResolution;\r\n\r\n\r\n//\r\n// Description : Array and textureless GLSL 2D simplex noise function.\r\n//      Author : Ian McEwan, Ashima Arts.\r\n//  Maintainer : stegu\r\n//     Lastmod : 20110822 (ijm)\r\n//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.\r\n//               Distributed under the MIT License. See LICENSE file.\r\n//               https://github.com/ashima/webgl-noise\r\n//               https://github.com/stegu/webgl-noise\r\n// \r\n\r\n\r\nvec3 mod289(vec3 x) {\r\n  return x - floor(x * (1.0 / 289.0)) * 289.0;\r\n}\r\n\r\nvec2 mod289(vec2 x) {\r\n  return x - floor(x * (1.0 / 289.0)) * 289.0;\r\n}\r\n\r\nvec3 permute(vec3 x) {\r\n  return mod289(((x*34.0)+10.0)*x);\r\n}\r\n\r\nfloat snoise(vec2 v)\r\n  {\r\n  const vec4 C = vec4(0.211324865405187,  // (3.0-sqrt(3.0))/6.0\r\n                      0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)\r\n                     -0.577350269189626,  // -1.0 + 2.0 * C.x\r\n                      0.024390243902439); // 1.0 / 41.0\r\n// First corner\r\n  vec2 i  = floor(v + dot(v, C.yy) );\r\n  vec2 x0 = v -   i + dot(i, C.xx);\r\n\r\n// Other corners\r\n  vec2 i1;\r\n  //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0\r\n  //i1.y = 1.0 - i1.x;\r\n  i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);\r\n  // x0 = x0 - 0.0 + 0.0 * C.xx ;\r\n  // x1 = x0 - i1 + 1.0 * C.xx ;\r\n  // x2 = x0 - 1.0 + 2.0 * C.xx ;\r\n  vec4 x12 = x0.xyxy + C.xxzz;\r\n  x12.xy -= i1;\r\n\r\n// Permutations\r\n  i = mod289(i); // Avoid truncation effects in permutation\r\n  vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))\r\n\t\t+ i.x + vec3(0.0, i1.x, 1.0 ));\r\n\r\n  vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);\r\n  m = m*m ;\r\n  m = m*m ;\r\n\r\n// Gradients: 41 points uniformly over a line, mapped onto a diamond.           // What the magickal spell is this?\r\n// The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)\r\n\r\n  vec3 x = 2.0 * fract(p * C.www) - 1.0;\r\n  vec3 h = abs(x) - 0.5;\r\n  vec3 ox = floor(x + 0.5);\r\n  vec3 a0 = x - ox;\r\n\r\n// Normalise gradients implicitly by scaling m\r\n// Approximation of: m *= inversesqrt( a0*a0 + h*h );\r\n  m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );\r\n\r\n// Compute final noise value at P\r\n  vec3 g;\r\n  g.x  = a0.x  * x0.x  + h.x  * x0.y;\r\n  g.yz = a0.yz * x12.xz + h.yz * x12.yw;\r\n  return 130.0 * dot(m, g);\r\n}\r\n\r\n\r\nfloat snoise_octaved(vec2 pos, int octaves, float lacunarity) {\r\n    float value = 0.0;\r\n    float amplitude = 1.0;\r\n    float frequency = 1.0;\r\n    float totalAmplitude = 0.0;\r\n    \r\n    for (int i = 0; i < octaves; i++) {\r\n        float s = (snoise(pos * frequency) * 0.5) + 0.5;\r\n        value += amplitude * s;\r\n        totalAmplitude += amplitude;\r\n        amplitude *= 0.5;\r\n        frequency *= lacunarity;\r\n        pos += vec2(321.0, -54.1);\r\n    }\r\n    \r\n    return value / totalAmplitude;\r\n}\r\n\r\nfloat trunc(float x) {\r\n    return float(int(x));\r\n}\r\n\r\nfloat fast_sin(float x){\r\n   x = fract( 0.75 + x*0.159155 )*2.0 - 1.0; \r\n   return x*x * (6.0 - 4.0*x) - 1.0;\r\n}\r\n\r\nfloat fast_cos(float x) {\r\n    return fast_sin(x + 1.5707963267948966);\r\n}\r\n\r\nvec3 fast_cos(vec3 x) {\r\n    return vec3(\r\n        fast_cos(x.x),\r\n        fast_cos(x.y),\r\n        fast_cos(x.z)\r\n    );\r\n}\r\n\r\n\r\n// https://iquilezles.org/articles/palettes/\r\nvec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d )\r\n{\r\n    return a + b * cos( 6.283185*(c*t+d) );\r\n\r\n    // gives strange visual artifacts that looks cool tbh \u{1f609}\u{1f609}\u{1f609}\r\n    // this is how i justify mo code quality )))\r\n    // ok, its too broken\r\n    // return a + b * fast_cos( 6.283185*(c*t+d) );\r\n}\r\n\r\n\r\n// vec2 rotate_uv_domain_slow(vec2 uv, float radians) {\r\n//     float c = cos(radians);\r\n//     float s = sin(radians);\r\n//     mat2 rotationMatrix = mat2(c, -s, s, c);\r\n//     return rotationMatrix * uv;\r\n// }\r\n// vec2 rotate_uv_domain_fast(vec2 uv, float radians) {\r\n//     float c = fast_cos(radians);\r\n//     float s = fast_sin(radians);\r\n//     mat2 rotationMatrix = mat2(c, -s, s, c);\r\n//     return rotationMatrix * uv;\r\n// }\r\nvec2 rotate_uv_domain_fast(vec2 uv, float radians) {\r\n    float r = radians;\r\n    float r2 = r * r;\r\n    float approx_cos = 1.0 - 0.5 * r2;\r\n    float approx_sin = r;\r\n    mat2 rotationMatrix = mat2(approx_cos, -approx_sin, approx_sin, approx_cos);\r\n    return rotationMatrix * uv;\r\n}\r\n\r\n\r\n\r\nvoid main() {\r\n\r\n    vec2 screen_uv = gl_FragCoord.xy / uResolution.xy;\r\n\r\n    vec2 noise_coord = gl_FragCoord.xy;\r\n    float noise_big = snoise_octaved(\r\n        noise_coord / 100.0 / 2.0 / 2.0 / 2.0 / 2.0, \r\n        2, \r\n        1.5\r\n    );\r\n\r\n\r\n    float waves_1 = rotate_uv_domain_fast(gl_FragCoord.xy, -0.9).x * (0.0025) + uTime * 0.05 + noise_big * 1.5;\r\n    float waves_bands = trunc(waves_1);\r\n    float waves_fracted = fract(waves_1);\r\n\r\n    float noise_big2 = snoise_octaved(\r\n        rotate_uv_domain_fast(gl_FragCoord.xy, -2.5) * vec2(0.2, 4.0) / 1000.0 / 5.0 + vec2(321.0, waves_bands * 50.0),\r\n        2, \r\n        1.5 \r\n    ) - 0.5;\r\n    float waves_2 = abs((waves_fracted - 0.5 + noise_big2) * 2.0);\r\n\r\n\r\n\r\n    // vec3 waves_gradient = palette(\r\n    //     waves_2,\r\n    //     vec3(0.938, 0.328, 0.718),\r\n    //     vec3(0.659, 0.438, 0.328),\r\n    //     vec3(0.388, 0.388, 0.296),\r\n    //     vec3(2.538, 2.478, 0.168)\r\n    // );\r\n    // vec3 waves_gradient2 = palette(\r\n    //     waves_2,\r\n    //     vec3(0.000, 0.500, 0.500), \r\n    //     vec3(0.000, 0.500, 0.500), \r\n    //     vec3(0.000, 0.500, 0.333), \r\n    //     vec3(0.000, 0.500, 0.667)\r\n    // );\r\n    vec3 color = mix(\r\n        // clamp(waves_gradient, 0.0, 1.0), \r\n        // clamp(waves_gradient2, 0.0, 1.0), \r\n        vec3(0.0),\r\n        vec3(waves_2 * 0.5),\r\n        clamp(waves_fracted, 0.0, 1.0)\r\n    );\r\n\r\n    // add border\r\n    float waves_border_threshold = 0.99;\r\n    if (waves_fracted > waves_border_threshold || waves_fracted < 1.0 - waves_border_threshold) {\r\n        // TailwindCSS::blue-400\r\n        // color = vec3(0.3765, 0.6471, 0.9804) * 1.5;\r\n        // color = vec3(0.835, 0.894, 0.929); // light cloud\r\n        color = vec3(0.502, 0.62, 0.706); // dark cloud\r\n        // TailwindCSS::blue-800\r\n        // color = vec3(1.0, 1.0, 1.0);\r\n    }\r\n\r\n    // vec3 gradient = palette(\r\n    //     1.0 - color.r * 1.0,\r\n    //     vec3(0.360, -0.180, 0.530), \r\n    //     vec3(0.310, 1.030, 0.320), \r\n    //     vec3(0.650, 0.390, 0.510), \r\n    //     vec3(1.970, 0.983, 1.967)\r\n    // );\r\n    vec3 gradient = mix(\r\n        vec3(0.063, 0.094, 0.157), // dark background\r\n        vec3(0.502, 0.62, 0.706), // dark cloud \r\n        // vec3(0.835, 0.894, 0.929), // light cloud\r\n        clamp(color.r, 0.0, 1.0)\r\n    );\r\n\r\n    float bloom = pow(color.r, 2.0) * 0.5;\r\n\r\n    color = gradient + bloom;\r\n    // color = gradient;\r\n\r\n\r\n\tgl_FragColor = vec4(\r\n        color,\r\n        1.0\r\n    );\r\n}\r\n\r\n";
    /// Props for the [`BackgroundCanvas`] component.
    ///
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct BackgroundCanvasProps {}
    #[automatically_derived]
    impl BackgroundCanvasProps {
        /**
                Create a builder for building `BackgroundCanvasProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `BackgroundCanvasProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> BackgroundCanvasPropsBuilder<()> {
            BackgroundCanvasPropsBuilder {
                fields: (),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct BackgroundCanvasPropsBuilder<TypedBuilderFields = ()> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for BackgroundCanvasPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl BackgroundCanvasPropsBuilder<()> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> BackgroundCanvasProps {
            let () = self.fields;
            #[allow(deprecated)] BackgroundCanvasProps {}.into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for BackgroundCanvasProps {
        type Builder = BackgroundCanvasPropsBuilder;
        fn builder() -> Self::Builder {
            BackgroundCanvasProps::builder()
        }
    }
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    pub fn BackgroundCanvas() -> impl IntoView {
        ::leptos::reactive::graph::untrack_with_diagnostics(move || {
            __component_background_canvas()
        })
    }
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments, clippy::needless_lifetimes)]
    pub fn __component_background_canvas() -> impl IntoView {
        let canvas_ref = NodeRef::<Canvas>::new();
        Effect::new(move |_| {
            return;
            let Some(canvas) = canvas_ref.get() else {
                ::leptos_dom::logging::console_error(
                    &format_args!("Canvas not found").to_string(),
                );
                return;
            };
            let gl = canvas
                .get_context("webgl2")
                .ok()
                .flatten()
                .and_then(|ctx| ctx.dyn_into::<WebGl>().ok())
                .expect("Failed to create WebGL2 context!");
            gl.disable(WebGl::DEPTH_TEST);
            let width = canvas.width() as f32;
            let height = canvas.height() as f32;
            let Ok(vert_shader) = compile_shader(
                &gl,
                WebGl::VERTEX_SHADER,
                MAZE_SHADER_VERT,
            ) else {
                std::hint::cold_path();
                ::leptos_dom::logging::console_error(
                    &format_args!("Failed to compile vertex shader").to_string(),
                );
                return;
            };
            let Ok(frag_shader) = compile_shader(
                &gl,
                WebGl::FRAGMENT_SHADER,
                MAZE_SHADER_FRAG,
            ) else {
                std::hint::cold_path();
                ::leptos_dom::logging::console_error(
                    &format_args!("Failed to compile fragment shader").to_string(),
                );
                return;
            };
            let program = link_program(&gl, &vert_shader, &frag_shader)
                .expect("Failed to link shader program");
            gl.use_program(Some(&program));
            let u_time = gl.get_uniform_location(&program, "uTime");
            let u_resolution = gl.get_uniform_location(&program, "uResolution");
            let vertices: &[f32] = &[
                -15.0,
                -15.0,
                0.0,
                15.0,
                -15.0,
                0.0,
                15.0,
                15.0,
                0.0,
                -15.0,
                15.0,
                0.0,
            ];
            let vertex_buffer = gl
                .create_buffer()
                .expect("Failed to create vertex buffer");
            gl.bind_buffer(WebGl::ARRAY_BUFFER, Some(&vertex_buffer));
            let vertices_array = js_sys::Float32Array::new_with_length(
                vertices.len() as u32,
            );
            vertices_array.copy_from(vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl::ARRAY_BUFFER,
                &vertices_array,
                WebGl::STATIC_DRAW,
            );
            let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
            let index_buffer = gl
                .create_buffer()
                .expect("Failed to create index buffer");
            gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
            let indices_array = js_sys::Uint16Array::new_with_length(
                indices.len() as u32,
            );
            indices_array.copy_from(indices);
            gl.buffer_data_with_array_buffer_view(
                WebGl::ELEMENT_ARRAY_BUFFER,
                &indices_array,
                WebGl::STATIC_DRAW,
            );
            let position = gl.get_attrib_location(&program, "a_position") as u32;
            gl.vertex_attrib_pointer_with_i32(position, 3, WebGl::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(position);
            let time = Rc::new(RefCell::new(0.0));
            let gl = Rc::new(gl);
            let program = Rc::new(program);
            let canvas = Rc::new(canvas);
            let mut render_loop = MaybeUninit::<Closure<dyn FnMut()>>::uninit();
            let render_loop_ref = unsafe {
                render_loop.as_mut_ptr().as_ref().unwrap_unchecked()
            };
            render_loop
                .write(
                    Closure::new(move || {
                        let now = Date::now() / 1000.0;
                        *time.borrow_mut() = now as f32;
                        let gl = gl.clone();
                        let program = program.clone();
                        let canvas = canvas.clone();
                        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
                        gl.clear_color(0.0, 0.0, 0.0, 1.0);
                        gl.clear(WebGl::COLOR_BUFFER_BIT);
                        if let Some(loc) = u_time.as_ref() {
                            gl.uniform1f(Some(loc), *time.borrow());
                        }
                        if let Some(loc) = u_resolution.as_ref() {
                            gl.uniform2f(Some(loc), width, height);
                        }
                        gl.draw_elements_with_i32(
                            WebGl::TRIANGLES,
                            6,
                            WebGl::UNSIGNED_SHORT,
                            0,
                        );
                        web_sys::window()
                            .unwrap()
                            .request_animation_frame(
                                render_loop_ref.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }),
                );
            let closure: &js_sys::Function = unsafe { render_loop.assume_init_ref() }
                .as_ref()
                .unchecked_ref();
            web_sys::window().unwrap().request_animation_frame(closure).unwrap();
        });
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new(
                        ::leptos::tachys::html::element::canvas()
                            .class("w-full h-full")
                            .node_ref(canvas_ref)
                            .width("800")
                            .height("600"),
                    )
                    .with_view_marker("src-glsl.rs-144")
            }
        }
    }
    fn compile_shader(
        gl: &WebGl,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, Box<str>> {
        let shader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        let Some(_compiled) = gl
            .get_shader_parameter(&shader, WebGl::COMPILE_STATUS)
            .as_bool() else {
            std::hint::cold_path();
            return Err(
                gl
                    .get_shader_info_log(&shader)
                    .map(String::into_boxed_str)
                    .unwrap_or(Box::from("Unknown shader error")),
            );
        };
        Ok(shader)
    }
    fn link_program(
        gl: &WebGl,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, Box<str>> {
        let program = gl.create_program().ok_or("Unable to create program")?;
        gl.attach_shader(&program, vert_shader);
        gl.attach_shader(&program, frag_shader);
        gl.link_program(&program);
        let Some(_linked) = gl
            .get_program_parameter(&program, WebGl::LINK_STATUS)
            .as_bool() else {
            std::hint::cold_path();
            return Err(
                gl
                    .get_program_info_log(&program)
                    .map(String::into_boxed_str)
                    .unwrap_or(Box::from("Unknown program error")),
            );
        };
        Ok(program)
    }
}
#[allow(dead_code)]
pub mod style {
    const _: &[u8] = b"@import \"tailwindcss\";\r\n\r\n\r\n:root {\r\n    font-family: \'Montserrat\', sans-serif, system-ui, Avenir, Helvetica, Arial, sans-serif;\r\n    line-height: 1.5;\r\n    font-weight: 400;\r\n\r\n    color-scheme: light dark;\r\n    color: rgba(255, 255, 255, 0.87);\r\n    background-color: #242424;\r\n\r\n    font-synthesis: none;\r\n    text-rendering: optimizeLegibility;\r\n    -webkit-font-smoothing: antialiased;\r\n    -moz-osx-font-smoothing: grayscale;\r\n}\r\n\r\n\r\nbody {\r\n    min-width: 375px;\r\n    min-height: 100vh;\r\n    @apply flex flex-col items-center justify-between;\r\n    @apply overflow-x-hidden md:overflow-hidden;\r\n}\r\n\r\n\r\nh1 {\r\n    font-weight: 1000;\r\n}\r\n\r\nh2 {\r\n    font-weight: 600;\r\n    @apply text-[20px];\r\n}\r\n\r\na {\r\n    @apply underline;\r\n    @apply hover:text-yellow-400 hover:cursor-pointer;\r\n}\r\n\r\n.a {\r\n    @apply underline;\r\n    @apply hover:text-yellow-400 hover:cursor-pointer;\r\n}\r\n\r\np {\r\n    @apply text-gray-300;\r\n}\r\n\r\n.p_acc {\r\n    @apply text-yellow-600;\r\n}\r\n\r\n::selection {\r\n    background-color: orange;\r\n    color: black;\r\n}\r\n\r\n::-moz-selection {\r\n    background-color: orange;\r\n    color: black;\r\n}\r\n\r\n::-webkit-selection {\r\n    background-color: orange;\r\n    color: black;\r\n}\r\n\r\n\r\n@media (prefers-color-scheme: light) {\r\n    :root {\r\n        color: #213547;\r\n        background-color: #ffffff;\r\n    }\r\n}\r\n\r\n\r\n.jumbotron {\r\n    background: blue;\r\n}";
    #[allow(non_upper_case_globals)]
    pub const a: &str = "a-13007a9";
    #[allow(non_upper_case_globals)]
    pub const jumbotron: &str = "jumbotron-13007a9";
    #[allow(non_upper_case_globals)]
    pub const p_acc: &str = "p_acc-13007a9";
}
/// WASM environment btw...
fn main() {
    leptos::mount::mount_to_body(App2)
}
/// Props for the [`App`] component.
///
#[builder(crate_module_path = ::leptos::typed_builder)]
#[allow(non_snake_case)]
struct AppProps {}
#[automatically_derived]
impl AppProps {
    /**
                Create a builder for building `AppProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `AppProps`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> AppPropsBuilder<()> {
        AppPropsBuilder {
            fields: (),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct AppPropsBuilder<TypedBuilderFields = ()> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for AppPropsBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl AppPropsBuilder<()> {
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> AppProps {
        let () = self.fields;
        #[allow(deprecated)] AppProps {}.into()
    }
}
#[allow(missing_docs)]
impl ::leptos::component::Props for AppProps {
    type Builder = AppPropsBuilder;
    fn builder() -> Self::Builder {
        AppProps::builder()
    }
}
#[allow(non_snake_case, clippy::too_many_arguments)]
#[allow(clippy::needless_lifetimes)]
fn App() -> impl IntoView {
    ::leptos::reactive::graph::untrack_with_diagnostics(move || { __component_app() })
}
#[doc(hidden)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes)]
pub fn __component_app() -> impl IntoView {
    let (count, set_count) = signal(0);
    {
        #[allow(unused_braces)]
        {
            ::leptos::prelude::View::new(
                    ::leptos::tachys::html::element::main()
                        .child((
                            ::leptos::tachys::html::element::h1()
                                .child(#[allow(unused_braces)] { "Hello, world!" })
                                .class(style::jumbotron),
                            ::leptos::tachys::html::element::div()
                                .child((
                                    ::leptos::tachys::html::element::p()
                                        .child((
                                            "This is a counter: ",
                                            ::leptos::prelude::IntoRender::into_render({ count }),
                                        )),
                                    ::leptos::tachys::html::element::button()
                                        .child(#[allow(unused_braces)] { "+1" })
                                        .on(
                                            ::leptos::tachys::html::event::click,
                                            move |_| set_count.set(count.get() + 1),
                                        ),
                                    ::leptos::tachys::html::element::button()
                                        .child(#[allow(unused_braces)] { "-1" })
                                        .on(
                                            ::leptos::tachys::html::event::click,
                                            move |_| set_count.set(count.get() - 1),
                                        ),
                                )),
                        )),
                )
                .with_view_marker("src-main.rs-31")
        }
    }
}
/// Props for the [`App2`] component.
///
#[builder(crate_module_path = ::leptos::typed_builder)]
#[allow(non_snake_case)]
struct App2Props {}
#[automatically_derived]
impl App2Props {
    /**
                Create a builder for building `App2Props`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `App2Props`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> App2PropsBuilder<()> {
        App2PropsBuilder {
            fields: (),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct App2PropsBuilder<TypedBuilderFields = ()> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for App2PropsBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl App2PropsBuilder<()> {
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> App2Props {
        let () = self.fields;
        #[allow(deprecated)] App2Props {}.into()
    }
}
#[allow(missing_docs)]
impl ::leptos::component::Props for App2Props {
    type Builder = App2PropsBuilder;
    fn builder() -> Self::Builder {
        App2Props::builder()
    }
}
#[allow(non_snake_case, clippy::too_many_arguments)]
#[allow(clippy::needless_lifetimes)]
fn App2() -> impl IntoView {
    ::leptos::reactive::graph::untrack_with_diagnostics(move || { __component_app_2() })
}
#[doc(hidden)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes)]
pub fn __component_app_2() -> impl IntoView {
    {
        #[allow(unused_braces)]
        {
            (
                ::leptos::tachys::html::element::section()
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::leptos::tachys::html::element::div()
                                .child((
                                    ::leptos::tachys::html::element::div()
                                        .child((
                                            ::leptos::tachys::html::element::a()
                                                .child(#[allow(unused_braces)] { "INSTAGRAM" })
                                                .href(
                                                    ::leptos::prelude::IntoAttributeValue::into_attribute_value(
                                                        consts::socials::INSTAGRAM,
                                                    ),
                                                ),
                                            ::leptos::tachys::html::element::a()
                                                .child(#[allow(unused_braces)] { "Gmail" })
                                                .href(
                                                    ::leptos::prelude::IntoAttributeValue::into_attribute_value(
                                                        consts::socials::MAILTO,
                                                    ),
                                                ),
                                            ::leptos::tachys::html::element::a()
                                                .child(#[allow(unused_braces)] { "Github" })
                                                .href(
                                                    ::leptos::prelude::IntoAttributeValue::into_attribute_value(
                                                        consts::socials::GITHUB,
                                                    ),
                                                ),
                                            ::leptos::tachys::html::element::a()
                                                .child(#[allow(unused_braces)] { "TIKTOK" })
                                                .href(
                                                    ::leptos::prelude::IntoAttributeValue::into_attribute_value(
                                                        consts::socials::TIKTOK,
                                                    ),
                                                ),
                                            ::leptos::tachys::html::element::a()
                                                .child(#[allow(unused_braces)] { "XDOTCOM" })
                                                .href(
                                                    ::leptos::prelude::IntoAttributeValue::into_attribute_value(
                                                        consts::socials::XDOTCOM,
                                                    ),
                                                ),
                                        ))
                                        .class("flex flex-row self-end md:self-start gap-x-4"),
                                    ::leptos::tachys::html::InertElement::new(
                                        "<div class=\"self-end\"><a>Linkzz</a></div>",
                                    ),
                                ))
                                .class(
                                    "flex flex-col md:flex-row w-[90%] md:w-1/2 h-fit justify-between md:gap-[42px] gap-[6px] py-6",
                                )
                        },
                    )
                    .class("flex flex-row w-full justify-center h-fit"),
                ::leptos::tachys::html::element::section()
                    .child((
                        ::leptos::tachys::html::InertElement::new(
                            "<div class=\"md:hidden flex h-[90px]\"></div>",
                        ),
                        ::leptos::tachys::html::element::div()
                            .child(
                                #[allow(unused_braces)]
                                {
                                    {
                                        #[allow(unreachable_code)] #[allow(unused_mut)]
                                        #[allow(clippy::let_and_return)]
                                        ::leptos::component::component_view(
                                            #[allow(clippy::needless_borrows_for_generic_args)]
                                            &circles::Patterns,
                                            {
                                                let mut props = ::leptos::component::component_props_builder(
                                                        &circles::Patterns,
                                                    )
                                                    .amount(#[allow(unused_braces)] { 10 })
                                                    .radius_base(#[allow(unused_braces)] { 200 })
                                                    .radius_max(#[allow(unused_braces)] { 800 })
                                                    .build();
                                                props
                                            },
                                        )
                                    }
                                },
                            )
                            .class(
                                "flex flex-col w-full justify-center items-center p-[42px]",
                            ),
                        ::leptos::tachys::html::InertElement::new(
                            "<div class=\"md:hidden flex h-[60px]\"></div>",
                        ),
                        ::leptos::tachys::html::InertElement::new(
                            "<div class=\"flex flex-col w-full justify-center items-center gap-[28px]\"><div class=\"flex flex-col w-[90%] max-w-[550px] min-w-[0px] md:min-w-[440px] justify-center items-center gap-[28px]\"><div class=\"flex flex-col w-full gap-[6px]\"><a><h1 class=\"text-[48px] font-extrabold\">Yan B.</h1></a><h1 class=\"text-[38px] text-left text-yellow-600\">Do lorem</h1><h1 class=\"text-[38px] text-right text-yellow-600\">Ipsum</h1></div><div class=\"flex flex-col w-full gap-[6px]\"><div class=\"flex flex-col justify-center items-center\"><div class=\"w-[80%] flex flex-col gap-2\"><p class=\"bg-gray-900/80 p-2\">🤗 I'm a ...... <span class=\"p_acc\">............</span> .....................</p><p class=\"bg-gray-900/80 p-2\">🤗 ...... ..... ... ....... .......... ........... ...... . <span class=\"p_acc\">..... .....</span>, <span class=\"p_acc\">... ..</span> ................. <span class=\"p_acc\">....... ..... .........</span> .......... ....... ....... </p><p class=\"bg-gray-900/80 p-2\">🤗 <span class=\"p_acc\">.....</span> ...... . <span class=\"p_acc\">... .... ...</span> ... .. .................. ........ ..... ...... ............ ............ ........... </p></div></div></div></div></div>",
                        ),
                    ))
                    .class(
                        "flex flex-col md:flex-row w-full h-fit max-w-[1200px] gap-[5px] md:gap-[60px]",
                    ),
                ::leptos::tachys::html::element::section()
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::leptos::tachys::html::InertElement::new(
                                "<p class=\"w-[90%] md:w-1/2 text-center opacity-70\">This website was built by me over a weekend using Leptos, TailwindCSS, Rust, and WebGLGLGLGLGLGL.</p>",
                            )
                        },
                    )
                    .class("flex flex-row w-full h-fit justify-center py-6"),
                ::leptos::tachys::html::element::section()
                    .child(
                        #[allow(unused_braces)]
                        {
                            {
                                #[allow(unreachable_code)] #[allow(unused_mut)]
                                #[allow(clippy::let_and_return)]
                                ::leptos::component::component_view(
                                    #[allow(clippy::needless_borrows_for_generic_args)]
                                    &glsl::BackgroundCanvas,
                                    {
                                        let mut props = ::leptos::component::component_props_builder(
                                                &glsl::BackgroundCanvas,
                                            )
                                            .build();
                                        props
                                    },
                                )
                            }
                        },
                    )
                    .class("z-[-10000000] fixed inset-0 bg-gray-900"),
            )
        }
    }
}
