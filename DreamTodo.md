O Component scope css (prevent leaks)
O Nesting 
? Dont break hot-reload
MVP --------------------------------------------------------
O Support hot-reload
O Allow leak with ::deep
O Global styles, variables, keyframes, etc.
O Variables, functions, mixins, inheritance cfr. SCSS
O Element coscos id ellision via `parent > unique_across_sibs_tagname` or `parent > .unique_across_sibs_class ` or `parent > shared_tagname::pseudo` etc.
	O Across for
	O Across if
O Roots wrap <div style="display:contents;"></div> syntax
O Compile time combinator leak detection, drop component scope attribute if no leak, configurable warn/error on leak
O Component children argument styling
O Component style injection
O LSP support
O Tailwind syntax support
	e.g. div {
            coscos_tw : "bg-slate-100; w-24; & > div {h-24;}"
		}