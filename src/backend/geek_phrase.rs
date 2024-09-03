use getrandom::getrandom;
pub fn get_geek_phrase() -> &'static str {
    let phrases = vec![
        "Rust by the way.",
        "Vim by the way.",
        "All your Emails are belong to us.",
        "These are not the Emails you are looking for.",
        "Luke, I am your Email Client.",
        "It puts the Emails in the basket.",
        "May the Force be with your inbox",
        "Live long and process Emails.",
        "Resistance is futile, your Emails will be processed.",
        "To infinity and beyond with your Email attachments.",
        "I am the one who parses Emails.",
        "The Email Client has entered the Matrix.",
        "In the beginning, there were Emails.",
        "The quick brown fox jumps over the Email server.",
        "I am Groot... for managing Emails.",
        "You've got mail, and it's encrypted.",
        "Just one more Email to rule them all.",
        "In a world of Emails, be a unicorn.",
        "Keep calm and handle your Emails.",
        "The Email Client of the future is here.",
        "Hello World, meet my Email client.",
        "To Email or not to Email, that is the question.",
        "This is the way of Email delivery.",
        "I see Emails in your future.",
        "There's no place like Email.",
        "Email in a bottle.",
        "It's alive! (The Email Client, that is).",
        "One Email to find them all.",
        "You shall not pass... without reading your Emails.",
        "Welcome to the Email Jungle.",
        "Hasta la vista, Email.",
        "Beam me up, Scotty, I've got an Email.",
        "E=mc² (Email squared).",
        "The Email Client will be with you, always.",
        "All the Emails you can handle and more.",
        "You can't handle the Emails!",
        "Winter is coming... for your Emails.",
        "I drink and I know things... about Email management.",
        "You win or you get spammed.",
        "The night is dark and full of Emails.",
        "Chaos isn’t a pit. Chaos is a ladder of unread Emails.",
        "When you play the game of Emails, you win or you get spam.",
        "Valar Emailis: All men must check their inbox.",
        "A Lannister always replies his Emails.",
        "Dracarys to your Email filter rules.",
        "Hodor! Hold the Emails!.",
        "The Wall isn’t the only thing keeping your Emails out.",
        "I am the sword in the darkness... of your Email inbox.",
        "In the land of Emails, the inbox is king.",
        "You know nothing about handling Emails.",
        "Not all those who wander are lost... some are just lost in their Emails.",
        "The Iron Throne isn’t as hot as an overflowing inbox.",
        "We are the people who will send you Emails.",
        "No one ever truly deletes an Email.",
        "By the Old Gods and the New, manage your Emails wisely.",
        "The only thing we have to fear is fear itself... and spam Emails.",
        "Bend the knee to your Email Client.",
        "Here in the realm of Emails, the best defense is a good filter.",
        "In the game of Emails, the inbox is the battlefield.",
        "The Hand of the King is also the Hand of Email Filtering.",
        "The Night’s Watch of your Email is always vigilant.",
        "Let the Emails fall where they may.",
        "The Seven Kingdoms of your Email Accounts.",
        "He who controls the Emails controls the realm.",
        "A dragon on your side is better than an Email overload.",
        "When the winter comes, so do the unread Emails.",
        "The long night of Emails is coming.",
        "You’ve got mail! It’s like owl post, but with fewer owls.",
        "I solemnly swear I am up to no good... with my emails.",
        "Accio Emails! Where are my unread messages?",
        "The Sorting Hat says: Your house is Email.",
        "A wand chooses the wizard... and so does the spam filter.",
        "Mischief Managed! Your inbox is now clean.",
        "It’s levio-sa, not levio-sa! And it’s Email, not mail.",
        "In a world of Emails, be a Hogwarts student.",
        "Expecto Patronum to ward off those phishing Emails.",
        "Gryffindor values bravery... in handling Email overload.",
        "Slytherin ambition: mastering the art of Email sorting.",
        "Ravenclaw wisdom: know when to archive and when to delete Emails.",
        "Hufflepuff loyalty: never miss an important Email.",
        "You can’t stop the owls, but you can filter your Emails.",
        "My email is like a portkey... it takes me to new messages.",
        "Hogwarts Express to your inbox!.",
        "The Chamber of Secrets is open... and it’s full of unread Emails.",
        "Weasley’s Wizard Wheezes: for when your Emails are just too boring.",
        "Dumbledore’s Army... for organizing your Email.",
        "The Pensieve of Email: Remember every important message.",
        "Slytherins don’t just get sorted, they sort their Emails too.",
        "A magical world of Emails awaits.",
        "Gringotts for your Email accounts.",
        "Beware the Unforgivable Curses... and the Unread Emails.",
        "Sorting Emails is like sorting students into houses.",
        "Time-Turner: Because sometimes you need more time to read Emails.",
        "Wingardium Leviosa: Make your Emails float to the top.",
        "I’m not just a wizard, I’m a wizard at Email management.",
        "It’s not just magic, it’s Email magic.",
        "The Marauder’s Map: Finding your way through your inbox",
    ];
    phrases[get_random_number_in_range(0..phrases.len() as u32) as usize]
}

fn get_random_number_in_range(range: std::ops::Range<u32>) -> u32 {
    let mut random_index = [0u8; 1];

    let random_u8_number = match getrandom(&mut random_index) {
        Ok(_) => random_index[0],
        Err(_) => 0,
    };
    (random_u8_number as u32 * (range.end - range.start)) / 256 + range.start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_are_random() {
        let mut random_numbers = vec![];
        for _ in 0..100 {
            random_numbers.push(get_random_number_in_range(0..10));
        }
        let mut unique_numbers = random_numbers.clone();
        unique_numbers.sort();
        unique_numbers.dedup();
        assert!(unique_numbers.len() > 1);
    }

    #[test]
    fn test_within_provided_range() {
        let max = 50;
        let min = 10;
        for _ in 0..100 {
            let random_number = get_random_number_in_range(min..max);
            assert!(random_number >= min && random_number < max);
        }
    }
}
