#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

use std::cmp::Ordering;

//Collects shuffled deck
fn deal(shuf: &[u8; 52]) -> [u8; 52]{
    //Sends deck to deal_cards to split into two piles
    let (mut pile1, mut pile2) = deal_cards(shuf);
    pile1.reverse();
    pile2.reverse();
    //Call the main function, where the game will be played
    main(pile1, pile2)
}

//Change 1s to 14s in order to rank as aces
fn modify_card(card: u8) -> u8 {
    if card == 1 {
        14
    } else {
        card
    }
}

//Change 14s back to 1s for the output
fn change_card(card: u8) -> u8 {
    if card == 14 {
        1
    } else {
        card
    }
}

//Maps through the cards to change all 1s to 14s
fn modify_cards(cards: &[u8]) -> Vec<u8> {
    cards.iter().map(|card| modify_card(*card)).collect()
}

//Maps through the cards to change all 14s to 1s
fn change_cards(cards: &[u8]) -> Vec<u8> {
    cards.iter().map(|card| change_card(*card)).collect()
}

//Function to split deck into two pules
fn deal_cards(shuf: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut pile1 = Vec::new();
    let mut pile2 = Vec::new();

    for (i, card) in shuf.iter().enumerate() {
        //Even or odd index (depends on player)
        if i % 2 == 0 {
            pile1.push(*card);
        } else {
            pile2.push(*card);
        }
    }

    (pile1, pile2)
}

//Main function where winning pile is returned
fn main(pile1: Vec<u8>, pile2: Vec<u8>) -> [u8; 52] {
    let mut result = [0u8; 52];
    //Ensure that the deck returns correctly after calling the reveal method (ensure that 1s are 14s while playing, and 14s are 1s when returning)
    let cards = change_cards(&reveal(&modify_cards(&pile1), &modify_cards(&pile2), &[]));
    for i in 0..52 {
        result[i] = cards[i];
    }
    //Return winner pile
    result
}

//Function where the game is played
fn reveal(pile1: &[u8], pile2: &[u8], war: &[u8]) -> Vec<u8> {
    let mut mp1 = pile1.to_owned();
    let mut mp2 = pile2.to_owned();
    let mut mwar = war.to_owned();
    //If pile1 is empty, return second player's pile
    if pile1.is_empty() {
        let mut war_res = war.to_owned();
        war_res.sort();
        war_res.reverse();
        mp2.append(&mut war_res);
        return mp2;
    //If pile2 is empty, return first player's pile
    } else if pile2.is_empty() {
        let mut war_res = war.to_owned();
        war_res.sort();
        war_res.reverse();
        mp1.append(&mut war_res);
        return mp1;
    }
    //If war pile is empty, compare the first card in each player's deck
    else if war.is_empty()
    {
        //Remove top card from each player's pile
        let p1_card = mp1.remove(0);
        let p2_card = mp2.remove(0);
        //If player 1's card is greater
        if p1_card > p2_card
        {
            mp1.push(p1_card);
            mp1.push(p2_card);
        } 
        //If player 2's card is greater
        else if p1_card < p2_card 
        {
            mp2.push(p2_card);
            mp2.push(p1_card);
        }
        //Add cards to war pile if there is a tie
        else {
            mwar.push(p1_card);
            mwar.push(p2_card);
        }
        //Continue the game with the updated piles
        return reveal(&mp1, &mp2, &mwar);
    }
    //If the war pile is not empty
    else
    {
        //If player 1 does not have enough cards to continue the war, return their pile to player 2
        if mp1.len() < 2
            {
                mp1.append(&mut mwar);
                mp1.sort();
                mp1.reverse();
                mp2.append(&mut mp1);
                return reveal(&[], &mp2, &[]);
            }
        //If player 2 does not have enough cards to continue the war, return their pile to player 1
        else if mp1.len() < 2
            {
                mp2.append(&mut mwar);
                mp2.sort();
                mp2.reverse();
                mp1.append(&mut mp2);
                return reveal(&mp1, &[], &[]);
            }
        //Continue the war with each player playing their next two cards and adding them to the war pile
        else
            {
                //Extract first two cards from each player's pile
                let (a, b, xs) = (mp1[0], mp1[1], &mp1[2..]);
                let (c, d, ys) = (mp2[0], mp2[1], &mp2[2..]);
                //Add the four cards to war pile
                let mut tie_play_cards = vec![a, b, c, d];
                tie_play_cards.append(&mut mwar);
                tie_play_cards.sort();
                tie_play_cards.reverse();
                let mut xs1 = xs.to_vec();
                let mut ys1 = ys.to_vec();

                //Compares the second card of both players in case of a tie
                match b.cmp(&d) {
                    Ordering::Greater => {
                        xs1.append(&mut tie_play_cards);
                         return reveal(&xs1, &ys, &[])},
                    Ordering::Less => { 
                        ys1.append(&mut tie_play_cards);
                         return reveal(&xs1, &ys1, &[])},
                    Ordering::Equal => { return reveal(&xs1, &ys1, &tie_play_cards)},
                }

            }
    }
}


#[cfg(test)]
#[path = "tests.rs"]
mod tests;

