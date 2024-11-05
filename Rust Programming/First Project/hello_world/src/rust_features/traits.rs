// Traits: A trait is a way to define a set of methods that a type must implement. We can use traits to define shared behavior in an abstract way.
// A trait defines the functionality a particular type has and can share with other types.
// Traits are similar to interfaces in other languages.
// In Rust, we can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

// In this example, we define a trait called Summary with a method called summarize. We then implement the Summary trait for the NewsArticle and Tweet types. Finally, we call the summarize method on instances of NewsArticle and Tweet.
pub trait Summary {
    fn summarize(&self) -> String {
        // Default implementation
        String::from("(Summary Trait Default Implementation)")
    }
}

pub trait NewsSummary {
    fn summarize_news(&self) -> String; // No default implementation
}

struct NewsArticle {
    headline: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // Overriding default implementation
        format!("{} ({})", self.headline, self.location)
    }
}

impl NewsSummary for NewsArticle {
    fn summarize_news(&self) -> String {
        // implementing NewsSummary trait
        format!("{} ({})", self.headline, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // Overriding default implementation
        format!("{}: {}", self.username, self.content)
    }
}

struct WeatherAlert {
    headline: String,
    location: String,
}

impl Summary for WeatherAlert {
    fn summarize(&self) -> String { // Overriding default implementation
        format!("{}: {}", self.headline, self.location)
    }
}

impl NewsSummary for WeatherAlert {
    fn summarize_news(&self) -> String { // implementing NewsSummary trait
        format!("{}: {}", self.headline, self.location)
    }
}

// Function that takes a type that implements the Summary trait or traits as a parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Traits Bound Syntax: The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound;
// Function that takes a type that implements both the Summary and NewsSummary traits
// Specifying Multiple Trait Bounds with the + Syntax
fn notify_2<T: Summary + NewsSummary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn traits_demo() {
    let article = NewsArticle {
        headline: String::from("Breaking news! Heavy Rain"),
        location: String::from("Bangalore"),
    };

    println!("News article summary: {}", article.summarize());

    let article_2 = NewsArticle {
        headline: String::from("Heavy Rain Expected!!"),
        location: String::from("Bangalore"),
    };

    println!("News article summary: {}", article_2.summarize_news());

    let article_3 = NewsArticle {
        headline: String::from("Yellow alert!! Heavy Rain Expected!!"),
        location: String::from("Bangalore"),
    };

    let article_4 = NewsArticle {
        headline: String::from("Yellow alert!!"),
        location: String::from("Bangalore"),
    };

    println!("\n---------- Notify ----------");
    notify(&article_3);

    println!("\n---------- Notify 2 ----------");
    notify_2(&article_4);

    let tweet = Tweet {
        username: String::from("Honnur Ali"),
        content: String::from("Heavy rain expected in Bangalore"),
    };

    
    notify(&tweet); // Will call the summarize method of the Tweet type

    // Traits Bound Syntax
    println!("\n---------- Traits Bound Syntax ----------");
    let weather_alert = WeatherAlert {
        headline: String::from("Yellow alert!! Heavy Rain Expected!!"),
        location: String::from("Bangalore"),
    };
    // notify_2(&tweet); // Will give an error because the Tweet type does not implement the NewsSummary trait must bound to Summary and NewsSummary
    notify_2(&weather_alert); // bound to Summary and NewsSummary trait
}
