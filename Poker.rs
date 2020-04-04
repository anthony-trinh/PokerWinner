//Methods for initializing/formatting hands 

fn get_hand_one(list:[usize;10], deck:[(i32,char);53]) -> [(i32, char);5]{
	//initializes hand one 
	let mut ret = [(0,'Z'), (0,'Z'), (0,'Z'), (0,'Z'), (0,'Z')];
	
	ret[0] = deck[list[0]] ; 
	ret[1] = deck[list[2]] ; 
	ret[2] = deck[list[4]] ; 
	ret[3] = deck[list[6]] ; 
	ret[4] = deck[list[8]] ; 

	ret 
}


fn get_hand_two(list:[usize;10], deck:[(i32,char);53]) -> [(i32, char);5]{
	//initialize hand two
	let mut ret = [(0,'Z'), (0,'Z'), (0,'Z'), (0,'Z'), (0,'Z')];
	
	ret[0] = deck[list[1]] ; 
	ret[1] = deck[list[3]] ; 
	ret[2] = deck[list[5]] ; 
	ret[3] = deck[list[7]] ; 
	ret[4] = deck[list[9]] ; 
 
	ret 
}


fn format_card(card:(i32, char)) -> String{ 
	//formats a card from tuple to string 
	let mut rank:String = (card.0).to_string();
	rank.push_str(&(card.1).to_string());
	let rank_slice:String = rank[..].to_string() ; 
	rank_slice
}


fn format_hand(hand:[(i32, char);5]) -> [String;5]{ 
	//format an entire hand of cards 
	let ret:[String;5] = [format_card(hand[0]), format_card(hand[1]),
	format_card(hand[2]), format_card(hand[3]), format_card(hand[4])] ;

	sort(ret)
}


//Boolean functions regarding hands and their types

fn is_same_suit(hand:[(i32,char);5]) -> bool{ 
	let suit = hand[0].1 ; 

	hand.iter().all(|&x| x.1 == suit) 
}


fn is_royal_flush(hand:[(i32,char);5]) -> bool{
	//return true iff this variable hand is a royal flush 

	let mut flag:bool = true ; 
	let suits:[i32;5] = [1,10,11,12,13] ; 

	for elem in hand.iter(){
		if suits.iter().any(|&x| x == elem.0) {
			flag = true ;
		}
		else{
			flag = false ;
			break ; 
		}
	}

	flag && is_same_suit(hand)
}


fn is_straight_flush(hand:[(i32,char);5]) -> bool{
	//return true iff variable hand is a straight flush 
	let max = hand.iter().max().unwrap().0 ; 
	let min = hand.iter().min().unwrap().0 ; 

	((max - min) == 4) && (is_same_suit(hand))	
}


fn is_four_of_a_kind(hand:[(i32,char);5]) -> bool{
	//return true iff variable hand is four of a kind 
	let max = hand.iter().max().unwrap().0 ; 
	let min = hand.iter().min().unwrap().0 ;

	(hand.iter().filter(|&x| x.0 == min).count() == 4) || (hand.iter().filter(|&x| x.0 == max).count() == 4)
}


fn is_full_house(hand:[(i32,char);5]) -> bool {
	//return true iff variable hand is 
	let max = hand.iter().max().unwrap().0 ; 
	let min = hand.iter().min().unwrap().0 ;
	let flag1 = (hand.iter().filter(|&x| x.0 == min).count() == 3) && (hand.iter().filter(|&x| x.0 == max).count() == 2) ; 
	let flag2 = (hand.iter().filter(|&x| x.0 == min).count() == 2) && (hand.iter().filter(|&x| x.0 == max).count() == 3) ;

	flag1 || flag2 
}


fn is_flush(hand:[(i32,char);5]) -> bool{ 
	//return true iff variable hand is a flush
	let same_suit:bool = is_same_suit(hand) ;
	let flag1: bool = !(is_straight_flush(hand)) ;
	let flag2: bool = !(is_royal_flush(hand)) ;

	same_suit && flag1 && flag2
}


fn is_straight(hand:[(i32,char);5]) -> bool{
	//return true iff variable hand is a straight flush 
	let max = hand.iter().max().unwrap().0 ; 
	let min = hand.iter().min().unwrap().0 ; 

	((max - min) == 4) && !(is_same_suit(hand))
}


fn is_three_of_a_kind(hand:[(i32,char);5]) -> bool{ 
	//return true iff variable hand is a three of a kind 

	let mut flag: bool = false ; 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ; 

	for rank in ranks.iter(){
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 3{
			flag = true ;
			break ; 
		}
	}

	flag && !(is_full_house(hand))
}


fn is_two_pair(hand:[(i32,char);5]) -> bool{
	//return true iff variable hand is a two pair 
	let mut pair_one_flag:bool = false ; 
	let mut pair_two_flag:bool = false ;
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;

	for rank in ranks.iter(){ 
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 2{
			pair_one_flag = true ; 
			continue ;
		}
		if ((hand.iter().filter(|&x| &x.0 == rank).count()) == 2) && pair_one_flag{
			pair_two_flag = true ; 
			break ; 
		}
	}

	pair_one_flag && pair_two_flag && !is_four_of_a_kind(hand) && !is_three_of_a_kind(hand) && !is_full_house(hand)
}


fn is_pair(hand:[(i32,char);5]) -> bool{
	//return true iff variable hand is a pair 
	let mut pair_flag:bool = false ; 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;

	for rank in ranks.iter(){
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 2{
			pair_flag = true ; 
			break ; 
		}
	}

	pair_flag && !is_four_of_a_kind(hand) && !is_three_of_a_kind(hand) && !is_two_pair(hand) && !is_full_house(hand)
}


//Scoring and tie-breaking methods and helpers 

fn sort<A, T>(mut array: A) -> A
//sort an array
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();

    array
}

fn get_score(hand:[(i32,char);5]) -> i32{
	//give a score based on what type of hand is given
	if is_royal_flush(hand){
		10
	}
	else if is_straight_flush(hand){
		9
	}
	else if is_four_of_a_kind(hand){
		8
	}
	else if is_full_house(hand){
		7
	}
	else if is_flush(hand){
		6
	}
	else if is_straight(hand){
		5
	}
	else if is_three_of_a_kind(hand){
		4
	}
	else if is_two_pair(hand){
		3
	}
	else if is_pair(hand){
		2
	}
	else{
		1
	}
}


fn get_max_rank(hand:[(i32,char);5]) -> i32{
	//get the max rank of cards in hand 

	let mut max_rank:i32 = 1 ; 

	for card in hand.iter(){ 
		if card.0 > max_rank{
			max_rank = card.0 ; 
		}
	}

	max_rank 
}


fn get_min_rank(hand:[(i32,char);5]) -> i32{
	//get min rank of cards in hand 

	let mut min_rank:i32 = 1 ; 

	for card in hand.iter(){
		if card.0 < min_rank{
			min_rank = card.0 ; 
		}
	}

	min_rank

}


fn get_rank_list(hand:[(i32,char);5]) -> [i32;5]{
	//get a list of all the ranks that occur in a hand 
	let rank_list:[i32;5] = [hand[0].0, hand[1].0, hand[2].0, hand[3].0, hand[4].0] ;

	rank_list 
}


fn get_hand_total(hand:[(i32,char);5]) -> i32{
	//get the total numeric value to a hand
	//based on the index of variable deck where a card belongs
	let mut total:i32 = 0 ; 

	for card in hand.iter()
	{
		let suit:char = card.1 ; 

		match suit{
			'C' => total = total + card.0,  
			'D' => total = total + card.0 + 13,
			'H' => total = total + card.0 + 26,
			'S' => total = total + card.0 + 39,
			_ => total = total,
		}
	}

	total
}


fn tie_break_five(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	//tie breaking method for hands which have five relevant cards
	//i.e. royal flushes, straight flushes, straights, full houses, and flushes

	let max1 = get_max_rank(hand1) ; 
	let max2 = get_max_rank(hand2) ; 

	if max1 > max2{
		hand1 
	}
	else if max1 < max2{
		hand2
	}
	else{
		let total1 = get_hand_total(hand1) ; 
		let total2 = get_hand_total(hand2) ;
		if total1 > total2{
			hand1
		}
		else{
			hand2
		}
	}
}


fn tie_break_four(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	//tie breaking method for a four of a kind 
	let max1 = get_max_rank(hand1) ; 
	let max2 = get_max_rank(hand2) ;

	if max1 > max2{
		hand1 
	}
	else if max1 < max2{
		hand2 
	}
	else{
		let min1 = get_min_rank(hand1) ; 
		let min2 = get_min_rank(hand2) ;
		if min1 > min2{
			hand1
		}
		else if min1 < min2{
			hand2 
		}
		else{
			let total1 = get_hand_total(hand1) ; 
			let total2 = get_hand_total(hand2) ; 
			if total1 > total2{
				hand1
			}
			else{
				hand2 
			}
		}
	}


}


fn get_rank_three(hand:[(i32,char);5]) -> i32{
	//get the rank of the triple in a three of a kind 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;
	let mut ret:i32 = 0 ; 

	for rank in ranks.iter(){ 
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 3{
			ret = *rank ; 
			break ; 
		}
	}
	
	ret 
}


fn tie_break_three(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	let rank1 = get_rank_three(hand1) ; 
	let rank2 = get_rank_three(hand2) ;

	if rank1 > rank2{
		hand1 
	}
	else if rank1 < rank2{
		hand2 
	}
	else{
		let max1 = get_max_rank(hand1) ; 
		let max2 = get_max_rank(hand2) ; 
		if max1 > max2{
			hand1 
		}
		else if max1 < max2{ 
			hand2 
		}
		else{
			let total1 = get_hand_total(hand1) ; 
			let total2 = get_hand_total(hand2) ; 
			if total1 > total2{
				hand1
			}
			else{
				hand2 
			}
		}
	}
}


fn get_two_pair(hand:[(i32,char);5]) -> [i32;2]{
	//return the two pairs that exists in a two pair 
	let mut ret:[i32;2] = [0,0] ;
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;

	for rank in ranks.iter(){ 
		if ((hand.iter().filter(|&x| &x.0 == rank).count()) == 2) && (ret[0] == 0){
			ret[0] = *rank ; 
			continue ; 
		}
		if ((hand.iter().filter(|&x| &x.0 == rank).count()) == 2) && (ret[0] != 0){
			ret[1] = *rank ;
			break ; 
		}
	}

	sort(ret)

}


fn get_other_two_pair(hand:[(i32,char);5]) -> i32{
	//get the last card in a two pair 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;
	let mut num:i32 = 0 ;

	for rank in ranks.iter(){ 
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 1{
			num = *rank ; 
		}
	}

	num
}


fn tie_break_two_pair(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	//tie break two two pairs 
	let max1 = get_two_pair(hand1)[1] ;
	let max2 = get_two_pair(hand2)[1] ;

	if max1 > max2{
		hand1
	}
	else if max1 < max2{ 
		hand2
	}
	else{
		let temp1 = get_two_pair(hand1)[0] ; 
		let temp2 = get_two_pair(hand2)[0] ; 
		if temp1 > temp2{
			hand1 
		}
		else if temp1 < temp2{
			hand2 
		}
		else{
			let other1 = get_other_two_pair(hand1);
			let other2 = get_other_two_pair(hand2);
			if other1 > other2{
				hand1
			}
			else{
				hand2
			}
		}
	}
}


fn get_pair(hand:[(i32,char);5]) -> i32{
	//get the rank of the pair in a pair 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;
	let mut num:i32 = 0 ;

	for rank in ranks.iter(){ 
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 2{
			num = *rank ; 
		}
	}

	num
}


fn get_others_pair(hand:[(i32,char);5]) -> [i32;3]{
	//get the other cards in a pair 
	let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13] ;
	let mut ret:[i32;3] = [0,0,0] ; 
	let mut index:usize = 0 ; 
	
	for rank in ranks.iter(){
		if (hand.iter().filter(|&x| &x.0 == rank).count()) == 1{
			ret[index] = *rank ; 
			index = index + 1 ;
		}
	}
	sort(ret)
}


fn tie_break_pair(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	//tie break two hands that are pairs 
	let p1:i32 = get_pair(hand1) ; 
	let p2:i32 = get_pair(hand2) ;

	if p1 > p2{
		hand1
	}
	else if p1 < p2{
		hand2 
	}
	else{
		let l1:[i32;3] = get_others_pair(hand1) ;
		let l2:[i32;3] = get_others_pair(hand2) ; 

		for i in 0..3{
			if l1[i] > l2[i]{
				hand1 
			} 
			else if l1[i] < l2[i]{
				hand2 
			}
			else{
				break ;
			}
			;
		}

		let t1:i32 = get_hand_total(hand1) ; 
		let t2:i32 = get_hand_total(hand2) ;

		if t1 > t2 {
			hand1
		} else{
			hand2 
		}
		
	}

}


fn tie_break_high_card(hand1:[(i32,char);5], hand2:[(i32,char);5]) -> [(i32,char);5]{
	let max1:i32 = get_max_rank(hand1) ; 
	let max2:i32 = get_max_rank(hand2) ; 

	if max1 > max2{
		hand1
	}
	else if max1 < max2{
		hand2
	}
	else{
		let l1 = sort(get_rank_list(hand1)) ; 
		let l2 = sort(get_rank_list(hand2)) ;

		for i in 0..3{
			if l1[i] > l2[i]{
				hand1 
			}
			else if l1[i] < l2[i]{
				hand2
			}
			else{
				break ; 
			};
		}

		let t1:i32 = get_hand_total(hand1) ;
		let t2:i32 = get_hand_total(hand2) ;

		if t1 > t2{
			hand1
		}
		else{
			hand2 
		}
	}
}


fn tie_break(hand1:[(i32,char);5], hand2:[(i32,char);5], n:i32) -> [(i32,char);5]{
	//main tie break function
	match n{
		10 => tie_break_five(hand1, hand2),
		9 => tie_break_five(hand1, hand2),
		8 => tie_break_four(hand1,hand2),
		7 => tie_break_five(hand1,hand2),
		6 => tie_break_five(hand1,hand2),
		5 => tie_break_five(hand1,hand2),
		4 => tie_break_three(hand1, hand2),
		3 => tie_break_two_pair(hand1,hand2),
		2 => tie_break_pair(hand1,hand2), 
		_ => tie_break_high_card(hand1, hand2)
	}
}

//Main fuction and a helper 

fn convert_list(list:[u32;10]) -> [usize;10]{
	//convert a [u32] list to [usize] list
	//as per project requirements
	[(list[0] as usize),(list[1] as usize),(list[2] as usize),(list[3] as usize),(list[4] as usize),
	(list[5] as usize),(list[6] as usize),(list[7] as usize),(list[8] as usize),(list[9] as usize)]
}


fn deal(list:[u32;10]) -> [String;5]{
	
	let deck = [(0, 'Z'),
	(1, 'C'), (2, 'C'), (3, 'C'), (4, 'C'), (5, 'C'), (6, 'C'), (7, 'C'), (8, 'C'), (9, 'C'), (10, 'C'), (11, 'C'), (12, 'C'), (13, 'C'),  
	(1, 'D'), (2, 'D'), (3, 'D'), (4, 'D'), (5, 'D'), (6, 'D'), (7, 'D'), (8, 'D'), (9, 'D'), (10, 'D'), (11, 'D'), (12, 'D'), (13, 'D'),
	(1, 'H'), (2, 'H'), (3, 'H'), (4, 'H'), (5, 'H'), (6, 'H'), (7, 'H'), (8, 'H'), (9, 'H'), (10, 'H'), (11, 'H'), (12, 'H'), (13, 'H'),
	(1, 'S'), (2, 'S'), (3, 'S'), (4, 'S'), (5, 'S'), (6, 'S'), (7, 'S'), (8, 'S'), (9, 'S'), (10, 'S'), (11, 'S'), (12, 'S'), (13, 'S')
	];

	let hand1 = get_hand_one(convert_list(list), deck) ;
	let hand2 = get_hand_two(convert_list(list), deck) ;
	let score1 = get_score(hand1) ; 
	let score2 = get_score(hand2) ; 

	if score1 > score2{
		format_hand(hand1)
	}
	else if score1 < score2{
		format_hand(hand2)
	}
	else{
		let winner = tie_break(hand1, hand2, score1) ;
		format_hand(winner)
	}
}