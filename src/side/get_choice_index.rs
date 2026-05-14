use crate::side::*;

impl Side {

    /// Get the current choice action index
    //
    // 	getChoiceIndex(isPass?: boolean) {
    // 		let index = this.choice.actions.length;
    //
    // 		if (!isPass) {
    // 			switch (this.requestState) {
    // 			case 'move':
    // 				// auto-pass
    // 				while (
    // 					index < this.active.length &&
    // 					(this.active[index].fainted || this.active[index].volatiles['commanding'])
    // 				) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			case 'switch':
    // 				while (index < this.active.length && !this.active[index].switchFlag) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			}
    // 		}
    //
    // 		return index;
    // 	}
    //
    pub fn get_choice_index(&mut self, is_pass: bool) -> usize {
        let mut index = self.choice.actions.len();

        if !is_pass {
            match self.request_state {
                RequestState::Move => {
                    // auto-pass for fainted Pokemon or those with 'commanding' volatile
                    while index < self.active.len() {
                        if let Some(Some(pokemon_idx)) = self.active.get(index) {
                            if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                                use crate::dex_data::ID;
                                let is_fainted = pokemon.is_fainted();
                                let has_commanding = pokemon.volatiles.contains_key(&ID::from("commanding"));

                                if is_fainted || has_commanding {
                                    self.choose_pass();
                                    index += 1;
                                    continue;
                                }
                            }
                        }
                        break;
                    }
                }
                RequestState::Switch => {
                    // JS: while (index < this.active.length && !this.active[index].switchFlag) {
                    //     this.choosePass(); index++;
                    // }
                    // In JS, switchFlag is always set when requestState == 'switch' (forced
                    // switches set it). The Rust port doesn't set switchFlag consistently for
                    // all forced switch scenarios (U-turn, Roar, Dragon Tail, etc.).
                    // If NO active slot has switch_flag but request_state is Switch, the
                    // battle engine still expects a switch — skip auto-pass to accept it.
                    let any_switch_flag = (0..self.active.len()).any(|i| {
                        self.active.get(i)
                            .and_then(|&a| a)
                            .and_then(|idx| self.pokemon.get(idx))
                            .map_or(false, |p| p.switch_flag.is_some())
                    });
                    if any_switch_flag {
                        while index < self.active.len() {
                            if let Some(Some(pokemon_idx)) = self.active.get(index) {
                                if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                                    if pokemon.switch_flag.is_none() {
                                        self.choose_pass();
                                        index += 1;
                                        continue;
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        index
    }
}
