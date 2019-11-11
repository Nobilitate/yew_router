# Changelog

<!-- START TEMPLATE

## ✨ **VERSION** *(DATE)*

- #### ⚡️ Features
  - Sample
- #### 🛠 Fixes
  - Sample
- #### 🚨 Breaking changes
  - Sample

END TEMPLATE-->

## ✨ **0.8.0** *(TBD)*

## ✨ **0.7.0** *(2019-11-11)*

- #### ⚡️ Features
  - Redirects that happen in the `Router` component actually change the url in the browser [[171](https://github.com/yewstack/yew_router/issues/171)]
  - Allow parsing (almost) any character after a `#` is encountered in matcher strings. 
  This enables this library to be used as a fragment router. [[150](https://github.com/yewstack/yew_router/issues/150)]
- #### 🛠 Fixes
  - Allow `!` to appear after `{...}` in matcher strings. [[148](https://github.com/yewstack/yew_router/issues/148)]
  - Matcher strings can now start with `&`. [[168](https://github.com/yewstack/yew_router/issues/168)] 
- #### 🚨 Breaking changes
  - Upgrade to Yew 0.10.0
  - Switch components now need to implement `Clone` in order to be used with the `Router` [[171](https://github.com/yewstack/yew_router/issues/171)]

## ✨ **0.6.1** *(2019-11-1)*
- #### ⚡️ Features
  - Bring back `{}`, `{*}`, and `{<number>}` capture syntax for tuple structs/enum variants. 
  If your variant or struct doesn't have named fields, you don't need to supply names in the matcher string [[116](https://github.com/yewstack/yew_router/issues/116)]
  - Allow ! special character in more places.
  - Greatly improve the quality of matcher string parsing errors. [[171](https://github.com/yewstack/yew_router/issues/149)]
  - Add `impl<SW: Switch, T> From<SW> for Route<T>`. Now Routes can be created from Switches easily.
  - Allow escaping {, }, and ! special characters by using `{{`, `}}`, and `!!` respectively.
  - Provide a correct error message when attempting to derive `Switch` for a Unit struct/variant with a capture group.

## ✨ **0.6.0** *(2019-10-24)*
- #### ⚡️ Features
  - `Switch` trait and Proc Macro enables extracting data from route strings.
  - `Router` component added.
  - `RouterLink` and `RouterButton` helper components added.
- #### 🚨 Breaking changes
  - Nearly everything. Most items were renamed.
  - Upgrade to Yew 0.9.0
