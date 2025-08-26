use quote::{ToTokens, quote};
use syn::{parse::Parse, *};
struct WasherwomanLibrarianInvestigator {
    player_index: PlayerIndex,
    right_effect: StatusEffectType,
    wrong_effect: StatusEffectType,
    character_type: CharacterType,
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
        let target = CharacterType::parse(input)?;

        Ok(Self {
            player_index,
            right_effect,
            wrong_effect,
            character_type: target,
        })
    }
}

impl quote::ToTokens for WasherwomanLibrarianInvestigator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let PlayerIndex(player_index) = &self.player_index;
        let StatusEffectType(right_effect) = &self.right_effect;
        let StatusEffectType(wrong_effect) = &self.wrong_effect;
        let CharacterType(character_type) = &self.character_type;

        tokens.extend(quote! {
            {
                let right_description = format!("Select a {}", #character_type.to_string());

                let right_status = move || StatusEffect::new(std::sync::Arc::new(#right_effect {}), #player_index, None);

                let change_type = ChangeType::ChoosePlayers(1);

                let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
                    let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

                    let len = target_player_indices.len();
                    if len != 1 {
                        return Err(ChangeError::WrongNumberOfSelectedPlayers { wanted: 1, got: len});
                    }

                    for target_player_index in target_player_indices {
                        if *target_player_index == player_index {
                            return Ok(false);
                        }

                        let player = state.get_player(*target_player_index);
                        if matches!(
                            player.get_character_type(),
                            #character_type | CharacterType::Any
                        ) {
                            return Ok(true);
                        }
                    }

                    return Ok(false);
                };

                let right_state_change = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                    let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

                    let target_player = state.get_player_mut(target_player_indices[0]);
                    target_player.add_status(right_status());


                    let wrong_status = move || StatusEffect::new(std::sync::Arc::new(#wrong_effect {}), #player_index, None);
                    let wrong_description = "Select a different player";

                    let wrong_change_type = ChangeType::ChoosePlayers(1);

                    let wrong_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
                        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

                        let len = target_player_indices.len();
                        if len != 1 {
                            return Err(ChangeError::WrongNumberOfSelectedPlayers { wanted: 1, got: len});
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

                    let wrong_state_change = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

                        // Assign the chosen player the wrong status effect
                        let target_player = state.get_player_mut(target_player_indices[0]);
                        target_player.add_status(wrong_status());

                        None
                    };

                    return Some(ChangeRequest::new(wrong_change_type, wrong_description.into(), wrong_check_func, wrong_state_change));
                };

                return Some(
                    ChangeRequest::new(
                        change_type,
                        right_description.into(),
                        right_check_func,
                        right_state_change
                    )
                );
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

struct CharacterType(syn::Type);

impl Parse for CharacterType {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        syn::Type::parse(input).map(Self)
    }
}

#[proc_macro]
pub fn washerwoman_librarian_investigator(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as WasherwomanLibrarianInvestigator);
    quote! {#c}.into()
}
