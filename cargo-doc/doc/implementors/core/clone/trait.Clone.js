(function() {var implementors = {
"dynamic_modint":[["impl&lt;MOD: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"dynamic_modint/trait.ModContainer.html\" title=\"trait dynamic_modint::ModContainer\">ModContainer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"dynamic_modint/struct.DynamicModInt.html\" title=\"struct dynamic_modint::DynamicModInt\">DynamicModInt</a>&lt;MOD&gt;"]],
"fenwick_tree":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.AddAssign.html\" title=\"trait core::ops::arith::AddAssign\">AddAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"fenwick_tree/struct.FenwickTree.html\" title=\"struct fenwick_tree::FenwickTree\">FenwickTree</a>&lt;T&gt;"]],
"hld":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"hld/enum.Path.html\" title=\"enum hld::Path\">Path</a>"]],
"matrix":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"algebra/trait.Semiring.html\" title=\"trait algebra::Semiring\">Semiring</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"matrix/struct.Matrix.html\" title=\"struct matrix::Matrix\">Matrix</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"algebra/trait.Semiring.html#associatedtype.Target\" title=\"type algebra::Semiring::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"maxflow":[["impl&lt;Cap: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"maxflow/struct.MaxFlow.html\" title=\"struct maxflow::MaxFlow\">MaxFlow</a>&lt;Cap&gt;"]],
"modint_mersenne":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"modint_mersenne/struct.ModIntMersenne.html\" title=\"struct modint_mersenne::ModIntMersenne\">ModIntMersenne</a>"]],
"rolling_hash":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"rolling_hash/struct.RollingHash.html\" title=\"struct rolling_hash::RollingHash\">RollingHash</a>"]],
"segtree":[["impl&lt;M: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"algebra/trait.Monoid.html\" title=\"trait algebra::Monoid\">Monoid</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"segtree/struct.SegTree.html\" title=\"struct segtree::SegTree\">SegTree</a>&lt;M&gt;<span class=\"where fmt-newline\">where\n    M::<a class=\"associatedtype\" href=\"algebra/trait.Monoid.html#associatedtype.Target\" title=\"type algebra::Monoid::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"sparse_table":[["impl&lt;M: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"algebra/trait.IdempotentMonoid.html\" title=\"trait algebra::IdempotentMonoid\">IdempotentMonoid</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"sparse_table/struct.SparseTable.html\" title=\"struct sparse_table::SparseTable\">SparseTable</a>&lt;M&gt;<span class=\"where fmt-newline\">where\n    M::<a class=\"associatedtype\" href=\"algebra/trait.Monoid.html#associatedtype.Target\" title=\"type algebra::Monoid::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"static_modint":[["impl&lt;const MOD: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"static_modint/struct.StaticModInt.html\" title=\"struct static_modint::StaticModInt\">StaticModInt</a>&lt;MOD&gt;"]],
"top2":[["impl&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"top2/struct.Top2Map.html\" title=\"struct top2::Top2Map\">Top2Map</a>&lt;K, V&gt;"]],
"union_find":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"union_find/struct.UnionFind.html\" title=\"struct union_find::UnionFind\">UnionFind</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()