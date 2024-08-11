(function() {var type_impls = {
"assembly_fdb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3C%26%5Bu8%5D,+%26%5BT%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#173-178\">source</a><a href=\"#impl-BaseHandle%3C%26%5Bu8%5D,+%26%5BT%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>], &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[T]</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.get\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#175-177\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.get\" class=\"fn\">get</a>(self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"type\" href=\"assembly_fdb/ro/type.RefHandle.html\" title=\"type assembly_fdb::ro::RefHandle\">RefHandle</a>&lt;'a, T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the reference at <code>index</code></p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3C%26%5Bu8%5D,+%26T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#180-185\">source</a><a href=\"#impl-BaseHandle%3C%26%5Bu8%5D,+%26T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"assembly_core/buffer/trait.Repr.html\" title=\"trait assembly_core::buffer::Repr\">Repr</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>], <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a T</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_extract\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#182-184\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.map_extract\" class=\"fn\">map_extract</a>(self) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/type.Handle.html\" title=\"type assembly_fdb::ro::Handle\">Handle</a>&lt;'a, T::<a class=\"associatedtype\" href=\"assembly_core/buffer/trait.Repr.html#associatedtype.Value\" title=\"type assembly_core::buffer::Repr::Value\">Value</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Handle&lt;&#39;a, T::Value&gt;\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Extract a value from a reference</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3C%26%5Bu8%5D,+()%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#177-188\">source</a><a href=\"#impl-BaseHandle%3C%26%5Bu8%5D,+()%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>], <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_ref\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#179-181\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.new_ref\" class=\"fn\">new_ref</a>(mem: &amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a new database handle</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.tables\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#184-187\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.tables\" class=\"fn\">tables</a>(&amp;self) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.Result.html\" title=\"type assembly_fdb::ro::handle::Result\">Result</a>&lt;'a, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBHeader.html\" title=\"struct assembly_fdb::file::FDBHeader\">FDBHeader</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the header for the local database</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+()%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#31-39\">source</a><a href=\"#impl-BaseHandle%3CP,+()%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#36-38\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.new\" class=\"fn\">new</a>(mem: P) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new handle</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+()%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#110-118\">source</a><a href=\"#impl-BaseHandle%3CP,+()%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    P::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_tables\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#115-117\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.into_tables\" class=\"fn\">into_tables</a>(self) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBHeader.html\" title=\"struct assembly_fdb::file::FDBHeader\">FDBHeader</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+FDBHeader%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#120-142\">source</a><a href=\"#impl-BaseHandle%3CP,+FDBHeader%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBHeader.html\" title=\"struct assembly_fdb::file::FDBHeader\">FDBHeader</a>&gt;<div class=\"where\">where\n    P::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_table_at\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#125-130\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.into_table_at\" class=\"fn\">into_table_at</a>(\n    self,\n    index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>,\n) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableHeader.html\" title=\"struct assembly_fdb::file::FDBTableHeader\">FDBTableHeader</a>&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_table_by_name\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#133-141\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.into_table_by_name\" class=\"fn\">into_table_by_name</a>(\n    self,\n    name: &amp;Latin1Str,\n) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableHeader.html\" title=\"struct assembly_fdb::file::FDBTableHeader\">FDBTableHeader</a>&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+FDBTableDataHeader%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#159-172\">source</a><a href=\"#impl-BaseHandle%3CP,+FDBTableDataHeader%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableDataHeader.html\" title=\"struct assembly_fdb::file::FDBTableDataHeader\">FDBTableDataHeader</a>&gt;<div class=\"where\">where\n    P::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_bucket_for_hash\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#164-171\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.get_bucket_for_hash\" class=\"fn\">get_bucket_for_hash</a>(self, id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBBucketHeader.html\" title=\"struct assembly_fdb::file::FDBBucketHeader\">FDBBucketHeader</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the bucket for a particular id / hash</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+FDBTableHeader%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#144-157\">source</a><a href=\"#impl-BaseHandle%3CP,+FDBTableHeader%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableHeader.html\" title=\"struct assembly_fdb::file::FDBTableHeader\">FDBTableHeader</a>&gt;<div class=\"where\">where\n    P::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_definition\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#149-151\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.into_definition\" class=\"fn\">into_definition</a>(self) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableDefHeader.html\" title=\"struct assembly_fdb::file::FDBTableDefHeader\">FDBTableDefHeader</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_data\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#154-156\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.into_data\" class=\"fn\">into_data</a>(self) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, <a class=\"struct\" href=\"assembly_fdb/file/struct.FDBTableDataHeader.html\" title=\"struct assembly_fdb::file::FDBTableDataHeader\">FDBTableDataHeader</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+Option%3CT%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#41-53\">source</a><a href=\"#impl-BaseHandle%3CP,+Option%3CT%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt;<div class=\"where\">where\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.transpose\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#46-52\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.transpose\" class=\"fn\">transpose</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Turns a handle of an option into an option of a handle</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#55-78\">source</a><a href=\"#impl-BaseHandle%3CP,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>, T&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;<div class=\"where\">where\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.raw\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#60-62\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.raw\" class=\"fn\">raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;T</a></h4></section></summary><div class=\"docblock\"><p>Get a reference to the raw value inside</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.raw_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#65-67\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.raw_mut\" class=\"fn\">raw_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut T</a></h4></section></summary><div class=\"docblock\"><p>Get a reference to the raw value inside</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_bytes\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#70-72\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.as_bytes\" class=\"fn\">as_bytes</a>(&amp;self) -&gt; &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>] <a href=\"#\" class=\"tooltip\" data-notable-ty=\"&amp;[u8]\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Get the byte slice for the whole database</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.replace\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#75-77\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.replace\" class=\"fn\">replace</a>&lt;O&gt;(self, raw: O) -&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, O&gt;</h4></section></summary><div class=\"docblock\"><p>Replace the value that is stored with the memory pointer</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseHandle%3CP,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#87-108\">source</a><a href=\"#impl-BaseHandle%3CP,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>, T&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;<div class=\"where\">where\n    P::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_into\" class=\"method\"><a class=\"src rightside\" href=\"src/assembly_fdb/ro/handle.rs.html#92-107\">source</a><h4 class=\"code-header\">pub fn <a href=\"assembly_fdb/ro/struct.BaseHandle.html#tymethod.map_into\" class=\"fn\">map_into</a>&lt;M, O, E&gt;(self, map: M) -&gt; <a class=\"type\" href=\"assembly_fdb/ro/handle/type.BaseResult.html\" title=\"type assembly_fdb::ro::handle::BaseResult\">BaseResult</a>&lt;P, O&gt;<div class=\"where\">where\n    M: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>], T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;O, E&gt;,\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"assembly_fdb/ro/handle/enum.BaseErrorKind.html\" title=\"enum assembly_fdb::ro::handle::BaseErrorKind\">BaseErrorKind</a>&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Get the tables</p>\n</div></details></div></details>",0,"assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BaseHandle%3CP,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#12\">source</a><a href=\"#impl-Clone-for-BaseHandle%3CP,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;<div class=\"where\">where\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#12\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#175\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BaseHandle%3CP,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#12\">source</a><a href=\"#impl-Debug-for-BaseHandle%3CP,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;<div class=\"where\">where\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#12\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"],["<section id=\"impl-Copy-for-BaseHandle%3CP,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/assembly_fdb/handle.rs.html#23-29\">source</a><a href=\"#impl-Copy-for-BaseHandle%3CP,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"assembly_fdb/ro/struct.BaseHandle.html\" title=\"struct assembly_fdb::ro::BaseHandle\">BaseHandle</a>&lt;P, T&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    &lt;P as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]&gt;,</div></h3></section>","Copy","assembly_fdb::handle::Handle","assembly_fdb::ro::ArcHandle"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()