mod components;
pub mod pages;

// use components::navigation::Navigation;
// use pages::blog::Blog;
// use pages::blog_post::BlogPost;
use pages::home_page::HomePage;
use pages::not_found::NotFound;

use bevy_ecs::prelude::Resource;
// use bevy_ecs::query::With;
// use bevy_ecs::system::Query;
// use cinnog::{run_system, FileName};
// use async_bevy_web::{run_system, FileName};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use pages::blog_post::Post;
// use pages::home_page::PersonName;

#[component]
pub fn MyApp() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Meta name="description" content="A static website generated using Leptos and Bevy ECS"/>
        <Stylesheet href="/pkg/cinnog_example.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=HomePage
                        // route_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />

                    <Route
                        path="/404"
                        view=NotFound
                        // static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />

                    // <StaticRoute
                    //     path="/person/*person"
                    //     view=HomePage
                    //     static_params=move || Box::pin(async move { run_system(people_static_params) })
                    // />

                    // <StaticRoute
                    //     path="/blog"
                    //     view=Blog
                    //     static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    // />

                    // <StaticRoute
                    //     path="/blog/*post"
                    //     view=BlogPost
                    //     static_params=move || Box::pin(async move { run_system(blog_static_params) })
                    // />

                </Routes>
            </main>
        </Router>
    }
}

// fn people_static_params(people: Query<&FileName, With<PersonName>>) -> StaticParamsMap {
//     let mut map = StaticParamsMap::default();
//     map.insert(
//         "person".to_string(),
//         people.iter().map(|person| person.0.clone()).collect(),
//     );
//     map
// }

// fn blog_static_params(posts: Query<&FileName, With<Post>>) -> StaticParamsMap {
//     let mut map = StaticParamsMap::default();
//     map.insert(
//         "post".to_string(),
//         posts.iter().map(|post| post.0.clone()).collect(),
//     );
//     map
// }

#[derive(Resource, Clone)]
pub struct SiteName(pub String);
