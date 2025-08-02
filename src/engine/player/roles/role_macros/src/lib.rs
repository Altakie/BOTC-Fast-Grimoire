use syn::{parse::Parse, token::Token, *};
struct Comp {
    player_index: PlayerIndex,
    right_effect: StatusEffectType,
    wrong_effect: StatusEffectType,
    target: TargetString,
}

impl Parse for Comp {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        // Input should look like function arguments
        let player_index = PlayerIndex::parse(input);
        _ = input.parse::<Token![,]>()?;
        let right_effect = StatusEffectType::parse(input);
        _ = input.parse::<Token![,]>()?;
        let wrong_effect = StatusEffectType::parse(input);
        _ = input.parse::<Token![,]>()?;
        let target = TargetString::parse(input);
    }
}

struct PlayerIndex(syn::Ident);

impl Parse for PlayerIndex {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        syn::Ident::parse(input).map(Self)
    }
}

struct StatusEffectType(syn::Type);

impl Parse for StatusEffectType {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        syn::Type::parse(input).map(Self)
    }
}

struct TargetString(syn::LitStr);

impl Parse for TargetString {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        input.parse::<syn::LitStr>().map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
