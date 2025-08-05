use quote::{ToTokens, quote};
use syn::{parse::Parse, token::Token, *};

struct RolePtrArgs {
    role: RoleType,
}

impl Parse for RolePtrArgs {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(Self {
            role: RoleType::parse(input)?,
        })
    }
}

impl ToTokens for RolePtrArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let RoleType(role) = &self.role;
        tokens.extend(quote! {
            {
                let role = #role::default();
                RolePtr(std::sync::Arc::new(role))
            }
        })
    }
}

struct RoleType(syn::Type);

impl Parse for RoleType {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        syn::Type::parse(input).map(Self)
    }
}

#[proc_macro]
pub fn roleptr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as RolePtrArgs);
    quote! {#parsed}.into()
}

struct RolePtrFromArgs {
    role: RoleStruct,
}

impl Parse for RolePtrFromArgs {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(Self {
            role: RoleStruct::parse(input)?,
        })
    }
}

impl ToTokens for RolePtrFromArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let RoleStruct(role_struct) = &self.role;
        tokens.extend(quote! {
            RolePtr(std::sync::Arc::new(#role_struct))
        });
    }
}

struct RoleStruct(syn::ExprStruct);

impl Parse for RoleStruct {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        syn::ExprStruct::parse(input).map(Self)
    }
}

#[proc_macro]
pub fn roleptr_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(input as RolePtrFromArgs);
    quote! {#p}.into()
}

struct WasherwomanLibrarianInvestigator {
    player_index: PlayerIndex,
    right_effect: StatusEffectType,
    wrong_effect: StatusEffectType,
    target: TargetString,
}

impl Parse for WasherwomanLibrarianInvestigator {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        // Input should look like function arguments
        let player_index = PlayerIndex::parse(input)?;
        _ = input.parse::<Token![,]>()?;
        let right_effect = StatusEffectType::parse(input)?;
        _ = input.parse::<Token![,]>()?;
        let wrong_effect = StatusEffectType::parse(input)?;
        _ = input.parse::<Token![,]>()?;
        let target = TargetString::parse(input)?;

        Ok(Self {
            player_index,
            right_effect,
            wrong_effect,
            target,
        })
    }
}

impl quote::ToTokens for WasherwomanLibrarianInvestigator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let PlayerIndex(player_index) = &self.player_index;
        let StatusEffectType(right_effect) = &self.right_effect;
        let StatusEffectType(wrong_effect) = &self.wrong_effect;
        let TargetString(target_string) = &self.target;

        tokens.extend(quote! {
            {
                let right_description = format!("Select a {}", #target_string);
                let wrong_description = "Select a different player".to_string();

                let right_status = move || StatusEffect::new(std::sync::Arc::new(#right_effect {}), #player_index);
                let wrong_status = move || StatusEffect::new(std::sync::Arc::new(#wrong_effect {}), #player_index);

                let change_type = ChangeType::ChoosePlayers(1);
                let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
                    let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

                    if target_player_indices.len() != 1 {
                        return Err(());
                    }

                    for target_player_index in target_player_indices {
                        if *target_player_index == player_index {
                            return Ok(false);
                        }

                        let player = state.get_player(*target_player_index);
                        if matches!(
                            player.get_character_type(),
                            CharacterType::Townsfolk | CharacterType::Any
                        ) {
                            return Ok(true);
                        }
                    }

                    return Ok(false);
                };

                let right_state_change = move |state: &mut State, args: ChangeArgs| {
                    let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

                    let target_player = state.get_player_mut(target_player_indices[0]);
                    target_player.add_status(right_status());
                };

                let wrong_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
                    let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

                    if target_player_indices.len() != 1 {
                        return Err(());
                    }

                    let target_player_index = target_player_indices[0];

                    if target_player_index == player_index {
                        return Ok(false);
                    }

                    let target_player = state.get_player(target_player_index);
                    if target_player
                        .get_statuses()
                        .iter()
                        .any(|se| *se == right_status())
                    {
                        return Ok(false);
                    }
                    return Ok(true);
                };

                let wrong_state_change = move |state: &mut State, args: ChangeArgs| {
                    let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

                    // Assign the chosen player the wrong status effect
                    let target_player = state.get_player_mut(target_player_indices[0]);
                    target_player.add_status(wrong_status());
                };

                Some(vec![
                    new_change_request!(
                        change_type,
                        right_description,
                        right_check_func,
                        right_state_change
                    ),
                    new_change_request!(
                        change_type,
                        wrong_description,
                        wrong_check_func,
                        wrong_state_change
                    ),
                ])
            }
        });
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

#[proc_macro]
pub fn washerwoman_librarian_investigator(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as WasherwomanLibrarianInvestigator);
    quote! {#c}.into()
}
