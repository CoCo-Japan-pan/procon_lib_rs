var searchIndex = JSON.parse('{\
"algebra":{"doc":"<code>Algrebra</code>…","t":"IIIIQIIQIIQQQKKLKLKLKLKKLKKK","n":["Commutative","Group","IdempotentMonoid","Map","Map","MapMonoid","Monoid","Monoid","NonCommutative","Semiring","Target","Target","Target","add_assign","binary_operation","binary_operation","composition","composition","id_element","id_element","id_map","id_map","inverse","mapping","mapping","mul","one","zero"],"q":[[0,"algebra"]],"d":["可換  ","群 モノイドに加えて、逆元を持つ  ","冪等なモノイド つまり x = x op x …","作用 作用自体もモノイドであることを要求 …","作用素のモノイド","自己準同型性を要求 …","モノイド","作用の対象のモノイド","非可換","半環 加算は可換モノイド 乗算はモノイド …","作用の対象","モノイドの要素","","","二項演算","二項演算","作用の合成(selfが先、rhsが後)","作用の合成(fが先、gが後)","単位元","単位元","恒等写像","恒等写像","","作用の適用","作用の適用","","",""],"i":[0,0,0,0,1,0,0,1,0,0,2,3,4,4,3,1,2,1,3,1,2,1,5,2,1,4,4,4],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]]],"c":[],"p":[[8,"MapMonoid"],[8,"Map"],[8,"Monoid"],[8,"Semiring"],[8,"Group"]]},\
"binom":{"doc":"","t":"DLLLLLLLLL","n":["Binom","borrow","borrow_mut","cmp","from","into","new","try_from","try_into","type_id"],"q":[[0,"binom"]],"d":["","","","nCkの計算(n&lt;kの場合は0とする)","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","",""],"i":[0,2,2,2,2,2,2,2,2,2],"f":[0,[[]],[[]],[[[2,[1]],3,3],1],[[]],[[]],[3,[[2,[1]]]],[[],4],[[],4],[[],5]],"c":[],"p":[[8,"ModInt"],[3,"Binom"],[15,"usize"],[4,"Result"],[3,"TypeId"]]},\
"dual_segtree":{"doc":"作用素を通常のセグメント木のように持つ …","t":"DLLLLLLLLLLLLL","n":["DualSegTree","apply_commutative","apply_non_commutative","borrow","borrow_mut","fmt","from","get_composition","get_mapped","into","new","try_from","try_into","type_id"],"q":[[0,"dual_segtree"]],"d":["作用を区間適用, …","区間に可換な作用を適用する …","区間に非可換な作用を適用する …","","","","Returns the argument unchanged.","一点取得(その点への作用の合成を返す)","…","Calls <code>U::from(self)</code>.","","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[0,[[[3,[[0,[1,2]]]],[5,[4]],[0,[1,2]]]],[[[3,[[0,[1,6]]]],[5,[4]],[0,[1,6]]]],[[]],[[]],[[[3,[[0,[7,1]]]],8],9],[[]],[[[3,[1]],4],1],[[[3,[1]],4]],[[]],[4,[[3,[1]]]],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"Map"],[8,"Commutative"],[3,"DualSegTree"],[15,"usize"],[8,"RangeBounds"],[8,"NonCommutative"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[4,"Result"],[3,"TypeId"]]},\
"dynamic_modint":{"doc":"動的に決定するModを持つModInt …","t":"DILLLLLLLLOLLLLLLLLLLLLLLLLLLLLKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["DynamicModInt","ModContainer","add","add_assign","add_assign","borrow","borrow_mut","clone","clone_into","default","define_modint","div","div_assign","div_assign","eq","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from_str","get_static_modulus","hash","into","inv","modulus","modulus","modulus","mul","mul_assign","mul_assign","neg","new","new","one","pow","product","raw","raw","set_modulus","set_modulus","sub","sub_assign","sub_assign","sum","to_owned","to_string","try_from","try_into","type_id","value","value","zero"],"q":[[0,"dynamic_modint"]],"d":["","","","","","","","","","","ModContainerを定義するマクロ …","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,0,[[[2,[1]]],[[2,[1]]]],[[[2,[1]],[2,[1]]]],[[[2,[1]],3]],[[]],[[]],[[[2,[[0,[4,1]]]]],[[2,[[0,[4,1]]]]]],[[]],[[],[[2,[[0,[5,1]]]]]],0,[[[2,[1]]],[[2,[1]]]],[[[2,[1]],3]],[[[2,[1]],[2,[1]]]],[[[2,[[0,[6,1]]]],[2,[[0,[6,1]]]]],7],[[[2,[1]],8],9],[[[2,[[0,[10,1]]]],8],9],[11,[[2,[1]]]],[12,[[2,[1]]]],[13,[[2,[1]]]],[14,[[2,[1]]]],[15,[[2,[1]]]],[16,[[2,[1]]]],[17,[[2,[1]]]],[18,[[2,[1]]]],[19,[[2,[1]]]],[20,[[2,[1]]]],[21,[[2,[1]]]],[[]],[22,[[2,[1]]]],[23,[[25,[[2,[1]],24]]]],[[],[[26,[22]]]],[[[2,[[0,[27,1]]]],28]],[[]],[[[2,[1]]],[[2,[1]]]],[[],22],[[],22],[[],22],[[[2,[1]]],[[2,[1]]]],[[[2,[1]],3]],[[[2,[1]],[2,[1]]]],[[[2,[1]]],[[2,[1]]]],[3,[[2,[1]]]],[3,[[2,[1]]]],[[],[[2,[1]]]],[[[2,[1]],16],[[2,[1]]]],[29,[[2,[1]]]],[22,[[2,[1]]]],[22,[[2,[1]]]],[22],[22],[[[2,[1]]],[[2,[1]]]],[[[2,[1]],[2,[1]]]],[[[2,[1]],3]],[29,[[2,[1]]]],[[]],[[],30],[[],25],[[],25],[[],31],[[[2,[1]]],22],[[[2,[1]]],22],[[],[[2,[1]]]]],"c":[],"p":[[8,"ModContainer"],[3,"DynamicModInt"],[8,"RemEuclidU32"],[8,"Clone"],[8,"Default"],[8,"PartialEq"],[15,"bool"],[3,"Formatter"],[6,"Result"],[8,"Debug"],[15,"u8"],[15,"i128"],[15,"i16"],[15,"usize"],[15,"isize"],[15,"u64"],[15,"u128"],[15,"u16"],[15,"i32"],[15,"i64"],[15,"i8"],[15,"u32"],[15,"str"],[3,"ParseIntError"],[4,"Result"],[3,"OnceLock"],[8,"Hash"],[8,"Hasher"],[8,"Iterator"],[3,"String"],[3,"TypeId"]]},\
"fenwick_tree":{"doc":"","t":"DLLLLLLLLLLLLLLLLL","n":["FenwickTree","add","borrow","borrow_mut","clone","clone_into","eq","fmt","from","into","lower_bound","new","set","sum","to_owned","try_from","try_into","type_id"],"q":[[0,"fenwick_tree"]],"d":["","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","<code>a[0] + ... a[x] &gt;= w</code> となる最小の x を返す","","","","","","",""],"i":[0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4],"f":[0,[[[4,[[0,[1,2,3]]]],5,[0,[1,2,3]]]],[[]],[[]],[[[4,[[0,[1,1,2,3]]]]],[[4,[[0,[1,1,2,3]]]]]],[[]],[[[4,[[0,[6,1,2,3]]]],[4,[[0,[6,1,2,3]]]]],7],[[[4,[[0,[8,1,2,3]]]],9],10],[[]],[[]],[[[4,[[0,[11,12]],[0,[1,2,3]]]],[0,[11,12]],[0,[1,2,3]]],5],[[5,[0,[1,2,3]]],[[4,[[0,[1,2,3]]]]]],[[[4,[[0,[1,2,3]]]],5,[0,[1,2,3]]]],[[[4,[[0,[1,2,3]]]],[13,[5]]],[[0,[1,2,3]]]],[[]],[[],14],[[],14],[[],15]],"c":[],"p":[[8,"Clone"],[8,"AddAssign"],[8,"Sub"],[3,"FenwickTree"],[15,"usize"],[8,"PartialEq"],[15,"bool"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[8,"PartialOrd"],[8,"SubAssign"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"hld":{"doc":"HCPCの資料 …","t":"NNDELLLLLLMLLLLMLLLLMLMLMLLLLLLL","n":["Ascending","Descending","HLD","Path","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","depth","fmt","fmt","from","from","hld_in","into","into","lca","new","parent","path","sorted_graph","subtree","subtree_size","to_owned","try_from","try_from","try_into","try_into","type_id","type_id"],"q":[[0,"hld"]],"d":["hldの上では右から左に進む","hldの上では左から右に進む","","","","","","","","","各頂点の深さ(根は0とする)","","","Returns the argument unchanged.","Returns the argument unchanged.","heavy pathを並べた配列における各頂点のindex …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","各頂点の親(根にはusize::MAXを入れる)","uからvへのパスを列挙する(これらはheavy …","…","…","…","","","","","","",""],"i":[1,1,0,0,2,1,2,1,1,1,2,2,1,2,1,2,2,1,2,2,2,2,2,2,2,1,2,1,2,1,2,1],"f":[0,0,0,0,[[]],[[]],[[]],[[]],[1,1],[[]],0,[[2,3],4],[[1,3],4],[[]],[[]],0,[[]],[[]],[[2,5,5],5],[[[6,[[6,[5]]]],5],2],0,[[2,5,5,7],[[6,[1]]]],0,[[2,5,7]],0,[[]],[[],8],[[],8],[[],8],[[],8],[[],9],[[],9]],"c":[],"p":[[4,"Path"],[3,"HLD"],[3,"Formatter"],[6,"Result"],[15,"usize"],[3,"Vec"],[15,"bool"],[4,"Result"],[3,"TypeId"]]},\
"internal_type_traits":{"doc":"","t":"IIIIIKKKK","n":["BoundedAbove","BoundedBelow","Integral","One","Zero","max_value","min_value","one","zero"],"q":[[0,"internal_type_traits"]],"d":["","","数値型を使いたいときのトレイト …","Class that has multiplicative identity element","Class that has additive identity element","","","The multiplicative identity element","The additive identity element"],"i":[0,0,0,0,0,1,2,3,4],"f":[0,0,0,0,0,[[]],[[]],[[]],[[]]],"c":[],"p":[[8,"BoundedAbove"],[8,"BoundedBelow"],[8,"One"],[8,"Zero"]]},\
"lazy_segtree":{"doc":"Reference …","t":"DLLLLLLLLLLLLLLLLLLL","n":["LazySegTree","all_prod","apply","apply_range_commutative","apply_range_non_commutative","borrow","borrow_mut","fmt","from","from","get","into","max_right","min_left","new","prod","set","try_from","try_into","type_id"],"q":[[0,"lazy_segtree"]],"d":["","","","可換な作用の区間適用","非可換な作用の区間適用","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,[[[2,[1]]]],[[[2,[1]],3]],[[[2,[1]],[4,[3]]]],[[[2,[1]],[4,[3]]]],[[]],[[]],[[[2,[[0,[5,1]]]],6],7],[[]],[8,[[2,[1]]]],[[[2,[1]],3]],[[]],[[[2,[1]],3,9],3],[[[2,[1]],3,9],3],[3,[[2,[1]]]],[[[2,[1]],[4,[3]]]],[[[2,[1]],3]],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"MapMonoid"],[3,"LazySegTree"],[15,"usize"],[8,"RangeBounds"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[8,"Fn"],[4,"Result"],[3,"TypeId"]]},\
"matrix":{"doc":"行列ライブラリ 行列積は普通に<code>O(d^3)</code>…","t":"DDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["Matrix","UsualSemiring","add","add_assign","add_assign","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","eq","eq","fmt","fmt","from","from","from","from","get","get_mut","into","into","mul","mul","mul_assign","new","one","pow","sub","sub_assign","to_owned","to_owned","transpose","try_from","try_from","try_into","try_into","type_id","type_id","unit","zero"],"q":[[0,"matrix"]],"d":["","通常の足し算、掛け算による半環","","","","","","","","","","","","","","","","Returns the argument unchanged.","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","",""],"i":[0,0,2,10,2,10,2,10,2,10,2,10,2,10,2,10,2,10,2,2,2,2,2,10,2,10,2,2,2,10,2,2,2,10,2,2,10,2,10,2,10,2,2,10],"f":[0,0,[[[2,[1]],[2,[1]]],[[2,[1]]]],[[]],[[[2,[1]],[2,[1]]]],[[]],[[]],[[]],[[]],[[[10,[[0,[3,4,3,5,6,7,8,9]]]]],[[10,[[0,[3,4,3,5,6,7,8,9]]]]]],[[[2,[[0,[3,1]]]]],[[2,[[0,[3,1]]]]]],[[]],[[]],[[[10,[[0,[11,4,3,5,6,7,8,9]]]],[10,[[0,[11,4,3,5,6,7,8,9]]]]],12],[[[2,[[0,[11,1]]]],[2,[[0,[11,1]]]]],12],[[[10,[[0,[4,4,3,5,6,7,8,9]]]],13],14],[[[2,[[0,[4,1]]]],13],14],[[]],[[[15,[15]]],[[2,[1]]]],[[]],[[],[[2,[1]]]],[[[2,[1]],16,16]],[[[2,[1]],16,16]],[[]],[[]],[[]],[[[2,[1]],[2,[1]]],[[2,[1]]]],[[[2,[1]],[2,[1]]]],[[16,16],[[2,[1]]]],[[]],[[[2,[1]],17],[[2,[1]]]],[[[2,[1]],[2,[1]]],[[2,[1]]]],[[[2,[1]],[2,[1]]]],[[]],[[]],[[[2,[1]]],[[2,[1]]]],[[],18],[[],18],[[],18],[[],18],[[],19],[[],19],[16,[[2,[1]]]],[[]]],"c":[],"p":[[8,"Semiring"],[3,"Matrix"],[8,"Clone"],[8,"Debug"],[8,"Eq"],[8,"Zero"],[8,"One"],[8,"AddAssign"],[8,"Mul"],[3,"UsualSemiring"],[8,"PartialEq"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[15,"usize"],[15,"u64"],[4,"Result"],[3,"TypeId"]]},\
"maxflow":{"doc":"Reference","t":"DDLLLLLMLLLLLLLLMLLLLLMLLLLLMLLLLLLL","n":["Edge","MaxFlow","add_edge","borrow","borrow","borrow_mut","borrow_mut","cap","change_edge","clone","clone_into","default","edges","eq","eq","flow","flow","flow_with_capacity","fmt","fmt","from","from","from","get_edge","into","into","min_cut","new","to","to_owned","try_from","try_from","try_into","try_into","type_id","type_id"],"q":[[0,"maxflow"]],"d":["","最大流を解く","","","","","","","","","","","","","","<code>s != t</code> must hold, otherwise it panics.","","Parameters","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","",""],"i":[0,0,2,6,2,6,2,6,2,2,2,2,2,6,2,2,6,2,6,2,6,2,6,2,6,2,2,2,6,2,6,2,6,2,6,2],"f":[0,0,[[[2,[1]],3,3,1],3],[[]],[[]],[[]],[[]],0,[[[2,[1]],3,1,1]],[[[2,[4]]],[[2,[4]]]],[[]],[[],[[2,[5]]]],[[[2,[1]]],[[7,[[6,[1]]]]]],[[[6,[[0,[8,1]]]],[6,[[0,[8,1]]]]],9],[[[2,[8]],[2,[8]]],9],[[[2,[1]],3,3],1],0,[[[2,[1]],3,3,1],1],[[[6,[[0,[10,1]]]],11],12],[[[2,[10]],11],12],[[]],[[]],0,[[[2,[1]],3],[[6,[1]]]],[[]],[[]],[[[2,[1]],3],[[7,[9]]]],[3,[[2,[1]]]],0,[[]],[[],13],[[],13],[[],13],[[],13],[[],14],[[],14]],"c":[],"p":[[8,"Integral"],[3,"MaxFlow"],[15,"usize"],[8,"Clone"],[8,"Default"],[3,"Edge"],[3,"Vec"],[8,"PartialEq"],[15,"bool"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[4,"Result"],[3,"TypeId"]]},\
"maxflow_lower_bound":{"doc":"最小流量制限付き最大流 …","t":"DLLLLLLLLLLLL","n":["MaxFlowLowerBound","add_edge","add_edge_with_lower_bound","borrow","borrow_mut","flow","from","get_edge","into","new","try_from","try_into","type_id"],"q":[[0,"maxflow_lower_bound"]],"d":["","…","…","","","…","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,[[[2,[1]],3,3,1],3],[[[2,[1]],3,3,[4,[1]]],3],[[]],[[]],[[[2,[1]],3,3],[[5,[1]]]],[[]],[[[2,[1]],3],[[6,[1]]]],[[]],[3,[[2,[1]]]],[[],7],[[],7],[[],8]],"c":[],"p":[[8,"Integral"],[3,"MaxFlowLowerBound"],[15,"usize"],[8,"RangeBounds"],[4,"Option"],[3,"Edge"],[4,"Result"],[3,"TypeId"]]},\
"mo":{"doc":"区間[L, …","t":"IDKKLLLLLKLKKLLLL","n":["MoFuncs","MoRunner","add_left","add_right","borrow","borrow_mut","fmt","from","into","memo","new","remove_left","remove_right","run","try_from","try_into","type_id"],"q":[[0,"mo"]],"d":["","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,0,6,6,2,2,2,2,2,6,2,6,6,2,2,2,2],"f":[0,0,[1],[1],[[]],[[]],[[2,3],4],[[]],[[]],[1],[[1,5],2],[1],[1],[[2,6]],[[],7],[[],7],[[],8]],"c":[],"p":[[15,"usize"],[3,"MoRunner"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[8,"MoFuncs"],[4,"Result"],[3,"TypeId"]]},\
"modint_mersenne":{"doc":"","t":"DILLLLLLLLLLLLLLLLLKLLLLLLLL","n":["ModIntMersenne","RemEuclidU64","add","add_assign","borrow","borrow_mut","clone","clone_into","eq","fmt","fmt","from","hash","into","modulus","mul","mul_assign","neg","new","rem_euclid_u64","sub","sub_assign","to_owned","to_string","try_from","try_into","type_id","value"],"q":[[0,"modint_mersenne"]],"d":["","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,7,1,1,1,1,1,1,1,1],"f":[0,0,[[1,1],1],[[1,1]],[[]],[[]],[1,1],[[]],[[1,1],2],[[1,3],4],[[1,3],4],[[]],[[1,5]],[[]],[[],6],[[1,1],1],[[1,1]],[1,1],[7,1],[[],1],[[1,1],1],[[1,1]],[[]],[[],8],[[],9],[[],9],[[],10],[1,6]],"c":[],"p":[[3,"ModIntMersenne"],[15,"bool"],[3,"Formatter"],[6,"Result"],[8,"Hasher"],[15,"u64"],[8,"RemEuclidU64"],[3,"String"],[4,"Result"],[3,"TypeId"]]},\
"modint_traits":{"doc":"","t":"IILKKLKKK","n":["ModInt","RemEuclidU32","inv","modulus","new","pow","raw","rem_euclid_u32","value"],"q":[[0,"modint_traits"]],"d":["","Trait for primitive integer types.","","","","","","",""],"i":[0,0,4,4,4,4,4,2,4],"f":[0,0,[[]],[[],1],[2],[3],[1],[1,1],[[],1]],"c":[],"p":[[15,"u32"],[8,"RemEuclidU32"],[15,"u64"],[8,"ModInt"]]},\
"ntt":{"doc":"FFT 原始根, NTT friendly MOD 高速化 Reference …","t":"FF","n":["convolution","convolution_998244353"],"q":[[0,"ntt"]],"d":["NTTによる畳み込み","998244353 = 119 * 2^23 + 1 で原始根3を持つ"],"i":[0,0],"f":[[[],[[2,[1]]]],[[],[[2,[3]]]]],"c":[],"p":[[3,"StaticModInt"],[3,"Vec"],[6,"ModInt998244353"]]},\
"ntt_arbitrary_mod":{"doc":"任意mod  ","t":"F","n":["convolution_aribtrary_u32_mod"],"q":[[0,"ntt_arbitrary_mod"]],"d":["…"],"i":[0],"f":[[[],[[2,[1]]]]],"c":[],"p":[[8,"ModInt"],[3,"Vec"]]},\
"potentialized_union_find":{"doc":"ポテンシャル付きUnion-Find 可換群を載せる  ","t":"DLLLLLLLLLLLLL","n":["PotentializedUnionFind","borrow","borrow_mut","diff","fmt","from","into","new","relate","root_and_diff","size","try_from","try_into","type_id"],"q":[[0,"potentialized_union_find"]],"d":["","","","xから見たyの差分が定義されていれば返す","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","xからみたyの差分を追加する …","代表元と、それから見た差分を求める","…","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[0,[[]],[[]],[[[3,[[0,[1,2]]]],4,4],5],[[[3,[[0,[6,1,2]]]],7],8],[[]],[[]],[4,[[3,[[0,[1,2]]]]]],[[[3,[[0,[1,2]]]],4,4],[[10,[9]]]],[[[3,[[0,[1,2]]]],4]],[[[3,[[0,[1,2]]]],4],4],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"Group"],[8,"Commutative"],[3,"PotentializedUnionFind"],[15,"usize"],[4,"Option"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[15,"bool"],[4,"Result"],[3,"TypeId"]]},\
"raq_rsq":{"doc":"蟻本p165をもとにしている fenwick tree …","t":"DLLLLLLLLLLL","n":["RAQRSQ","add","add_point","borrow","borrow_mut","from","into","new","sum","try_from","try_into","type_id"],"q":[[0,"raq_rsq"]],"d":["","区間加算","1点加算","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,9,9,9,9,9,9,9,9,9,9,9],"f":[0,[[[9,[[0,[1,2,3,4,5,[7,[6]],8]]]],[10,[6]],[0,[1,2,3,4,5,[7,[6]],8]]]],[[[9,[[0,[1,2,3,4,5,[7,[6]],8]]]],6,[0,[1,2,3,4,5,[7,[6]],8]]]],[[]],[[]],[[]],[[]],[[6,[0,[1,2,3,4,5,[7,[6]],8]]],[[9,[[0,[1,2,3,4,5,[7,[6]],8]]]]]],[[[9,[[0,[1,2,3,4,5,[7,[6]],8]]]],[10,[6]]],[[0,[1,2,3,4,5,[7,[6]],8]]]],[[],11],[[],11],[[],12]],"c":[],"p":[[8,"Clone"],[8,"Add"],[8,"AddAssign"],[8,"Sub"],[8,"Neg"],[15,"usize"],[8,"TryFrom"],[8,"Mul"],[3,"RAQRSQ"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"rerooting":{"doc":"全方位木DP …","t":"QIDKLLLLLLLLLLL","n":["DPMonoid","Rerootable","Rerooting","add_root","borrow","borrow_mut","fmt","from","get_ans","into","leaf","new","try_from","try_into","type_id"],"q":[[0,"rerooting"]],"d":["DPテーブルに載せる可換モノイド …","","","部分木に頂点 subtree_root → new_root …","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","葉に入れる値(デフォルトでは単位元) …","","","",""],"i":[3,0,0,3,4,4,4,4,4,4,3,4,4,4,4],"f":[0,0,0,[[1,1]],[[]],[[]],[[[4,[[0,[2,3]]]],5],6],[[]],[[[4,[3]],1]],[[]],[1],[[[7,[[7,[1]]]]],[[4,[3]]]],[[],8],[[],8],[[],9]],"c":[],"p":[[15,"usize"],[8,"Debug"],[8,"Rerootable"],[3,"Rerooting"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[4,"Result"],[3,"TypeId"]]},\
"rolling_hash":{"doc":"","t":"DLLLLLLLLLLLLLLLLLLL","n":["RollingHash","borrow","borrow_mut","clone","clone_into","eq","fmt","from","get_base_pow","get_hash","get_prefix_hash","get_random_base","into","is_empty","len","new","to_owned","try_from","try_into","type_id"],"q":[[0,"rolling_hash"]],"d":["…","","","","","","","Returns the argument unchanged.","<code>base^i</code>を返す","部分列<code>s[range]</code>のhash値を返す <code>O(1)</code>","接頭辞のhash値を返す(<code>get_hash(0..i)</code>…","…","Calls <code>U::from(self)</code>.","","","sのrolling hashを構築 <code>O(|s|)</code> …","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[]],[[]],[1,1],[[]],[[1,1],2],[[1,3],4],[[]],[[1,5],6],[[1,[7,[5]]],6],[[1,5],6],[[],8],[[]],[1,2],[1,5],[[[9,[8]]],1],[[]],[[],10],[[],10],[[],11]],"c":[],"p":[[3,"RollingHash"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"usize"],[3,"ModIntMersenne"],[8,"RangeBounds"],[15,"u64"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"segtree":{"doc":"Reference","t":"DLLLLLLLLLLLLLLLLLLLL","n":["SegTree","all_prod","borrow","borrow_mut","clone","clone_into","eq","fmt","from","from","get","into","max_right","min_left","new","prod","set","to_owned","try_from","try_into","type_id"],"q":[[0,"segtree"]],"d":["","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,[[[2,[1]]]],[[]],[[]],[[[2,[[0,[3,1]]]]],[[2,[[0,[3,1]]]]]],[[]],[[[2,[[0,[4,1]]]],[2,[[0,[4,1]]]]],5],[[[2,[[0,[6,1]]]],7],8],[[]],[9,[[2,[1]]]],[[[2,[1]],10]],[[]],[[[2,[1]],10,11],10],[[[2,[1]],10,11],10],[10,[[2,[1]]]],[[[2,[1]],[12,[10]]]],[[[2,[1]],10]],[[]],[[],13],[[],13],[[],14]],"c":[],"p":[[8,"Monoid"],[3,"SegTree"],[8,"Clone"],[8,"PartialEq"],[15,"bool"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[15,"usize"],[8,"Fn"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"segtree_2d":{"doc":"内部で2次元配列を持つセグメント木 <code>O(HW)</code>…","t":"DLLLLLLLLLLLLLL","n":["SegTree2D","all_prod","borrow","borrow_mut","fmt","from","from","get","into","new","prod","set","try_from","try_into","type_id"],"q":[[0,"segtree_2d"]],"d":["","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[0,[[[3,[[0,[1,2]]]]]],[[]],[[]],[[[3,[[0,[4,1,2]]]],5],6],[[]],[[[7,[7]]],[[3,[[0,[1,2]]]]]],[[[3,[[0,[1,2]]]],8,8]],[[]],[[8,8],[[3,[[0,[1,2]]]]]],[[[3,[[0,[1,2]]]],[9,[8]],[9,[8]]]],[[[3,[[0,[1,2]]]],8,8]],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"Monoid"],[8,"Commutative"],[3,"SegTree2D"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[15,"usize"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"segtree_2d_compressed":{"doc":"…","t":"DLLLLLLLLLLLL","n":["SegTree2DCompressed","borrow","borrow_mut","fmt","from","get","into","new","prod","set","try_from","try_into","type_id"],"q":[[0,"segtree_2d_compressed"]],"d":["Tは座標圧縮する型  ","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,5,5,5,5,5,5,5,5,5,5,5,5],"f":[0,[[]],[[]],[[[5,[[0,[1,2,3]],[0,[1,4]]]],6],7],[[]],[[[5,[[0,[2,3]],4]],4,4]],[[]],[[],[[5,[[0,[2,3]],4]]]],[[[5,[[0,[2,3]],4]],[8,[4]],[8,[4]]]],[[[5,[[0,[2,3]],4]],4,4]],[[],9],[[],9],[[],10]],"c":[],"p":[[8,"Debug"],[8,"Monoid"],[8,"Commutative"],[8,"Integral"],[3,"SegTree2DCompressed"],[3,"Formatter"],[6,"Result"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"sparse_table":{"doc":"…","t":"DLLLLLLLLLLLLL","n":["SparseTable","borrow","borrow_mut","clone","clone_into","fmt","from","into","new","prod","to_owned","try_from","try_into","type_id"],"q":[[0,"sparse_table"]],"d":["","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","<code>O(nlogn)</code>","<code>O(1)</code>","","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[0,[[]],[[]],[[[3,[[0,[1,2]]]]],[[3,[[0,[1,2]]]]]],[[]],[[[3,[[0,[4,2]]]],5],6],[[]],[[]],[7,[[3,[2]]]],[[[3,[2]],[9,[8]]]],[[]],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"Clone"],[8,"IdempotentMonoid"],[3,"SparseTable"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[15,"usize"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"sparse_table_on_segtree":{"doc":"2D Sparse …","t":"DLLLLLLLLLL","n":["SparseTableOnSegTree","borrow","borrow_mut","fmt","from","into","new","prod","try_from","try_into","type_id"],"q":[[0,"sparse_table_on_segtree"]],"d":["","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","<code>O(HWlogH)</code>","<code>O(logH)</code>","","",""],"i":[0,5,5,5,5,5,5,5,5,5,5],"f":[0,[[]],[[]],[[[5,[[0,[1,2,3,4]]]],6],7],[[]],[[]],[[[8,[8]]],[[5,[[0,[2,3,4]]]]]],[[[5,[[0,[2,3,4]]]],[10,[9]],[0,[[10,[9]],4]]]],[[],11],[[],11],[[],12]],"c":[],"p":[[8,"Debug"],[8,"IdempotentMonoid"],[8,"Commutative"],[8,"Clone"],[3,"SparseTableOnSegTree"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[15,"usize"],[8,"RangeBounds"],[4,"Result"],[3,"TypeId"]]},\
"static_modint":{"doc":"","t":"GGDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["ModInt1000000007","ModInt998244353","StaticModInt","add","add_assign","add_assign","borrow","borrow_mut","clone","clone_into","default","div","div_assign","div_assign","eq","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from_str","hash","into","inv","modulus","modulus","mul","mul_assign","mul_assign","neg","new","new","one","pow","product","raw","raw","sub","sub_assign","sub_assign","sum","to_owned","to_string","try_from","try_into","type_id","value","value","zero"],"q":[[0,"static_modint"]],"d":["","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,0,0,[1,1],[[1,1]],[[1,2]],[[]],[[]],[1,1],[[]],[[],1],[1,1],[[1,1]],[[1,2]],[[1,1],3],[[1,4],5],[[1,4],5],[6,1],[7,1],[8,1],[9,1],[10,1],[11,1],[12,1],[[]],[13,1],[14,1],[15,1],[16,1],[17,1],[18,[[19,[1]]]],[[1,20]],[[]],[1,1],[[],13],[[],13],[1,1],[[1,2]],[[1,1]],[1,1],[2,1],[2,1],[[],1],[[1,11],1],[21,1],[13,1],[13,1],[1,1],[[1,1]],[[1,2]],[21,1],[[]],[[],22],[[],19],[[],19],[[],23],[1,13],[1,13],[[],1]],"c":[],"p":[[3,"StaticModInt"],[8,"RemEuclidU32"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"usize"],[15,"u16"],[15,"i16"],[15,"u128"],[15,"i128"],[15,"u64"],[15,"u8"],[15,"u32"],[15,"i64"],[15,"i32"],[15,"isize"],[15,"i8"],[15,"str"],[4,"Result"],[8,"Hasher"],[8,"Iterator"],[3,"String"],[3,"TypeId"]]},\
"top2":{"doc":"abc345e …","t":"DLLLLLLLLLLLLLLLL","n":["Top2Map","borrow","borrow_mut","clone","clone_into","default","first","fmt","from","insert","into","new","second","to_owned","try_from","try_into","type_id"],"q":[[0,"top2"]],"d":["Top2(大きい順)までのMapを持つ …","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5],"f":[0,[[]],[[]],[[[5,[[0,[1,2,3]],[0,[1,4,3]]]]],[[5,[[0,[1,2,3]],[0,[1,4,3]]]]]],[[]],[[],[[5,[[0,[2,3]],[0,[4,3]]]]]],[[[5,[[0,[2,3]],[0,[4,3]]]]],6],[[[5,[[0,[7,2,3]],[0,[7,4,3]]]],8],9],[[]],[[[5,[[0,[2,3]],[0,[4,3]]]],[0,[2,3]],[0,[4,3]]]],[[]],[[],[[5,[[0,[2,3]],[0,[4,3]]]]]],[[[5,[[0,[2,3]],[0,[4,3]]]]],6],[[]],[[],10],[[],10],[[],11]],"c":[],"p":[[8,"Clone"],[8,"Eq"],[8,"Copy"],[8,"Ord"],[3,"Top2Map"],[4,"Option"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[4,"Result"],[3,"TypeId"]]},\
"union_find":{"doc":"…","t":"DLLLLLLLLLLLLLLLLLLLL","n":["UnionFind","borrow","borrow_mut","clone","clone_into","eq","fmt","from","groups","into","is_empty","leader","len","merge","new","same","size","to_owned","try_from","try_into","type_id"],"q":[[0,"union_find"]],"d":["","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","合併しつつ、合併した集合の代表元を返す","","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[]],[[]],[1,1],[[]],[[1,1],2],[[1,3],4],[[]],[1,[[6,[[6,[5]]]]]],[[]],[1,2],[[1,5],5],[1,5],[[1,5,5],5],[5,1],[[1,5,5],2],[[1,5],5],[[]],[[],7],[[],7],[[],8]],"c":[],"p":[[3,"UnionFind"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"usize"],[3,"Vec"],[4,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
