# 附录 B：运算符与符号

该附录包含了 Rust 语法的词汇表，包括运算符以及其他的符号，这些符号单独出现或出现在路径、泛型、trait bounds、宏、属性、注释、元组以及大括号上下文中。

## 运算符

表 B-1 包含了 Rust 中的运算符、运算符如何出现在上下文中的示例、简短解释以及该运算符是否可重载。如果一个运算符是可重载的，则该运算符上用于重载的相关 trait 也会列出。

表 B-1: 运算符

<table><thead><tr><th>运算符</th><th>示例</th><th>解释</th><th>是否可重载</th></tr></thead><tbody>
<tr><td><code class="hljs">!</code></td><td><code class="hljs">ident!(...)</code>, <code class="hljs">ident!{...}</code>, <code class="hljs">ident![...]</code></td><td>宏展开</td><td></td></tr>
<tr><td><code class="hljs">!</code></td><td><code class="hljs">!expr</code></td><td>按位非或逻辑非</td><td><code class="hljs">Not</code></td></tr>
<tr><td><code class="hljs">!=</code></td><td><code class="hljs">var != expr</code></td><td>不等比较</td><td><code class="hljs">PartialEq</code></td></tr>
<tr><td><code class="hljs">%</code></td><td><code class="hljs">expr % expr</code></td><td>算术取模</td><td><code class="hljs">Rem</code></td></tr>
<tr><td><code class="hljs">%=</code></td><td><code class="hljs">var %= expr</code></td><td>算术取模与赋值</td><td><code class="hljs">RemAssign</code></td></tr>
<tr><td><code class="hljs">&amp;</code></td><td><code class="hljs">&amp;expr</code>, <code class="hljs">&amp;mut expr</code></td><td>借用</td><td></td></tr>
<tr><td><code class="hljs">&amp;</code></td><td><code class="hljs">&amp;type</code>, <code class="hljs">&amp;mut type</code>, <code class="hljs">&amp;'a type</code>, <code class="hljs">&amp;'a mut type</code></td><td>借用指针类型</td><td></td></tr>
<tr><td><code class="hljs">&amp;</code></td><td><code class="hljs">expr &amp; expr</code></td><td>按位与</td><td><code class="hljs">BitAnd</code></td></tr>
<tr><td><code class="hljs">&amp;=</code></td><td><code class="hljs">var &amp;= expr</code></td><td>按位与及赋值</td><td><code class="hljs">BitAndAssign</code></td></tr>
<tr><td><code class="hljs">&amp;&amp;</code></td><td><code class="hljs">expr &amp;&amp; expr</code></td><td>逻辑与</td><td></td></tr>
<tr><td><code class="hljs">*</code></td><td><code class="hljs">expr * expr</code></td><td>算术乘法</td><td><code class="hljs">Mul</code></td></tr>
<tr><td><code class="hljs">*=</code></td><td><code class="hljs">var *= expr</code></td><td>算术乘法与赋值</td><td><code class="hljs">MulAssign</code></td></tr>
<tr><td><code class="hljs">*</code></td><td><code class="hljs">*expr</code></td><td>解引用</td><td></td></tr>
<tr><td><code class="hljs">*</code></td><td><code class="hljs">*const type</code>, <code class="hljs">*mut type</code></td><td>裸指针</td><td></td></tr>
<tr><td><code class="hljs">+</code></td><td><code class="hljs">trait + trait</code>, <code class="hljs">'a + trait</code></td><td>复合类型限制</td><td></td></tr>
<tr><td><code class="hljs">+</code></td><td><code class="hljs">expr + expr</code></td><td>算术加法</td><td><code class="hljs">Add</code></td></tr>
<tr><td><code class="hljs">+=</code></td><td><code class="hljs">var += expr</code></td><td>算术加法与赋值</td><td><code class="hljs">AddAssign</code></td></tr>
<tr><td><code class="hljs">,</code></td><td><code class="hljs">expr, expr</code></td><td>参数以及元素分隔符</td><td></td></tr>
<tr><td><code class="hljs">-</code></td><td><code class="hljs">- expr</code></td><td>算术取负</td><td><code class="hljs">Neg</code></td></tr>
<tr><td><code class="hljs">-</code></td><td><code class="hljs">expr - expr</code></td><td>算术减法</td><td><code class="hljs">Sub</code></td></tr>
<tr><td><code class="hljs">-=</code></td><td><code class="hljs">var -= expr</code></td><td>算术减法与赋值</td><td><code class="hljs">SubAssign</code></td></tr>
<tr><td><code class="hljs">-&gt;</code></td><td><code class="hljs">fn(...) -&gt; type</code>, <code class="hljs">|...| -&gt; type</code></td><td>函数与闭包，返回类型</td><td></td></tr>
<tr><td><code class="hljs">.</code></td><td><code class="hljs">expr.ident</code></td><td>成员访问</td><td></td></tr>
<tr><td><code class="hljs">..</code></td><td><code class="hljs">..</code>, <code class="hljs">expr..</code>, <code class="hljs">..expr</code>, <code class="hljs">expr..expr</code></td><td>右排除范围</td><td></td></tr>
<tr><td><code class="hljs">..</code></td><td><code class="hljs">..expr</code></td><td>结构体更新语法</td><td></td></tr>
<tr><td><code class="hljs">..</code></td><td><code class="hljs">variant(x, ..)</code>, <code class="hljs">struct_type { x, .. }</code></td><td>“与剩余部分”的模式绑定</td><td></td></tr>
<tr><td><code class="hljs">...</code></td><td><code class="hljs">expr...expr</code></td><td>模式: 范围包含模式</td><td></td></tr>
<tr><td><code class="hljs">/</code></td><td><code class="hljs">expr / expr</code></td><td>算术除法</td><td><code class="hljs">Div</code></td></tr>
<tr><td><code class="hljs">/=</code></td><td><code class="hljs">var /= expr</code></td><td>算术除法与赋值</td><td><code class="hljs">DivAssign</code></td></tr>
<tr><td><code class="hljs">:</code></td><td><code class="hljs">pat: type</code>, <code class="hljs">ident: type</code></td><td>约束</td><td></td></tr>
<tr><td><code class="hljs">:</code></td><td><code class="hljs">ident: expr</code></td><td>结构体字段初始化</td><td></td></tr>
<tr><td><code class="hljs">:</code></td><td><code class="hljs">'a: loop {...}</code></td><td>循环标志</td><td></td></tr>
<tr><td><code class="hljs">;</code></td><td><code class="hljs">expr;</code></td><td>语句和语句结束符</td><td></td></tr>
<tr><td><code class="hljs">;</code></td><td><code class="hljs">[...; len]</code></td><td>固定大小数组语法的部分</td><td></td></tr>
<tr><td><code class="hljs">&lt;&lt;</code></td><td><code class="hljs">expr &lt;&lt; expr</code></td><td>左移</td><td><code class="hljs">Shl</code></td></tr>
<tr><td><code class="hljs">&lt;&lt;=</code></td><td><code class="hljs">var &lt;&lt;= expr</code></td><td>左移与赋值</td><td><code class="hljs">ShlAssign</code></td></tr>
<tr><td><code class="hljs">&lt;</code></td><td><code class="hljs">expr &lt; expr</code></td><td>小于比较</td><td><code class="hljs">PartialOrd</code></td></tr>
<tr><td><code class="hljs">&lt;=</code></td><td><code class="hljs">expr &lt;= expr</code></td><td>小于等于比较</td><td><code class="hljs">PartialOrd</code></td></tr>
<tr><td><code class="hljs">=</code></td><td><code class="hljs">var = expr</code>, <code class="hljs">ident = type</code></td><td>赋值/等值</td><td></td></tr>
<tr><td><code class="hljs">==</code></td><td><code class="hljs">expr == expr</code></td><td>等于比较</td><td><code class="hljs">PartialEq</code></td></tr>
<tr><td><code class="hljs">=&gt;</code></td><td><code class="hljs">pat =&gt; expr</code></td><td>匹配准备语法的部分</td><td></td></tr>
<tr><td><code class="hljs">&gt;</code></td><td><code class="hljs">expr &gt; expr</code></td><td>大于比较</td><td><code class="hljs">PartialOrd</code></td></tr>
<tr><td><code class="hljs">&gt;=</code></td><td><code class="hljs">expr &gt;= expr</code></td><td>大于等于比较</td><td><code class="hljs">PartialOrd</code></td></tr>
<tr><td><code class="hljs">&gt;&gt;</code></td><td><code class="hljs">expr &gt;&gt; expr</code></td><td>右移</td><td><code class="hljs">Shr</code></td></tr>
<tr><td><code class="hljs">&gt;&gt;=</code></td><td><code class="hljs">var &gt;&gt;= expr</code></td><td>右移与赋值</td><td><code class="hljs">ShrAssign</code></td></tr>
<tr><td><code class="hljs">@</code></td><td><code class="hljs">ident @ pat</code></td><td>模式绑定</td><td></td></tr>
<tr><td><code class="hljs">^</code></td><td><code class="hljs">expr ^ expr</code></td><td>按位异或</td><td><code class="hljs">BitXor</code></td></tr>
<tr><td><code class="hljs">^=</code></td><td><code class="hljs">var ^= expr</code></td><td>按位异或与赋值</td><td><code class="hljs">BitXorAssign</code></td></tr>
<tr><td><code class="hljs">|</code></td><td><code class="hljs">pat | pat</code></td><td>模式选择</td><td></td></tr>
<tr><td><code class="hljs">|</code></td><td><code class="hljs">expr | expr</code></td><td>按位或</td><td><code class="hljs">BitOr</code></td></tr>
<tr><td><code class="hljs">|=</code></td><td><code class="hljs">var |= expr</code></td><td>按位或与赋值</td><td><code class="hljs">BitOrAssign</code></td></tr>
<tr><td><code class="hljs">||</code></td><td><code class="hljs">expr || expr</code></td><td>逻辑或</td><td></td></tr>
<tr><td><code class="hljs">?</code></td><td><code class="hljs">expr?</code></td><td>错误传播</td><td></td></tr>
</tbody></table>

## 非运算符符号

下面的列表中包含了所有和运算符不一样功能的非字符符号；也就是说，他们并不像函数调用或方法调用一样表现。

表 B-2 展示了以其自身出现以及出现在合法其他各个地方的符号。

表 B-2：独立语法

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">'ident</code></td><td>命名生命周期或循环标签</td></tr>
<tr><td><code class="hljs">...u8</code>, <code class="hljs">...i32</code>, <code class="hljs">...f64</code>, <code class="hljs">...usize</code>, 等</td><td>指定类型的数值常量</td></tr>
<tr><td><code class="hljs">"..."</code></td><td>字符串常量</td></tr>
<tr><td><code class="hljs">r"..."</code>, <code class="hljs">r#"..."#</code>, <code class="hljs">r##"..."##</code>, etc.</td><td>原始字符串字面值, 未处理的转义字符</td></tr>
<tr><td><code class="hljs">b"..."</code></td><td>字节字符串字面值; 构造一个 <code class="hljs">[u8]</code> 类型而非字符串</td></tr>
<tr><td><code class="hljs">br"..."</code>, <code class="hljs">br#"..."#</code>, <code class="hljs">br##"..."##</code>, 等</td><td>原始字节字符串字面值，原始和字节字符串字面值的结合</td></tr>
<tr><td><code class="hljs">'...'</code></td><td>字符字面值</td></tr>
<tr><td><code class="hljs">b'...'</code></td><td>ASCII 码字节字面值</td></tr>
<tr><td><code class="hljs">|...| expr</code></td><td>闭包</td></tr>
<tr><td><code class="hljs">!</code></td><td>离散函数的总是为空的类型</td></tr>
<tr><td><code class="hljs">_</code></td><td>“忽略” 模式绑定；也用于增强整型字面值的可读性</td></tr>
</tbody></table>

表 B-3 展示了出现在从模块结构到项的路径上下文中的符号

表 B-3：路径相关语法

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">ident::ident</code></td><td>命名空间路径</td></tr>
<tr><td><code class="hljs">::path</code></td><td>与 crate 根相对的路径（如一个显式绝对路径）</td></tr>
<tr><td><code class="hljs">self::path</code></td><td>与当前模块相对的路径（如一个显式相对路径）</td></tr>
<tr><td><code class="hljs">super::path</code></td><td>与父模块相对的路径</td></tr>
<tr><td><code class="hljs">type::ident</code>, <code class="hljs">&lt;type as trait&gt;::ident</code></td><td>关联常量、函数以及类型</td></tr>
<tr><td><code class="hljs">&lt;type&gt;::...</code></td><td>不可以被直接命名的关联项类型（如 <code class="hljs">&lt;&amp;T&gt;::...</code>，<code class="hljs">&lt;[T]&gt;::...</code>， 等）</td></tr>
<tr><td><code class="hljs">trait::method(...)</code></td><td>通过命名定义的 trait 来消除方法调用的二义性</td></tr>
<tr><td><code class="hljs">type::method(...)</code></td><td>通过命名定义的类型来消除方法调用的二义性</td></tr>
<tr><td><code class="hljs">&lt;type as trait&gt;::method(...)</code></td><td>通过命名 trait 和类型来消除方法调用的二义性</td></tr>
</tbody></table>

表 B-4 展示了出现在泛型类型参数上下文中的符号。

表 B-4：泛型

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">path&lt;...&gt;</code></td><td>为一个类型中的泛型指定具体参数（如 <code class="hljs">Vec&lt;u8&gt;</code>）</td></tr>
<tr><td><code class="hljs">path::&lt;...&gt;</code>, <code class="hljs">method::&lt;...&gt;</code></td><td>为一个泛型、函数或表达式中的方法指定具体参数，通常指 turbofish（如 <code class="hljs">"42".parse::&lt;i32&gt;()</code>）</td></tr>
<tr><td><code class="hljs">fn ident&lt;...&gt; ...</code></td><td>泛型函数定义</td></tr>
<tr><td><code class="hljs">struct ident&lt;...&gt; ...</code></td><td>泛型结构体定义</td></tr>
<tr><td><code class="hljs">enum ident&lt;...&gt; ...</code></td><td>泛型枚举定义</td></tr>
<tr><td><code class="hljs">impl&lt;...&gt; ...</code></td><td>定义泛型实现</td></tr>
<tr><td><code class="hljs">for&lt;...&gt; type</code></td><td>高级生命周期限制</td></tr>
<tr><td><code class="hljs">type&lt;ident=type&gt;</code></td><td>泛型，其一个或多个相关类型必须被指定为特定类型（如 <code class="hljs">Iterator&lt;Item=T&gt;</code>）</td></tr>
</tbody></table>

表 B-5 展示了出现在使用 trait bounds 约束泛型参数上下文中的符号。

表 B-5: Trait Bound 约束

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">T: U</code></td><td>泛型参数 <code class="hljs">T</code> 约束于实现了 <code class="hljs">U</code> 的类型</td></tr>
<tr><td><code class="hljs">T: 'a</code></td><td>泛型 <code class="hljs">T</code> 的生命周期必须长于 <code class="hljs">'a</code>（意味着该类型不能传递包含生命周期短于 <code class="hljs">'a</code> 的任何引用）</td></tr>
<tr><td><code class="hljs">T : 'static</code></td><td>泛型 T 不包含除 'static 之外的借用引用</td></tr>
<tr><td><code class="hljs">'b: 'a</code></td><td>泛型 <code class="hljs">'b</code> 生命周期必须长于泛型 <code class="hljs">'a</code></td></tr>
<tr><td><code class="hljs">T: ?Sized</code></td><td>使用一个不定大小的泛型类型</td></tr>
<tr><td><code class="hljs">'a + trait</code>, <code class="hljs">trait + trait</code></td><td>复合类型限制</td></tr>
</tbody></table>

表 B-6 展示了在调用或定义宏以及在其上指定属性时的上下文中出现的符号。

表 B-6: 宏与属性

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">#[meta]</code></td><td>外部属性</td></tr>
<tr><td><code class="hljs">#![meta]</code></td><td>内部属性</td></tr>
<tr><td><code class="hljs">$ident</code></td><td>宏替换</td></tr>
<tr><td><code class="hljs">$ident:kind</code></td><td>宏捕获</td></tr>
<tr><td><code class="hljs">$(…)…</code></td><td>宏重复</td></tr>
</tbody></table>

表 B-7 展示了写注释的符号。

表 B-7: 注释

<table><thead><tr><th>符号</th><th>注释</th></tr></thead><tbody>
<tr><td><code class="hljs">//</code></td><td>行注释</td></tr>
<tr><td><code class="hljs">//!</code></td><td>内部行文档注释</td></tr>
<tr><td><code class="hljs">///</code></td><td>外部行文档注释</td></tr>
<tr><td><code class="hljs">/*...*/</code></td><td>块注释</td></tr>
<tr><td><code class="hljs">/*!...*/</code></td><td>内部块文档注释</td></tr>
<tr><td><code class="hljs">/**...*/</code></td><td>外部块文档注释</td></tr>
</tbody></table>

表 B-8 展示了出现在使用元组时上下文中的符号。

表 B-8: 元组

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">()</code></td><td>空元组（亦称单元），即是字面值也是类型</td></tr>
<tr><td><code class="hljs">(expr)</code></td><td>括号表达式</td></tr>
<tr><td><code class="hljs">(expr,)</code></td><td>单一元素元组表达式</td></tr>
<tr><td><code class="hljs">(type,)</code></td><td>单一元素元组类型</td></tr>
<tr><td><code class="hljs">(expr, ...)</code></td><td>元组表达式</td></tr>
<tr><td><code class="hljs">(type, ...)</code></td><td>元组类型</td></tr>
<tr><td><code class="hljs">expr(expr, ...)</code></td><td>函数调用表达式；也用于初始化元组结构体 <code class="hljs">struct</code> 以及元组枚举 <code class="hljs">enum</code> 变体</td></tr>
<tr><td><code class="hljs">ident!(...)</code>, <code class="hljs">ident!{...}</code>, <code class="hljs">ident![...]</code></td><td>宏调用</td></tr>
<tr><td><code class="hljs">expr.0</code>, <code class="hljs">expr.1</code>, etc.</td><td>元组索引</td></tr>
</tbody></table>

表 B-9 展示了使用大括号的上下文。

表 B-9: 大括号

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">{...}</code></td><td>块表达式</td></tr>
<tr><td><code class="hljs">Type {...}</code></td><td><code class="hljs">struct</code> 字面值</td></tr>
</tbody></table>

表 B-10 展示了使用方括号的上下文。

表 B-10: 方括号

<table><thead><tr><th>符号</th><th>解释</th></tr></thead><tbody>
<tr><td><code class="hljs">[...]</code></td><td>数组</td></tr>
<tr><td><code class="hljs">[expr; len]</code></td><td>复制了 <code class="hljs">len</code>个 <code class="hljs">expr</code>的数组</td></tr>
<tr><td><code class="hljs">[type; len]</code></td><td>包含 <code class="hljs">len</code>个 <code class="hljs">type</code> 类型的数组</td></tr>
<tr><td><code class="hljs">expr[expr]</code></td><td>集合索引。 重载（<code class="hljs">Index</code>, <code class="hljs">IndexMut</code>）</td></tr>
<tr><td><code class="hljs">expr[..]</code>, <code class="hljs">expr[a..]</code>, <code class="hljs">expr[..b]</code>, <code class="hljs">expr[a..b]</code></td><td>集合索引，使用 <code class="hljs">Range</code>，<code class="hljs">RangeFrom</code>，<code class="hljs">RangeTo</code> 或 <code class="hljs">RangeFull</code> 作为索引来代替集合 slice</td></tr>
</tbody></table>
