use async_graphql::Context;
use async_graphql::CustomDirective;
use async_graphql::Directive;
use async_graphql::ResolveFut;
use async_graphql::ServerResult;
use async_graphql::Value;
use async_trait::async_trait;

struct ConcatDirective {
    value: String,
}

#[async_trait]
impl CustomDirective for ConcatDirective {
    async fn resolve_field(&self, _ctx: &Context<'_>, resolve: ResolveFut<'_>) -> ServerResult<Option<Value>> {
        resolve.await.map(|value| {
            value.map(|value| match value {
                Value::String(str) => Value::String(str + &self.value),
                _ => value,
            })
        })
    }
}

#[Directive(location = "Field")]
pub fn concat(value: String) -> impl CustomDirective {
    ConcatDirective { value }
}
