/* Solutions */

fn main()
{
    let mut trip: String = start_trip();
    
    visit_milan(&mut trip);
    visit_vinci(&mut trip);
    visit_itali(&mut trip);

    show_itinerary(&trip);
}

fn show_itinerary(trip:&String)
{
    println!("{}", trip);
}

fn visit_milan(trip:&mut String) {
    trip.push_str("to go to milan, ");
}

fn visit_vinci(trip:&mut String) {
    trip.push_str("vinci, ");
}

fn visit_itali(trip:&mut String) {
    trip.push_str("itali ");
}

fn start_trip() -> String {
    String::from("Plan is ")
}
