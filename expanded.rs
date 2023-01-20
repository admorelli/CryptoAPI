#![feature(prelude_import)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate rocket;
use database::bb8_util::{DbTypes, SurrealdbMan, SurrealdbBackend};
use rocket_prometheus::PrometheusMetrics;
mod api {
    pub mod category_item {
        use crate::security::auth_key::ApiKey;
        pub fn index(_key: ApiKey, category: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Default \'", "\' route"],
                        &[::core::fmt::ArgumentV1::new_display(&category)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct index {}
        /// Rocket code generated proxy static conversion implementations.
        impl index {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                4u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                4u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_category: &str = match __req.routed_segment(0usize)
                        {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"category", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            3u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                4u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                4u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                4u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = index(__rocket__key, __rocket_category);
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "index",
                    method: ::rocket::http::Method::Get,
                    uri: "/<category>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 4u32, 38u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 4u32, 39u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 4u32, 20u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 4u32, 47u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_index_12140799786893003556 as rocket_uri_macro_index;
        pub fn get(_key: ApiKey, _category: &str, key: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Found item with key \'", "\'"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct get {}
        /// Rocket code generated proxy static conversion implementations.
        impl get {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket__category: &str = match __req.routed_segment(0usize)
                        {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"_category", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            8u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(1usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            8u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                9u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = get(
                            __rocket__key,
                            __rocket__category,
                            __rocket_key,
                        );
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "get",
                    method: ::rocket::http::Method::Get,
                    uri: "/<_category>/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 9u32, 37u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 9u32, 38u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 9u32, 48u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 9u32, 49u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 9u32, 18u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 9u32, 57u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_get_6316042927963125952 as rocket_uri_macro_get;
        pub fn add(_key: ApiKey, _category: &str, key: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Added item with key \'", "\'"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct add {}
        /// Rocket code generated proxy static conversion implementations.
        impl add {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket__category: &str = match __req.routed_segment(0usize)
                        {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"_category", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            13u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(1usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            13u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                14u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = add(
                            __rocket__key,
                            __rocket__category,
                            __rocket_key,
                        );
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "add",
                    method: ::rocket::http::Method::Post,
                    uri: "/<_category>/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 14u32, 37u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 14u32, 38u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 14u32, 48u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 14u32, 49u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 14u32, 18u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 14u32, 57u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_add_8606062941496878952 as rocket_uri_macro_add;
        pub fn del(_key: ApiKey, _category: &str, key: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Deleted item with key \'", "\'"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct del {}
        /// Rocket code generated proxy static conversion implementations.
        impl del {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket__category: &str = match __req.routed_segment(0usize)
                        {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"_category", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            18u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(1usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            18u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                19u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = del(
                            __rocket__key,
                            __rocket__category,
                            __rocket_key,
                        );
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "del",
                    method: ::rocket::http::Method::Delete,
                    uri: "/<_category>/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 19u32, 37u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 19u32, 38u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 19u32, 48u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 19u32, 49u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 19u32, 18u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 19u32, 57u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_del_4209672330391686404 as rocket_uri_macro_del;
        pub fn patch(_key: ApiKey, _category: &str, key: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Changed item with key \'", "\'"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct patch {}
        /// Rocket code generated proxy static conversion implementations.
        impl patch {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket__category: &str = match __req.routed_segment(0usize)
                        {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"_category", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            23u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(1usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_item",
                                                            "src\\api\\category_item.rs",
                                                            23u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_item",
                                                "src\\api\\category_item.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = patch(
                            __rocket__key,
                            __rocket__category,
                            __rocket_key,
                        );
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "patch",
                    method: ::rocket::http::Method::Patch,
                    uri: "/<_category>/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 24u32, 39u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 24u32, 40u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 24u32, 50u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_item.rs", 24u32, 51u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 24u32, 20u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_item.rs", 24u32, 59u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_patch_11171066749725501051 as rocket_uri_macro_patch;
    }
    pub mod category_manager {
        use std::collections::BTreeMap;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use bb8::Pool;
        use bb8_surrealdb::SurrealdbConnectionManager;
        use rocket::{response::status::*, State};
        use crate::security::auth_key::ApiKey;
        use crate::models::categoria::schema::model as categoria_base;
        use crate::models::utils::SurrealdbQuery;
        pub async fn index(
            _key: ApiKey,
            pool: &State<Pool<SurrealdbConnectionManager>>,
        ) -> Result<String, String> {
            let localpool = pool.get().await.unwrap();
            let results = categoria_base.select(localpool, None.into()).await;
            match results {
                Ok(r) => {
                    Ok({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&r)],
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                        res
                    })
                }
                Err(e) => Err(e.to_string()),
                _ => Err("something".to_string()),
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct index {}
        /// Rocket code generated proxy static conversion implementations.
        impl index {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                13u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                13u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_pool: &State<Pool<SurrealdbConnectionManager>> = match <&State<
                            Pool<SurrealdbConnectionManager>,
                        > as ::rocket::request::FromRequest>::from_request(__req)
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                13u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                13u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let ___responder = index(__rocket__key, __rocket_pool).await;
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "index",
                    method: ::rocket::http::Method::Get,
                    uri: "/",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 13u32, 26u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 13u32, 40u32),
                                    specialized: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    &State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 13u32, 41u32),
                                    specialized: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 13u32, 47u32),
                                    specialized: Resolve::<
                                        Pool<SurrealdbConnectionManager>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<Pool<SurrealdbConnectionManager>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    type_name: std::any::type_name::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<Pool<SurrealdbConnectionManager>>(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 13u32, 52u32),
                                    specialized: Resolve::<
                                        SurrealdbConnectionManager,
                                    >::SPECIALIZED,
                                    abort: Resolve::<SurrealdbConnectionManager>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<Result<String, String>>(),
                                    type_name: std::any::type_name::<Result<String, String>>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 13u32, 85u32),
                                    specialized: Resolve::<Result<String, String>>::SPECIALIZED,
                                    abort: Resolve::<Result<String, String>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<Result<String, String>>())),
                                    location: ("src\\api\\category_manager.rs", 13u32, 92u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<Result<String, String>>())),
                                    location: ("src\\api\\category_manager.rs", 13u32, 100u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_index_7689865372536986260 as rocket_uri_macro_index;
        pub async fn get(
            api_key: ApiKey,
            pool: &State<Pool<SurrealdbConnectionManager>>,
            key: &str,
        ) -> Result<Accepted<String>, NotFound<String>> {
            let mut hasher = DefaultHasher::new();
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["", ":"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&key),
                            ::core::fmt::ArgumentV1::new_display(
                                &api_key.user.salt.to_string(),
                            ),
                        ],
                    ),
                );
                res
            }
                .hash(&mut hasher);
            let hash = hasher.finish().to_string();
            let localpool = pool.get().await.unwrap();
            let res = categoria_base.select_by_key(localpool, hash.as_str()).await;
            match res {
                Ok(r) => Ok(Accepted(Some("Record found".to_string()))),
                Err(r) => Err(NotFound(r.to_string())),
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct get {}
        /// Rocket code generated proxy static conversion implementations.
        impl get {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket_api_key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_pool: &State<Pool<SurrealdbConnectionManager>> = match <&State<
                            Pool<SurrealdbConnectionManager>,
                        > as ::rocket::request::FromRequest>::from_request(__req)
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(0usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_manager",
                                                            "src\\api\\category_manager.rs",
                                                            23u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                24u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = get(
                                __rocket_api_key,
                                __rocket_pool,
                                __rocket_key,
                            )
                            .await;
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "get",
                    method: ::rocket::http::Method::Get,
                    uri: "/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 24u32, 88u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_manager.rs", 24u32, 89u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 24u32, 27u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 24u32, 41u32),
                                    specialized: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    &State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 24u32, 42u32),
                                    specialized: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 24u32, 48u32),
                                    specialized: Resolve::<
                                        Pool<SurrealdbConnectionManager>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<Pool<SurrealdbConnectionManager>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    type_name: std::any::type_name::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<Pool<SurrealdbConnectionManager>>(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 24u32, 53u32),
                                    specialized: Resolve::<
                                        SurrealdbConnectionManager,
                                    >::SPECIALIZED,
                                    abort: Resolve::<SurrealdbConnectionManager>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        Result<Accepted<String>, NotFound<String>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        Result<Accepted<String>, NotFound<String>>,
                                    >(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 24u32, 97u32),
                                    specialized: Resolve::<
                                        Result<Accepted<String>, NotFound<String>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        Result<Accepted<String>, NotFound<String>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<Accepted<String>>(),
                                    type_name: std::any::type_name::<Accepted<String>>(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    Result<Accepted<String>, NotFound<String>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 24u32, 104u32),
                                    specialized: Resolve::<Accepted<String>>::SPECIALIZED,
                                    abort: Resolve::<Accepted<String>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<Accepted<String>>())),
                                    location: ("src\\api\\category_manager.rs", 24u32, 113u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<NotFound<String>>(),
                                    type_name: std::any::type_name::<NotFound<String>>(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    Result<Accepted<String>, NotFound<String>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 24u32, 122u32),
                                    specialized: Resolve::<NotFound<String>>::SPECIALIZED,
                                    abort: Resolve::<NotFound<String>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<NotFound<String>>())),
                                    location: ("src\\api\\category_manager.rs", 24u32, 131u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_get_3641272775969784559 as rocket_uri_macro_get;
        pub async fn add(
            _key: ApiKey,
            pool: &State<Pool<SurrealdbConnectionManager>>,
            key: &str,
        ) -> Result<Created<String>, Unauthorized<String>> {
            let mut hasher = DefaultHasher::new();
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["", ":"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&key),
                            ::core::fmt::ArgumentV1::new_display(&"teste".to_string()),
                        ],
                    ),
                );
                res
            }
                .hash(&mut hasher);
            Err(Unauthorized(Some("not implemented".to_string())))
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct add {}
        /// Rocket code generated proxy static conversion implementations.
        impl add {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_pool: &State<Pool<SurrealdbConnectionManager>> = match <&State<
                            Pool<SurrealdbConnectionManager>,
                        > as ::rocket::request::FromRequest>::from_request(__req)
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &"& State < Pool < SurrealdbConnectionManager > >",
                                                    ),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(0usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_manager",
                                                            "src\\api\\category_manager.rs",
                                                            38u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                39u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = add(
                                __rocket__key,
                                __rocket_pool,
                                __rocket_key,
                            )
                            .await;
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "add",
                    method: ::rocket::http::Method::Post,
                    uri: "/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 39u32, 85u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_manager.rs", 39u32, 86u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 39u32, 24u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 39u32, 38u32),
                                    specialized: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        &State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    &State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 39u32, 39u32),
                                    specialized: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        State<Pool<SurrealdbConnectionManager>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        Pool<SurrealdbConnectionManager>,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    State<Pool<SurrealdbConnectionManager>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 39u32, 45u32),
                                    specialized: Resolve::<
                                        Pool<SurrealdbConnectionManager>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<Pool<SurrealdbConnectionManager>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    type_name: std::any::type_name::<
                                        SurrealdbConnectionManager,
                                    >(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<Pool<SurrealdbConnectionManager>>(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 39u32, 50u32),
                                    specialized: Resolve::<
                                        SurrealdbConnectionManager,
                                    >::SPECIALIZED,
                                    abort: Resolve::<SurrealdbConnectionManager>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<
                                        Result<Created<String>, Unauthorized<String>>,
                                    >(),
                                    type_name: std::any::type_name::<
                                        Result<Created<String>, Unauthorized<String>>,
                                    >(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 39u32, 94u32),
                                    specialized: Resolve::<
                                        Result<Created<String>, Unauthorized<String>>,
                                    >::SPECIALIZED,
                                    abort: Resolve::<
                                        Result<Created<String>, Unauthorized<String>>,
                                    >::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<Created<String>>(),
                                    type_name: std::any::type_name::<Created<String>>(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    Result<Created<String>, Unauthorized<String>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 39u32, 101u32),
                                    specialized: Resolve::<Created<String>>::SPECIALIZED,
                                    abort: Resolve::<Created<String>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<Created<String>>())),
                                    location: ("src\\api\\category_manager.rs", 39u32, 109u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<Unauthorized<String>>(),
                                    type_name: std::any::type_name::<Unauthorized<String>>(),
                                    parent: None
                                        .or(
                                            Some(
                                                std::any::TypeId::of::<
                                                    Result<Created<String>, Unauthorized<String>>,
                                                >(),
                                            ),
                                        ),
                                    location: ("src\\api\\category_manager.rs", 39u32, 118u32),
                                    specialized: Resolve::<Unauthorized<String>>::SPECIALIZED,
                                    abort: Resolve::<Unauthorized<String>>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None
                                        .or(Some(std::any::TypeId::of::<Unauthorized<String>>())),
                                    location: ("src\\api\\category_manager.rs", 39u32, 131u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_add_16326871590614224323 as rocket_uri_macro_add;
        pub fn del(_key: ApiKey, key: &str) -> String {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["Deleted item with key \'", "\'"],
                        &[::core::fmt::ArgumentV1::new_display(&key)],
                    ),
                );
                res
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct del {}
        /// Rocket code generated proxy static conversion implementations.
        impl del {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                69u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                69u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let __rocket_key: &str = match __req.routed_segment(0usize) {
                            ::std::option::Option::Some(__s) => {
                                match <&str as ::rocket::request::FromParam>::from_param(
                                    __s,
                                ) {
                                    ::std::result::Result::Ok(__v) => __v,
                                    ::std::result::Result::Err(__error) => {
                                        return {
                                            {
                                                let lvl = ::log::Level::Warn;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Parameter guard `", ": ", "` is forwarding: ", "."],
                                                            &match (&"key", &"& str", &__error) {
                                                                args => {
                                                                    [
                                                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                                                        ::core::fmt::ArgumentV1::new_debug(args.2),
                                                                    ]
                                                                }
                                                            },
                                                        ),
                                                        lvl,
                                                        &(
                                                            "_",
                                                            "cripto_api::api::category_manager",
                                                            "src\\api\\category_manager.rs",
                                                            68u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            };
                                            ::rocket::outcome::Outcome::Forward(__data)
                                        };
                                    }
                                }
                            }
                            ::std::option::Option::None => {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Internal invariant broken: dyn param not found."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                69u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Please report this to the Rocket issue tracker."],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                69u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["https://github.com/SergioBenitez/Rocket/issues"],
                                                &[],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::api::category_manager",
                                                "src\\api\\category_manager.rs",
                                                69u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                        };
                        let ___responder = del(__rocket__key, __rocket_key);
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "del",
                    method: ::rocket::http::Method::Delete,
                    uri: "/<key>",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&str>(),
                                    type_name: std::any::type_name::<&str>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 69u32, 31u32),
                                    specialized: Resolve::<&str>::SPECIALIZED,
                                    abort: Resolve::<&str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&str>())),
                                    location: ("src\\api\\category_manager.rs", 69u32, 32u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 69u32, 18u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<String>(),
                                    type_name: std::any::type_name::<String>(),
                                    parent: None,
                                    location: ("src\\api\\category_manager.rs", 69u32, 40u32),
                                    specialized: Resolve::<String>::SPECIALIZED,
                                    abort: Resolve::<String>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_del_3497257772936061893 as rocket_uri_macro_del;
    }
}
mod pages {
    pub mod index {
        use crate::security::auth_key::ApiKey;
        pub fn index() -> &'static str {
            "Est├í api tem a inten├º├úo de armazenar valida├º├Áes criptograficas genericas para as rotas
    Lembrando que a base tem a inten├º├úo de ser fracamente associada tornando praticamente 
    impossivel a extra├º├úo dos dados associados por conta de n├úo sabermos o que est├í sendo validado
    nem de sabermos qual a chave associada a categoria criada
    /category/<key> - ir├í armazenar as categorias e gerencia-las
    /data/<category>/<key> - Crud para armazenar as valida├º├Áes
    
    Mais tarde implementar rotas de storage(seguro)
    /storage/<category>/<key> - Crud para armazenar dados de forma segura
    
    A inten├º├úo de uso ├® para armazenar senhas ou dados importantes sem a associa├º├úo com o uso delas tornando
    um vazamento de dados bem menos perigoso, uma vez que sem a categoria/chave(armazenada somente do lado do cliente)
    os dados s├úo s├│ um hash ou no caso do storage um blob de dados sem significado."
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct index {}
        /// Rocket code generated proxy static conversion implementations.
        impl index {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let ___responder = index();
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "index",
                    method: ::rocket::http::Method::Get,
                    uri: "/",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&'_ str>(),
                                    type_name: std::any::type_name::<&'_ str>(),
                                    parent: None,
                                    location: ("src\\pages\\index.rs", 3u32, 19u32),
                                    specialized: Resolve::<&'_ str>::SPECIALIZED,
                                    abort: Resolve::<&'_ str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&'_ str>())),
                                    location: ("src\\pages\\index.rs", 3u32, 28u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_index_14351133006731969501 as rocket_uri_macro_index;
        pub fn protected(_key: ApiKey) -> &'static str {
            "This is a protected route"
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        /// Rocket code generated proxy structure.
        pub struct protected {}
        /// Rocket code generated proxy static conversion implementations.
        impl protected {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            fn into_info(self) -> ::rocket::route::StaticInfo {
                fn monomorphized_function<'__r>(
                    __req: &'__r ::rocket::request::Request<'_>,
                    __data: ::rocket::data::Data<'__r>,
                ) -> ::rocket::route::BoxFuture<'__r> {
                    ::std::boxed::Box::pin(async move {
                        let __rocket__key: ApiKey = match <ApiKey as ::rocket::request::FromRequest>::from_request(
                                __req,
                            )
                            .await
                        {
                            ::rocket::outcome::Outcome::Success(__v) => __v,
                            ::rocket::outcome::Outcome::Forward(_) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` is forwarding."],
                                                &[::core::fmt::ArgumentV1::new_display(&"ApiKey")],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::pages::index",
                                                "src\\pages\\index.rs",
                                                21u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Forward(__data);
                            }
                            ::rocket::outcome::Outcome::Failure((__c, __e)) => {
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api_log(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Request guard `", "` failed: ", "."],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(&"ApiKey"),
                                                    ::core::fmt::ArgumentV1::new_debug(&__e),
                                                ],
                                            ),
                                            lvl,
                                            &(
                                                "_",
                                                "cripto_api::pages::index",
                                                "src\\pages\\index.rs",
                                                21u32,
                                            ),
                                            ::log::__private_api::Option::None,
                                        );
                                    }
                                };
                                return ::rocket::outcome::Outcome::Failure(__c);
                            }
                        };
                        let ___responder = protected(__rocket__key);
                        ::rocket::route::Outcome::from(__req, ___responder)
                    })
                }
                ::rocket::route::StaticInfo {
                    name: "protected",
                    method: ::rocket::http::Method::Get,
                    uri: "/protected",
                    handler: monomorphized_function,
                    format: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    sentinels: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<ApiKey>(),
                                    type_name: std::any::type_name::<ApiKey>(),
                                    parent: None,
                                    location: ("src\\pages\\index.rs", 21u32, 24u32),
                                    specialized: Resolve::<ApiKey>::SPECIALIZED,
                                    abort: Resolve::<ApiKey>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<&'_ str>(),
                                    type_name: std::any::type_name::<&'_ str>(),
                                    parent: None,
                                    location: ("src\\pages\\index.rs", 21u32, 35u32),
                                    specialized: Resolve::<&'_ str>::SPECIALIZED,
                                    abort: Resolve::<&'_ str>::abort,
                                }
                            },
                            {
                                #[allow(unused_imports)]
                                use ::rocket::sentinel::resolution::{
                                    Resolve, DefaultSentinel as _,
                                };
                                ::rocket::sentinel::Sentry {
                                    type_id: std::any::TypeId::of::<str>(),
                                    type_name: std::any::type_name::<str>(),
                                    parent: None.or(Some(std::any::TypeId::of::<&'_ str>())),
                                    location: ("src\\pages\\index.rs", 21u32, 44u32),
                                    specialized: Resolve::<str>::SPECIALIZED,
                                    abort: Resolve::<str>::abort,
                                }
                            },
                        ]),
                    ),
                }
            }
            #[doc(hidden)]
            pub fn into_route(self) -> ::rocket::Route {
                self.into_info().into()
            }
        }
        #[doc(hidden)]
        pub use rocket_uri_macro_protected_3142873793694383071 as rocket_uri_macro_protected;
    }
}
mod database {
    pub mod bb8_util {
        use bb8::{Pool, ManageConnection};
        use bb8_surrealdb::SurrealdbConnectionManager;
        use surrealdb::Session;
        pub enum SurrealdbBackend<'a> {
            Memory { ns_name: &'a str, db_name: &'a str },
            File { file: &'a str, ns_name: &'a str, db_name: &'a str },
        }
        pub struct SurrealdbMan<'a> {
            pub backend: SurrealdbBackend<'a>,
        }
        pub enum DbTypes<'a> {
            Surrealdb(SurrealdbMan<'a>),
        }
        trait Bb8manager<T>
        where
            T: ManageConnection,
        {
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn get_manager<'async_trait>(
                self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<T, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: Sized,
                Self: 'async_trait;
        }
        impl<'impl0> Bb8manager<SurrealdbConnectionManager> for SurrealdbMan<'impl0> {
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn get_manager<'async_trait>(
                self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<SurrealdbConnectionManager, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<
                            Result<SurrealdbConnectionManager, anyhow::Error>,
                        > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: Result<SurrealdbConnectionManager, anyhow::Error> = {
                        match __self.backend {
                            SurrealdbBackend::Memory { ns_name, db_name } => {
                                Ok(
                                    SurrealdbConnectionManager::memory(
                                            Session::for_kv().with_ns(ns_name).with_db(db_name),
                                        )
                                        .await,
                                )
                            }
                            SurrealdbBackend::File { file, ns_name, db_name } => {
                                Ok(
                                    SurrealdbConnectionManager::file(
                                            file,
                                            Session::for_kv().with_ns(ns_name).with_db(db_name),
                                        )
                                        .await,
                                )
                            }
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        pub async fn configure_bb8pool<T>(
            db_type: DbTypes<'_>,
        ) -> Result<Pool<T>, anyhow::Error>
        where
            T: ManageConnection,
            Pool<T>: From<Pool<SurrealdbConnectionManager>>,
        {
            let manager = match db_type {
                DbTypes::Surrealdb(t) => t.get_manager().await,
            };
            match manager {
                Ok(m) => Ok(Pool::builder().build(m).await.unwrap().into()),
                Err(e) => Err(e),
            }
        }
    }
}
mod security {
    pub mod auth_key {
        use bb8::Pool;
        use bb8_surrealdb::SurrealdbConnectionManager;
        use rocket::http::Status;
        use rocket::request::{Outcome, Request, FromRequest};
        use crate::models::user::User;
        use crate::models::utils::SurrealdbQuery;
        pub struct ApiKey {
            pub user: User,
        }
        pub enum ApiKeyError {
            Missing,
            Invalid,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ApiKeyError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ApiKeyError::Missing => {
                        ::core::fmt::Formatter::write_str(f, "Missing")
                    }
                    ApiKeyError::Invalid => {
                        ::core::fmt::Formatter::write_str(f, "Invalid")
                    }
                }
            }
        }
        impl<'r> FromRequest<'r> for ApiKey {
            type Error = ApiKeyError;
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn from_request<'life0, 'async_trait>(
                req: &'r Request<'life0>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Outcome<Self, Self::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'r: 'async_trait,
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<Outcome<Self, Self::Error>> {
                        return __ret;
                    }
                    let req = req;
                    let __ret: Outcome<Self, Self::Error> = {
                        /// Returns true if `key` is a valid API key string.
                        async fn find_user<'a>(
                            key: &'a str,
                            pool: &'a Pool<SurrealdbConnectionManager>,
                        ) -> Outcome<ApiKey, ApiKeyError> {
                            if key == "valid_api_key" {
                                match pool.get().await {
                                    Ok(conn) => {
                                        match User::select_by_key(conn, key).await {
                                            Ok(u) => Outcome::Success(ApiKey { user: u }),
                                            Err(e) => {
                                                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                                    }
                                }
                            } else {
                                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                            }
                        }
                        match req.headers().get_one("x-api-key") {
                            None => {
                                Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
                            }
                            Some(key) => {
                                find_user(key, req.rocket().state().unwrap()).await
                            }
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
}
mod models {
    pub mod user {
        use std::collections::BTreeMap;
        use anyhow::anyhow;
        use bb8::PooledConnection;
        use bb8_surrealdb::SurrealdbConnectionManager;
        use surreal_simple_querybuilder::prelude::*;
        use surrealdb::sql::Value;
        use super::algorithm::{Alg, schema::Alg as AlgSchema};
        use super::utils::{SurrealdbQuery, CowSegment};
        pub struct User {
            pub id: String,
            ///secret used for authentication
            pub secret: String,
            ///salt to be used by the user child objects(Categories, Hashes and Storages)
            pub salt: String,
            ///set if the user is active
            pub active: bool,
            ///algorithmn that can be used to find a category
            pub alg: Vec<Alg>,
        }
        pub mod schema {
            use super::*;
            use surreal_simple_querybuilder::prelude::*;
            pub struct User<const N: usize> {
                #[serde(skip_serializing)]
                origin: Option<OriginHolder<N>>,
                #[serde(skip_serializing)]
                pub id: SchemaField<N>,
                #[serde(skip_serializing)]
                pub secret: SchemaField<N>,
                #[serde(skip_serializing)]
                pub salt: SchemaField<N>,
                #[serde(skip_serializing)]
                pub active: SchemaField<N>,
                #[serde(skip_serializing)]
                pub alg: SchemaField<N>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<const N: usize> _serde::Serialize for User<N> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "User",
                            false as usize,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl<const N: usize> User<N> {
                const label: &'static str = "User";
                pub const fn new() -> Self {
                    Self {
                        origin: None,
                        id: SchemaField::new("id", SchemaFieldType::Property),
                        secret: SchemaField::new("secret", SchemaFieldType::Property),
                        salt: SchemaField::new("salt", SchemaFieldType::Property),
                        active: SchemaField::new("active", SchemaFieldType::Property),
                        alg: SchemaField::new(
                            "algorithm->AlgSchema",
                            SchemaFieldType::Relation,
                        ),
                    }
                }
                pub fn with_origin(origin: OriginHolder<N>) -> Self {
                    let origin = Some(origin);
                    Self {
                        id: SchemaField::with_origin(
                            "id",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        secret: SchemaField::with_origin(
                            "secret",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        salt: SchemaField::with_origin(
                            "salt",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        active: SchemaField::with_origin(
                            "active",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        alg: SchemaField::with_origin(
                            "algorithm->AlgSchema",
                            SchemaFieldType::Relation,
                            origin.clone(),
                        ),
                        origin,
                    }
                }
                pub fn alg(self) -> AlgSchema<{ N + 2 }> {
                    let origin = self
                        .origin
                        .unwrap_or_else(|| OriginHolder::new([""; N]));
                    let mut new_nested_origin: [&'static str; N + 2] = [""; N + 2];
                    new_nested_origin[..N].clone_from_slice(&origin.segments);
                    new_nested_origin[N] = "->";
                    new_nested_origin[N + 1] = self.alg.identifier;
                    AlgSchema::with_origin(OriginHolder::new(new_nested_origin))
                }
            }
            impl<const N: usize> std::fmt::Display for User<N> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&Self::label)],
                        ),
                    )
                }
            }
            impl<const N: usize> Into<std::borrow::Cow<'static, str>> for User<N> {
                fn into(self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::from(Self::label)
                }
            }
            impl<const N: usize> ToNodeBuilder for User<N> {}
            pub const model: User<0> = User::new();
        }
        impl SurrealdbQuery for User {
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn select<'a, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                filters: Option<BTreeMap<String, Value>>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Vec<Self>, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<
                            Result<Vec<Self>, anyhow::Error>,
                        > {
                        return __ret;
                    }
                    let conn = conn;
                    let filters = filters;
                    let __ret: Result<Vec<Self>, anyhow::Error> = {
                        return Ok(
                            [
                                Self {
                                    id: "teste".into(),
                                    secret: "teste".into(),
                                    salt: "teste".into(),
                                    active: true.into(),
                                    alg: Vec::new().into(),
                                },
                            ]
                                .into(),
                        )
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn select_by_key<'a, 'life0, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                hash: &'life0 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<Result<Self, anyhow::Error>> {
                        return __ret;
                    }
                    let conn = conn;
                    let hash = hash;
                    let __ret: Result<Self, anyhow::Error> = {
                        let filter = [("secret".into(), hash.into())].into();
                        match Self::select(conn, Some(filter)).await {
                            Ok(i) => {
                                if i.len() > 0 {
                                    Ok(i.into_iter().next().unwrap())
                                } else {
                                    Err(
                                        ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                ::core::fmt::Arguments::new_v1(&["Item not found"], &[]),
                                            );
                                            error
                                        }),
                                    )
                                }
                            }
                            Err(e) => Err(e),
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod algorithm {
        use surreal_simple_querybuilder::prelude::*;
        pub struct PlainTextAlgorithmn;
        pub struct ComplementSalting;
        pub enum CryptoAlgorithm {
            Plain(PlainTextAlgorithmn),
        }
        pub enum SaltingStrategy {
            Complement(ComplementSalting),
        }
        pub struct Alg {
            pub id: Option<String>,
            pub algorithm: CryptoAlgorithm,
            pub salting: SaltingStrategy,
        }
        pub mod schema {
            use super::*;
            use surreal_simple_querybuilder::prelude::*;
            pub struct Alg<const N: usize> {
                #[serde(skip_serializing)]
                origin: Option<OriginHolder<N>>,
                pub id: SchemaField<N>,
                pub algorithm: SchemaField<N>,
                pub salting: SchemaField<N>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<const N: usize> _serde::Serialize for Alg<N> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Alg",
                            false as usize + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "algorithm",
                            &self.algorithm,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "salting",
                            &self.salting,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl<const N: usize> Alg<N> {
                const label: &'static str = "Alg";
                pub const fn new() -> Self {
                    Self {
                        origin: None,
                        id: SchemaField::new("id", SchemaFieldType::Property),
                        algorithm: SchemaField::new(
                            "algorithm",
                            SchemaFieldType::Property,
                        ),
                        salting: SchemaField::new("salting", SchemaFieldType::Property),
                    }
                }
                pub fn with_origin(origin: OriginHolder<N>) -> Self {
                    let origin = Some(origin);
                    Self {
                        id: SchemaField::with_origin(
                            "id",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        algorithm: SchemaField::with_origin(
                            "algorithm",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        salting: SchemaField::with_origin(
                            "salting",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        origin,
                    }
                }
            }
            impl<const N: usize> std::fmt::Display for Alg<N> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&Self::label)],
                        ),
                    )
                }
            }
            impl<const N: usize> Into<std::borrow::Cow<'static, str>> for Alg<N> {
                fn into(self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::from(Self::label)
                }
            }
            impl<const N: usize> ToNodeBuilder for Alg<N> {}
            pub const model: Alg<0> = Alg::new();
        }
    }
    pub mod categoria {
        use std::collections::BTreeMap;
        use bb8::PooledConnection;
        use bb8_surrealdb::SurrealdbConnectionManager;
        use surreal_simple_querybuilder::prelude::*;
        use anyhow::anyhow;
        use surrealdb::sql::Value;
        use super::algorithm::{Alg, schema::Alg as AlgSchema};
        use super::user::{User, schema::User as UserSchema};
        use super::utils::SurrealdbQuery;
        pub struct Categoria {
            pub id: String,
            pub is_unsafe: bool,
            pub salt: String,
            pub alg: Vec<Alg>,
            pub owner: User,
        }
        pub mod schema {
            use super::*;
            use surreal_simple_querybuilder::prelude::*;
            pub struct Categoria<const N: usize> {
                #[serde(skip_serializing)]
                origin: Option<OriginHolder<N>>,
                pub id: SchemaField<N>,
                pub is_unsafe: SchemaField<N>,
                pub salt: SchemaField<N>,
                pub alg: SchemaField<N>,
                pub owner: SchemaField<N>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<const N: usize> _serde::Serialize for Categoria<N> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Categoria",
                            false as usize + 1 + 1 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "is_unsafe",
                            &self.is_unsafe,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "salt",
                            &self.salt,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "alg",
                            &self.alg,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "owner",
                            &self.owner,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl<const N: usize> Categoria<N> {
                const label: &'static str = "Categoria";
                pub const fn new() -> Self {
                    Self {
                        origin: None,
                        id: SchemaField::new("id", SchemaFieldType::Property),
                        is_unsafe: SchemaField::new(
                            "is_unsafe",
                            SchemaFieldType::Property,
                        ),
                        salt: SchemaField::new("salt", SchemaFieldType::Property),
                        alg: SchemaField::new(
                            "algorithm->AlgSchema",
                            SchemaFieldType::Relation,
                        ),
                        owner: SchemaField::new("owner", SchemaFieldType::Property),
                    }
                }
                pub fn with_origin(origin: OriginHolder<N>) -> Self {
                    let origin = Some(origin);
                    Self {
                        id: SchemaField::with_origin(
                            "id",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        is_unsafe: SchemaField::with_origin(
                            "is_unsafe",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        salt: SchemaField::with_origin(
                            "salt",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        alg: SchemaField::with_origin(
                            "algorithm->AlgSchema",
                            SchemaFieldType::Relation,
                            origin.clone(),
                        ),
                        owner: SchemaField::with_origin(
                            "owner",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        origin,
                    }
                }
                pub fn alg(self) -> AlgSchema<{ N + 2 }> {
                    let origin = self
                        .origin
                        .unwrap_or_else(|| OriginHolder::new([""; N]));
                    let mut new_nested_origin: [&'static str; N + 2] = [""; N + 2];
                    new_nested_origin[..N].clone_from_slice(&origin.segments);
                    new_nested_origin[N] = "->";
                    new_nested_origin[N + 1] = self.alg.identifier;
                    AlgSchema::with_origin(OriginHolder::new(new_nested_origin))
                }
                pub fn owner(self) -> UserSchema<{ N + 2 }> {
                    let origin = self
                        .origin
                        .unwrap_or_else(|| OriginHolder::new([""; N]));
                    let mut new_origin: [&'static str; N + 2] = [""; N + 2];
                    new_origin[..N].clone_from_slice(&origin.segments);
                    if (N > 0 && new_origin[N - 1] != ".") {
                        new_origin[N] = ".";
                    }
                    new_origin[N + 1] = self.owner.identifier;
                    UserSchema::with_origin(OriginHolder::new(new_origin))
                }
            }
            impl<const N: usize> std::fmt::Display for Categoria<N> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&Self::label)],
                        ),
                    )
                }
            }
            impl<const N: usize> Into<std::borrow::Cow<'static, str>> for Categoria<N> {
                fn into(self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::from(Self::label)
                }
            }
            impl<const N: usize> ToNodeBuilder for Categoria<N> {}
            pub const model: Categoria<0> = Categoria::new();
        }
        impl SurrealdbQuery for Categoria {
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn select<'a, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                filters: Option<BTreeMap<String, Value>>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Vec<Self>, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<
                            Result<Vec<Self>, anyhow::Error>,
                        > {
                        return __ret;
                    }
                    let conn = conn;
                    let filters = filters;
                    let __ret: Result<Vec<Self>, anyhow::Error> = {
                        ::core::panicking::panic("not yet implemented")
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
            #[allow(
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn select_by_key<'a, 'life0, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                hash: &'life0 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<Result<Self, anyhow::Error>> {
                        return __ret;
                    }
                    let conn = conn;
                    let hash = hash;
                    let __ret: Result<Self, anyhow::Error> = {
                        let filter = [("id".into(), hash.into())].into();
                        match Self::select(conn, Some(filter)).await {
                            Ok(i) => {
                                if i.len() > 0 {
                                    Ok(i.into_iter().next().unwrap())
                                } else {
                                    Err(
                                        ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                ::core::fmt::Arguments::new_v1(&["Item not found"], &[]),
                                            );
                                            error
                                        }),
                                    )
                                }
                            }
                            Err(e) => Err(e),
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod hash {
        use surreal_simple_querybuilder::prelude::*;
        use super::categoria::schema::Categoria;
        pub mod schema {
            use super::*;
            use surreal_simple_querybuilder::prelude::*;
            pub struct Hash<const N: usize> {
                #[serde(skip_serializing)]
                origin: Option<OriginHolder<N>>,
                pub id: SchemaField<N>,
                pub is_unsafe: SchemaField<N>,
                pub salt: SchemaField<N>,
                pub owner: SchemaField<N>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<const N: usize> _serde::Serialize for Hash<N> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Hash",
                            false as usize + 1 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "is_unsafe",
                            &self.is_unsafe,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "salt",
                            &self.salt,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "owner",
                            &self.owner,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl<const N: usize> Hash<N> {
                const label: &'static str = "Hash";
                pub const fn new() -> Self {
                    Self {
                        origin: None,
                        id: SchemaField::new("id", SchemaFieldType::Property),
                        is_unsafe: SchemaField::new(
                            "is_unsafe",
                            SchemaFieldType::Property,
                        ),
                        salt: SchemaField::new("salt", SchemaFieldType::Property),
                        owner: SchemaField::new("owner", SchemaFieldType::Property),
                    }
                }
                pub fn with_origin(origin: OriginHolder<N>) -> Self {
                    let origin = Some(origin);
                    Self {
                        id: SchemaField::with_origin(
                            "id",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        is_unsafe: SchemaField::with_origin(
                            "is_unsafe",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        salt: SchemaField::with_origin(
                            "salt",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        owner: SchemaField::with_origin(
                            "owner",
                            SchemaFieldType::Property,
                            origin.clone(),
                        ),
                        origin,
                    }
                }
                pub fn owner(self) -> Categoria<{ N + 2 }> {
                    let origin = self
                        .origin
                        .unwrap_or_else(|| OriginHolder::new([""; N]));
                    let mut new_origin: [&'static str; N + 2] = [""; N + 2];
                    new_origin[..N].clone_from_slice(&origin.segments);
                    if (N > 0 && new_origin[N - 1] != ".") {
                        new_origin[N] = ".";
                    }
                    new_origin[N + 1] = self.owner.identifier;
                    Categoria::with_origin(OriginHolder::new(new_origin))
                }
            }
            impl<const N: usize> std::fmt::Display for Hash<N> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&Self::label)],
                        ),
                    )
                }
            }
            impl<const N: usize> Into<std::borrow::Cow<'static, str>> for Hash<N> {
                fn into(self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::from(Self::label)
                }
            }
            impl<const N: usize> ToNodeBuilder for Hash<N> {}
            pub const model: Hash<0> = Hash::new();
        }
    }
    pub mod utils {
        use std::collections::BTreeMap;
        use std::borrow::Cow;
        use bb8::PooledConnection;
        use bb8_surrealdb::SurrealdbConnectionManager;
        use surrealdb::{
            sql::{Value, Object},
            Response,
        };
        pub type CowSegment<'a> = Cow<'a, str>;
        pub trait SurrealdbQuery {
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn select<'a, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                filters: Option<BTreeMap<String, Value>>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Vec<Self>, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: Sized,
                'a: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn select_by_key<'a, 'life0, 'async_trait>(
                conn: PooledConnection<'a, SurrealdbConnectionManager>,
                hash: &'life0 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self, anyhow::Error>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: Sized,
                'a: 'async_trait,
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub fn build_filter(filters: &Option<BTreeMap<String, Value>>) -> String {
            if let Some(filtered) = filters {
                filtered
                    .into_iter()
                    .map(|filter| {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", " = "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&filter.0),
                                    ::core::fmt::ArgumentV1::new_display(&filter.1),
                                ],
                            ),
                        );
                        res
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            } else {
                "".to_string()
            }
        }
        pub fn get_value(
            ress: Vec<Response>,
        ) -> Result<
            impl Iterator<Item = Result<Object, surrealdb::Error>>,
            surrealdb::Error,
        > {
            let result = ress.into_iter().next().map(|rp| rp.result).transpose()?;
            match result {
                Some(Value::Array(r)) => {
                    let it = r
                        .into_iter()
                        .map(|v| match v {
                            Value::Object(object) => Ok(object),
                            _ => Err(surrealdb::Error::Ignore),
                        });
                    Ok(it)
                }
                _ => Err(surrealdb::Error::Ignore),
            }
        }
    }
}
#[allow(dead_code)]
async fn rocket() -> ::rocket::Rocket<::rocket::Build> {
    let prometheus = PrometheusMetrics::new();
    let tp = DbTypes::Surrealdb(SurrealdbMan {
        backend: SurrealdbBackend::File {
            file: "tempdb.db",
            ns_name: "Teste",
            db_name: "Teste",
        },
    });
    let pool = database::bb8_util::configure_bb8pool(tp).await.unwrap();
    rocket::build()
        .manage(pool)
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount(
            "/",
            {
                let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let ___struct = pages::index::index {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = pages::index::protected {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                    ]),
                );
                ___vec
            },
        )
        .mount(
            "/api/category",
            {
                let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let ___struct = api::category_manager::index {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_manager::get {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_manager::add {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_manager::del {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                    ]),
                );
                ___vec
            },
        )
        .mount(
            "/api/data",
            {
                let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let ___struct = api::category_item::index {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_item::get {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_item::add {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_item::del {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                        {
                            let ___struct = api::category_item::patch {};
                            let ___item: ::rocket::Route = ___struct.into_route();
                            ___item
                        },
                    ]),
                );
                ___vec
            },
        )
}
fn main() {
    ::rocket::async_main(async move {
        let _res = {
            let ___rocket: ::rocket::Rocket<::rocket::Build> = {
                let prometheus = PrometheusMetrics::new();
                let tp = DbTypes::Surrealdb(SurrealdbMan {
                    backend: SurrealdbBackend::File {
                        file: "tempdb.db",
                        ns_name: "Teste",
                        db_name: "Teste",
                    },
                });
                let pool = database::bb8_util::configure_bb8pool(tp).await.unwrap();
                rocket::build()
                    .manage(pool)
                    .attach(prometheus.clone())
                    .mount("/metrics", prometheus)
                    .mount(
                        "/",
                        {
                            let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    {
                                        let ___struct = pages::index::index {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = pages::index::protected {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                ]),
                            );
                            ___vec
                        },
                    )
                    .mount(
                        "/api/category",
                        {
                            let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    {
                                        let ___struct = api::category_manager::index {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_manager::get {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_manager::add {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_manager::del {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                ]),
                            );
                            ___vec
                        },
                    )
                    .mount(
                        "/api/data",
                        {
                            let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    {
                                        let ___struct = api::category_item::index {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_item::get {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_item::add {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_item::del {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                    {
                                        let ___struct = api::category_item::patch {};
                                        let ___item: ::rocket::Route = ___struct.into_route();
                                        ___item
                                    },
                                ]),
                            );
                            ___vec
                        },
                    )
            };
            let ___rocket: ::rocket::Rocket<::rocket::Build> = ___rocket;
            ___rocket
        }
            .launch()
            .await;
    })
}
