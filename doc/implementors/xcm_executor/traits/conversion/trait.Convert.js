(function() {var implementors = {
"xcm_builder":[["impl&lt;Network:&nbsp;Get&lt;<a class=\"enum\" href=\"xcm_builder/test_utils/enum.NetworkId.html\" title=\"enum xcm_builder::test_utils::NetworkId\">NetworkId</a>&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.Account32Hash.html\" title=\"struct xcm_builder::Account32Hash\">Account32Hash</a>&lt;Network, AccountId&gt;"],["impl&lt;AccountId:&nbsp;Decode + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.ParentIsPreset.html\" title=\"struct xcm_builder::ParentIsPreset\">ParentIsPreset</a>&lt;AccountId&gt;"],["impl&lt;ParaId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + AccountIdConversion&lt;AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.ChildParachainConvertsVia.html\" title=\"struct xcm_builder::ChildParachainConvertsVia\">ChildParachainConvertsVia</a>&lt;ParaId, AccountId&gt;"],["impl&lt;ParaId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + AccountIdConversion&lt;AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SiblingParachainConvertsVia.html\" title=\"struct xcm_builder::SiblingParachainConvertsVia\">SiblingParachainConvertsVia</a>&lt;ParaId, AccountId&gt;"],["impl&lt;Network:&nbsp;Get&lt;<a class=\"enum\" href=\"xcm_builder/test_utils/enum.NetworkId.html\" title=\"enum xcm_builder::test_utils::NetworkId\">NetworkId</a>&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.AccountId32Aliases.html\" title=\"struct xcm_builder::AccountId32Aliases\">AccountId32Aliases</a>&lt;Network, AccountId&gt;"],["impl&lt;Network:&nbsp;Get&lt;<a class=\"enum\" href=\"xcm_builder/test_utils/enum.NetworkId.html\" title=\"enum xcm_builder::test_utils::NetworkId\">NetworkId</a>&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">20</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">20</a>]&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AccountId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.AccountKey20Aliases.html\" title=\"struct xcm_builder::AccountKey20Aliases\">AccountKey20Aliases</a>&lt;Network, AccountId&gt;"],["impl&lt;RuntimeOrigin:&nbsp;OriginTrait + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt;, Network:&nbsp;Get&lt;<a class=\"enum\" href=\"xcm_builder/test_utils/enum.NetworkId.html\" title=\"enum xcm_builder::test_utils::NetworkId\">NetworkId</a>&gt;&gt; Convert&lt;RuntimeOrigin, <a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SignedToAccountId32.html\" title=\"struct xcm_builder::SignedToAccountId32\">SignedToAccountId32</a>&lt;RuntimeOrigin, AccountId, Network&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;RuntimeOrigin::PalletsOrigin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;SystemRawOrigin&lt;AccountId&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\" title=\"trait core::convert::TryInto\">TryInto</a>&lt;SystemRawOrigin&lt;AccountId&gt;, Error = RuntimeOrigin::PalletsOrigin&gt;,</span>"],["impl&lt;RuntimeOrigin:&nbsp;OriginTrait + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, COrigin:&nbsp;GetBacking, Body:&nbsp;Get&lt;<a class=\"enum\" href=\"xcm_builder/test_utils/enum.BodyId.html\" title=\"enum xcm_builder::test_utils::BodyId\">BodyId</a>&gt;&gt; Convert&lt;RuntimeOrigin, <a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>&gt; for <a class=\"struct\" href=\"xcm_builder/struct.BackingToPlurality.html\" title=\"struct xcm_builder::BackingToPlurality\">BackingToPlurality</a>&lt;RuntimeOrigin, COrigin, Body&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;RuntimeOrigin::PalletsOrigin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;COrigin&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\" title=\"trait core::convert::TryInto\">TryInto</a>&lt;COrigin, Error = RuntimeOrigin::PalletsOrigin&gt;,</span>"],["impl&lt;Prefix:&nbsp;Get&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>&gt;, AssetId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ConvertAssetId:&nbsp;Convert&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a>, AssetId&gt;&gt; Convert&lt;<a class=\"struct\" href=\"xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct xcm_builder::test_utils::MultiLocation\">MultiLocation</a>, AssetId&gt; for <a class=\"struct\" href=\"xcm_builder/struct.AsPrefixedGeneralIndex.html\" title=\"struct xcm_builder::AsPrefixedGeneralIndex\">AsPrefixedGeneralIndex</a>&lt;Prefix, AssetId, ConvertAssetId&gt;"]],
"xcm_executor":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()