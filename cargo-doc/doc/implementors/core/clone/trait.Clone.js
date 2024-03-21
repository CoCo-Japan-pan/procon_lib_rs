(function() {var implementors = {
"dynamic_modint":[["impl&lt;MOD: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"dynamic_modint/trait.ModContainer.html\" title=\"trait dynamic_modint::ModContainer\">ModContainer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"dynamic_modint/struct.DynamicModInt.html\" title=\"struct dynamic_modint::DynamicModInt\">DynamicModInt</a>&lt;MOD&gt;"]],
"fenwick_tree":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.AddAssign.html\" title=\"trait core::ops::arith::AddAssign\">AddAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"fenwick_tree/struct.FenwickTree.html\" title=\"struct fenwick_tree::FenwickTree\">FenwickTree</a>&lt;T&gt;"]],
"lazy_seg_tree":[["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"algebra/trait.MapMonoid.html\" title=\"trait algebra::MapMonoid\">MapMonoid</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"lazy_seg_tree/struct.LazySegTree.html\" title=\"struct lazy_seg_tree::LazySegTree\">LazySegTree</a>&lt;F&gt;<span class=\"where fmt-newline\">where\n    F::<a class=\"associatedtype\" href=\"algebra/trait.MapMonoid.html#associatedtype.M\" title=\"type algebra::MapMonoid::M\">M</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    F::<a class=\"associatedtype\" href=\"algebra/trait.MapMonoid.html#associatedtype.F\" title=\"type algebra::MapMonoid::F\">F</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"maxflow":[["impl&lt;Cap: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"maxflow/struct.MaxFlow.html\" title=\"struct maxflow::MaxFlow\">MaxFlow</a>&lt;Cap&gt;"]],
"modint_mersenne":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"modint_mersenne/struct.ModIntMersenne.html\" title=\"struct modint_mersenne::ModIntMersenne\">ModIntMersenne</a>"]],
"rolling_hash":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"rolling_hash/struct.RollingHash.html\" title=\"struct rolling_hash::RollingHash\">RollingHash</a>"]],
"seg_tree":[["impl&lt;M: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"algebra/trait.Monoid.html\" title=\"trait algebra::Monoid\">Monoid</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"seg_tree/struct.SegTree.html\" title=\"struct seg_tree::SegTree\">SegTree</a>&lt;M&gt;<span class=\"where fmt-newline\">where\n    M::<a class=\"associatedtype\" href=\"algebra/trait.Monoid.html#associatedtype.S\" title=\"type algebra::Monoid::S\">S</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"static_modint":[["impl&lt;const MOD: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"static_modint/struct.StaticModInt.html\" title=\"struct static_modint::StaticModInt\">StaticModInt</a>&lt;MOD&gt;"]],
"top2":[["impl&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"top2/struct.Top2Map.html\" title=\"struct top2::Top2Map\">Top2Map</a>&lt;K, V&gt;"]],
"union_find":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"union_find/struct.UnionFind.html\" title=\"struct union_find::UnionFind\">UnionFind</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()