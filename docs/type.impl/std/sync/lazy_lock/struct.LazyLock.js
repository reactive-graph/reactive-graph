(function() {var type_impls = {
"reactive_graph_behaviour_model_impl":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#232\">source</a><a href=\"#impl-Debug-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#233\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-LazyLock%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#223\">source</a><a href=\"#impl-Default-for-LazyLock%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#226\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new lazy value using <code>Default</code> as the initializing function.</p>\n</div></details></div></details>","Default","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#208\">source</a><a href=\"#impl-Deref-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>() -&gt; T,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#217\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;T</a></h4></section></summary><div class=\"docblock\"><p>Dereferences the value.</p>\n<p>This method will block the calling thread if another initialization\nroutine is currently running.</p>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = T</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details></div></details>","Deref","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#195\">source</a><a href=\"#impl-Drop-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#196\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#86\">source</a><a href=\"#impl-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>() -&gt; T,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#90\">source</a><h4 class=\"code-header\">pub const fn <a href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html#tymethod.new\" class=\"fn\">new</a>(f: F) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;</h4></section><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>lazy_cell</code>)</span></div></span></summary><div class=\"docblock\"><p>Creates a new lazy value with the given initializing function.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_inner\" class=\"method\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#123\">source</a><h4 class=\"code-header\">pub fn <a href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html#tymethod.into_inner\" class=\"fn\">into_inner</a>(this: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, F&gt;</h4></section><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>lazy_cell_consume</code>)</span></div></span></summary><div class=\"docblock\"><p>Consumes this <code>LazyLock</code> returning the stored value.</p>\n<p>Returns <code>Ok(value)</code> if <code>Lazy</code> is initialized and <code>Err(f)</code> otherwise.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"attr\">#![feature(lazy_cell)]\n#![feature(lazy_cell_consume)]\n\n</span><span class=\"kw\">use </span>std::sync::LazyLock;\n\n<span class=\"kw\">let </span>hello = <span class=\"string\">\"Hello, World!\"</span>.to_string();\n\n<span class=\"kw\">let </span>lazy = LazyLock::new(|| hello.to_uppercase());\n\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">&amp;*</span>lazy, <span class=\"string\">\"HELLO, WORLD!\"</span>);\n<span class=\"macro\">assert_eq!</span>(LazyLock::into_inner(lazy).ok(), <span class=\"prelude-val\">Some</span>(<span class=\"string\">\"HELLO, WORLD!\"</span>.to_string()));</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.force\" class=\"method\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#159\">source</a><h4 class=\"code-header\">pub fn <a href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html#tymethod.force\" class=\"fn\">force</a>(this: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;T</a></h4></section><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>lazy_cell</code>)</span></div></span></summary><div class=\"docblock\"><p>Forces the evaluation of this lazy value and returns a reference to\nresult. This is equivalent to the <code>Deref</code> impl, but is explicit.</p>\n<p>This method will block the calling thread if another initialization\nroutine is currently running.</p>\n<h5 id=\"examples-1\"><a class=\"doc-anchor\" href=\"#examples-1\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"attr\">#![feature(lazy_cell)]\n\n</span><span class=\"kw\">use </span>std::sync::LazyLock;\n\n<span class=\"kw\">let </span>lazy = LazyLock::new(|| <span class=\"number\">92</span>);\n\n<span class=\"macro\">assert_eq!</span>(LazyLock::force(<span class=\"kw-2\">&amp;</span>lazy), <span class=\"kw-2\">&amp;</span><span class=\"number\">92</span>);\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">&amp;*</span>lazy, <span class=\"kw-2\">&amp;</span><span class=\"number\">92</span>);</code></pre></div>\n</div></details></div></details>",0,"reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<section id=\"impl-RefUnwindSafe-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#250\">source</a><a href=\"#impl-RefUnwindSafe-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,</div></h3></section>","RefUnwindSafe","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<section id=\"impl-Sync-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#246\">source</a><a href=\"#impl-Sync-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</div></h3></section>","Sync","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"],["<section id=\"impl-UnwindSafe-for-LazyLock%3CT,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/sync/lazy_lock.rs.html#252\">source</a><a href=\"#impl-UnwindSafe-for-LazyLock%3CT,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/lazy_lock/struct.LazyLock.html\" title=\"struct std::sync::lazy_lock::LazyLock\">LazyLock</a>&lt;T, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,</div></h3></section>","UnwindSafe","reactive_graph_behaviour_model_impl::entity::function::EntityBehaviourFunctionsStorage","reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()