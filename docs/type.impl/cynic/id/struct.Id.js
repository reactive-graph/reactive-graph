(function() {
    var type_impls = Object.fromEntries([["reactive_graph_client",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Id\" class=\"impl\"><a href=\"#impl-Clone-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Id</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Id\" class=\"impl\"><a href=\"#impl-Debug-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-Id\" class=\"impl\"><a href=\"#impl-Deserialize%3C'de%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(\n    __deserializer: __D,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Id, &lt;__D as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CT%3E-for-Id\" class=\"impl\"><a href=\"#impl-From%3CT%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for Id<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: T) -&gt; Id</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<T>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-Id\" class=\"impl\"><a href=\"#impl-Hash-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __H</a>)<div class=\"where\">where\n    __H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237\">Source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Id\" class=\"impl\"><a href=\"#impl-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(s: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;) -&gt; Id</h4></section></summary><div class=\"docblock\"><p>Constructs an <code>ID</code> from a <code>String</code>, <code>&amp;str</code> or similar</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code>cynic::Id::new(<span class=\"string\">\"123\"</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.inner\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">inner</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class=\"docblock\"><p>Returns a reference to the value of this <code>Id</code></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_inner\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">into_inner</a>(self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section></summary><div class=\"docblock\"><p>Converts this <code>Id</code> into its inner value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_ref\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">from_ref</a>(s: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>) -&gt; &amp;Id</h4></section></summary><div class=\"docblock\"><p>Converts a reference to a String to a reference to an Id</p>\n<p>To be used when you can access an <code>&amp;String</code> which you want to assume is\nan <code>Id</code> for use in Cynic structures without reallocating</p>\n<p>If you don’t have a <code>String</code> at hand but only an <code>&amp;str</code>, you should know\nthat these can be used directly in <code>InputObject</code>s as well when the\ntarget GraphQL type is an <code>Id</code>.</p>\n</div></details></div></details>",0,"reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IsScalar%3CId%3E-for-Id\" class=\"impl\"><a href=\"#impl-IsScalar%3CId%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl IsScalar&lt;Id&gt; for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.SchemaType\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.SchemaType\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">SchemaType</a> = Id</h4></section></summary><div class='docblock'>The schema marker type this scalar represents.</div></details></div></details>","IsScalar<Id>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-NamedType-for-Id\" class=\"impl\"><a href=\"#impl-NamedType-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl NamedType for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.NAME\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.NAME\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">NAME</a>: &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a> = &quot;ID&quot;</h4></section></summary><div class='docblock'>The name of this type</div></details></div></details>","NamedType","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Id\" class=\"impl\"><a href=\"#impl-PartialEq-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Id) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RefCast-for-Id\" class=\"impl\"><a href=\"#impl-RefCast-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html\" title=\"trait ref_cast::RefCast\">RefCast</a> for Id</h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.From\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.From\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html#associatedtype.From\" class=\"associatedtype\">From</a> = <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section><section id=\"method.ref_cast\" class=\"method trait-impl\"><a href=\"#method.ref_cast\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html#tymethod.ref_cast\" class=\"fn\">ref_cast</a>(_from: &amp;&lt;Id as <a class=\"trait\" href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html\" title=\"trait ref_cast::RefCast\">RefCast</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html#associatedtype.From\" title=\"type ref_cast::RefCast::From\">From</a>) -&gt; &amp;Id</h4></section><section id=\"method.ref_cast_mut\" class=\"method trait-impl\"><a href=\"#method.ref_cast_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html#tymethod.ref_cast_mut\" class=\"fn\">ref_cast_mut</a>(_from: &amp;mut &lt;Id as <a class=\"trait\" href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html\" title=\"trait ref_cast::RefCast\">RefCast</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/ref-cast/1.0.24/ref_cast/trait.RefCast.html#associatedtype.From\" title=\"type ref_cast::RefCast::From\">From</a>) -&gt; &amp;mut Id</h4></section></div></details>","RefCast","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-Id\" class=\"impl\"><a href=\"#impl-Serialize-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for Id</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(\n    &amp;self,\n    __serializer: __S,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, &lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Variable-for-Id\" class=\"impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_graphql/mod.rs.html#8\">Source</a><a href=\"#impl-Variable-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"reactive_graph_client/schema_graphql/schema/variable/trait.Variable.html\" title=\"trait reactive_graph_client::schema_graphql::schema::variable::Variable\">Variable</a> for Id</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.TYPE\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_graphql/mod.rs.html#8\">Source</a><a href=\"#associatedconstant.TYPE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"reactive_graph_client/schema_graphql/schema/variable/trait.Variable.html#associatedconstant.TYPE\" class=\"constant\">TYPE</a>: VariableType</h4></section></div></details>","Variable","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Variable-for-Id\" class=\"impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_plugin/mod.rs.html#7\">Source</a><a href=\"#impl-Variable-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"reactive_graph_client/schema_plugin/schema/variable/trait.Variable.html\" title=\"trait reactive_graph_client::schema_plugin::schema::variable::Variable\">Variable</a> for Id</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.TYPE\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_plugin/mod.rs.html#7\">Source</a><a href=\"#associatedconstant.TYPE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"reactive_graph_client/schema_plugin/schema/variable/trait.Variable.html#associatedconstant.TYPE\" class=\"constant\">TYPE</a>: VariableType</h4></section></div></details>","Variable","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Variable-for-Id\" class=\"impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_runtime/mod.rs.html#11\">Source</a><a href=\"#impl-Variable-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"reactive_graph_client/schema_runtime/schema/variable/trait.Variable.html\" title=\"trait reactive_graph_client::schema_runtime::schema::variable::Variable\">Variable</a> for Id</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.TYPE\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/reactive_graph_client/schema_runtime/mod.rs.html#11\">Source</a><a href=\"#associatedconstant.TYPE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"reactive_graph_client/schema_runtime/schema/variable/trait.Variable.html#associatedconstant.TYPE\" class=\"constant\">TYPE</a>: VariableType</h4></section></div></details>","Variable","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3CId%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3CId%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;Id&gt; for Id</h3></section>","CoercesTo<Id>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3COption%3CId%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3COption%3CId%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Id&gt;&gt; for Id</h3></section>","CoercesTo<Option<Id>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3COption%3COption%3CId%3E%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3COption%3COption%3CId%3E%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Id&gt;&gt;&gt; for Id</h3></section>","CoercesTo<Option<Option<Id>>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3COption%3CVec%3CId%3E%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3COption%3CVec%3CId%3E%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Id&gt;&gt;&gt; for Id</h3></section>","CoercesTo<Option<Vec<Id>>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3COption%3CVec%3COption%3CId%3E%3E%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3COption%3CVec%3COption%3CId%3E%3E%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Id&gt;&gt;&gt;&gt; for Id</h3></section>","CoercesTo<Option<Vec<Option<Id>>>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3CVec%3CId%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3CVec%3CId%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Id&gt;&gt; for Id</h3></section>","CoercesTo<Vec<Id>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-CoercesTo%3CVec%3CVec%3CId%3E%3E%3E-for-Id\" class=\"impl\"><a href=\"#impl-CoercesTo%3CVec%3CVec%3CId%3E%3E%3E-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl CoercesTo&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Id&gt;&gt;&gt; for Id</h3></section>","CoercesTo<Vec<Vec<Id>>>","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-Eq-for-Id\" class=\"impl\"><a href=\"#impl-Eq-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Id</h3></section>","Eq","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"],["<section id=\"impl-StructuralPartialEq-for-Id\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-Id\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for Id</h3></section>","StructuralPartialEq","reactive_graph_client::schema_graphql::schema::ID","reactive_graph_client::schema_plugin::schema::ID","reactive_graph_client::schema_runtime::schema::ID"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[29868]}