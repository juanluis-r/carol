fn main() {
    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves and",
        "A partridge in a pear tree"
        ];
    let ordinals = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth",
        "seventh", 
        "eighth", 
        "ninth", 
        "tenth", 
        "eleventh", 
        "twelfth"];

    for day in 1..=12 {
        println!("\nVerse [{}]", day);
        println!("On the {} day of Christmas my true love sent to me", ordinals[day-1]);

        for daily_gift in (1..=day).rev() {
            println!("{}", gifts[12-daily_gift]);
        }
    }
}
