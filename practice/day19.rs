/* Generic */

#[derive(Debug)]
struct DeviceIdentifier{}

fn identifier<T>(value:T) -> T {
    value
}

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
} 

#[derive(Debug)]
struct ChatMessage <T>{
    content:T,
    time:String
}

impl ChatMessage <DigitalContent> {
    fn consum_enetertainment(&self) {
        match self.content {
            DigitalContent::AudioFile => println!("Listened Songs {:?}", self.content),
            DigitalContent::VideoFile => println!("Watched Movies {:?}", self.content),
        }
    } 
}

impl<T> ChatMessage<T> {
    fn retrieve_time(self) -> String {
        self.time.clone()
    }
}

fn main()
{
    let id = DeviceIdentifier{};

    println!("{}", identifier(5));
    println!("{}", identifier(12.5));
    println!("{}", identifier(true));
    println!("{}", identifier("Hello"));
    println!("{:?}", identifier(id));

    let interstaller = DigitalContent::VideoFile;
    let send_movie = ChatMessage {
        content:interstaller,
        time:String::from("Sunday, 8.15 PM")
    };
    
    send_movie.consum_enetertainment();
    
    let notification = ChatMessage {
        content:DigitalContent::AudioFile,
        time:String::from("Saturday, 9.15 PM")
    };
    
    notification.consum_enetertainment();
    let watch_time = notification.retrieve_time();
    println!("{}", watch_time);
    
    let big_bang = ChatMessage {
        content:String::from("big bang"),
        time:String::from("Sunday, 8.15 PM")
    };

    /* consum_enetertainment is implemented for DigitalContent */
    //big_bang.consum_enetertainment();
    
    let watch_time = big_bang.retrieve_time();

    println!("{}", watch_time);

}

