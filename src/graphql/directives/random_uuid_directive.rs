use async_graphql::Context;
use async_graphql::CustomDirective;
use async_graphql::Directive;
use async_graphql::ResolveFut;
use async_graphql::ServerResult;
use async_graphql::Value;
use uuid::Uuid;

struct RandomUuidDirective {}

#[async_trait::async_trait]
impl CustomDirective for RandomUuidDirective {
    async fn resolve_field(&self, _ctx: &Context<'_>, resolve: ResolveFut<'_>) -> ServerResult<Option<Value>> {
        resolve.await.map(|_| Some(Value::String(Uuid::new_v4().to_string())))
    }
}

#[Directive(location = "field")]
pub fn random_uuid() -> impl CustomDirective {
    RandomUuidDirective {}
}
